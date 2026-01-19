use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use tauri::{AppHandle, Manager};
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use crate::{
    game::gamemanager::GameManager,
    network::{
        channels::{init_channels, HostChannelSenders},
        clientid::ClientId,
        clientnetworkhandler::ClientNetworkHandler,
        clientrequest::{ClientMessage, ClientRequest},
        networkclient::NetworkClient,
        networkhandler::NetworkHandler,
        serverevent::ServerEvent,
        socketmanager::{SocketData, SocketManager},
    },
};

pub struct StartupManager {
    tasks: Vec<JoinHandle<()>>,
    gm: Arc<Mutex<GameManager>>,
    app: AppHandle,
    shutdown_tx: Option<tokio::sync::watch::Sender<bool>>,
}

#[derive(Clone)]
pub struct ManagedSenders {
    pub inner: Arc<Mutex<HostChannelSenders>>,
}

impl StartupManager {
    pub fn new(gm: Arc<Mutex<GameManager>>, app: AppHandle) -> Self {
        // On app startup
        let (senders, _receivers) = init_channels();
        let managed_senders = ManagedSenders {
            inner: Arc::new(Mutex::new(senders)),
        };
        app.manage(managed_senders.clone());

        Self {
            tasks: Vec::new(),
            gm,
            app,
            shutdown_tx: None,
        }
    }

    pub async fn init_host(&mut self, port: u16) {
        self.close_tasks().await;

        let (senders, receivers) = init_channels();

        self.init_gm(&senders.snapshot_tx, receivers.game_rx);

        self.init_client(
            &senders,
            receivers.client_event_rx,
            receivers.frontend_request_rx,
        );

        self.init_network(
            &senders,
            receivers.snapshot_rx,
            receivers.client_message_rx,
            receivers.incoming_socket_data_rx,
        );

        self.init_socket(
            true,
            port,
            &senders,
            receivers.outgoing_socket_data_rx,
            receivers.shutdown_rx,
        );

        self.shutdown_tx = Some(senders.shutdown_tx.clone());
        // Update the existing managed state
        let managed_senders = self.app.state::<ManagedSenders>();
        *managed_senders.inner.lock().unwrap() = senders;
    }

    pub async fn init_join(&mut self, port: u16) {
        self.close_tasks().await;
        let (senders, receivers) = init_channels();

        self.init_client(
            &senders,
            receivers.client_event_rx,
            receivers.frontend_request_rx,
        );

        self.init_socket(
            false,
            port,
            &senders,
            receivers.outgoing_socket_data_rx,
            receivers.shutdown_rx,
        );

        self.init_client_network(
            &senders,
            receivers.client_message_rx,
            receivers.incoming_socket_data_rx,
        );
    }

    pub async fn close_tasks(&mut self) {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            // shutdown_tx is now owned, self.shutdown_tx is None
            let _ = shutdown_tx.send(true);
            println!("Shutdown Called.");
        }

        tokio::time::sleep(Duration::from_millis(10)).await;

        //abort open tasks and clear list
        for handle in self.tasks.drain(..) {
            handle.abort();
        }

        println!("All listeners aborted.");
        self.shutdown_tx = None;
    }

    //Helpers:

    fn init_gm(
        &self,
        snapshot_tx: &UnboundedSender<ServerEvent>,
        game_rx: UnboundedReceiver<(ClientRequest, ClientId)>,
    ) {
        let mut gm = self.gm.lock().unwrap(); // okay to panic on poisoned mutex
        gm.setup_game_manager(Some(snapshot_tx.clone()), Some(game_rx));
    }

    pub fn init_client(
        &mut self,
        senders: &HostChannelSenders,
        client_event_rx: tokio::sync::mpsc::UnboundedReceiver<ServerEvent>,
        frontend_request_rx: tokio::sync::mpsc::UnboundedReceiver<ClientRequest>,
    ) {
        let mut client = NetworkClient::new(
            self.app.clone(),
            senders.client_message_tx.clone(), // clone sender
            client_event_rx,                   // move receiver
            frontend_request_rx,               // move receiver
        );

        client.init_id(true, None);

        let client_handle: JoinHandle<()> = tokio::spawn(async move {
            client.start_listening().await;
        });

        self.tasks.push(client_handle);
    }

    fn init_network(
        &mut self,
        senders: &HostChannelSenders,
        snapshot_rx: UnboundedReceiver<ServerEvent>,
        client_message_rx: UnboundedReceiver<ClientMessage>,
        incoming_socket_data: UnboundedReceiver<SocketData>,
    ) {
        // Create the network manager
        let mut nm = NetworkHandler::new(
            senders.game_tx.clone(), // clone sender
            client_message_rx,       // move receiver
            snapshot_rx,
            senders.client_event_tx.clone(),
            incoming_socket_data,
            senders.outgoing_socket_data_tx.clone(),
        );

        //Network Listening
        let nm_handle: JoinHandle<()> = tokio::spawn(async move {
            nm.start_listening().await;
        });

        self.tasks.push(nm_handle);
    }

    fn init_client_network(
        &mut self,
        senders: &HostChannelSenders,
        incoming_client_request: UnboundedReceiver<ClientMessage>,
        incoming_socket_data: UnboundedReceiver<SocketData>,
    ) {
        let mut cnh = ClientNetworkHandler::new(
            incoming_client_request,
            senders.client_event_tx.clone(),
            incoming_socket_data,
            senders.outgoing_socket_data_tx.clone(),
        );

        //Client Network Listening
        let nm_handle: JoinHandle<()> = tokio::spawn(async move {
            cnh.start_listening().await;
        });

        self.tasks.push(nm_handle);
    }

    //Todo clean this up with an enum so one function can do this easily
    fn init_socket(
        &mut self,
        is_host: bool,
        port: u16,
        senders: &HostChannelSenders,
        outgoing_socket_data_rx: UnboundedReceiver<SocketData>,
        shutdown_rx: tokio::sync::watch::Receiver<bool>,
    ) {
        // Get incoming clone
        let incoming_tx = senders.incoming_socket_data_tx.clone();
        let sm_handle: JoinHandle<()>;
        if is_host {
            sm_handle = tokio::spawn(async move {
                //Host Socket
                let mut sm = match SocketManager::host(port).await {
                    Ok(sm) => sm,
                    Err(e) => {
                        eprintln!("Failed to host socket: {e}");
                        return;
                    }
                };

                //Start Polling
                sm.run(incoming_tx.clone(), outgoing_socket_data_rx, shutdown_rx)
                    .await;
            });
        } else {
            sm_handle = tokio::spawn(async move {
                let mut sm = match SocketManager::join(port).await {
                    Ok(sm) => sm,
                    Err(e) => {
                        eprintln!("Failed to host socket: {e}");
                        return;
                    }
                };

                sm.run(incoming_tx.clone(), outgoing_socket_data_rx, shutdown_rx)
                    .await;
            });
        }

        self.tasks.push(sm_handle);
    }
}
