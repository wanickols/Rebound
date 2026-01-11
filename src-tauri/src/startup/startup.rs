use std::sync::{mpsc::Receiver, Arc, Mutex};

use tauri::{App, AppHandle, Manager};
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use crate::{
    game::gamemanager::GameManager,
    network::{
        channels::{init_channels, HostChannelReceivers, HostChannelSenders},
        clientid::ClientId,
        clientrequest::{ClientMessage, ClientRequest},
        networkclient::NetworkClient,
        networkhandler::NetworkHandler,
        serverevent::ServerEvent,
        socketmanager::{SocketData, SocketManager},
    },
};

pub struct StartupManager {
    nh_listener: Option<JoinHandle<()>>,
    gm: Arc<Mutex<GameManager>>,
    sm_listener: Option<JoinHandle<()>>,
    client_listener: Option<JoinHandle<()>>,
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
            nh_listener: None,
            gm,
            sm_listener: None,
            client_listener: None,
            app,
            shutdown_tx: None,
        }
    }

    pub fn init_host(&mut self, port: u16) {
        self.close_listeners();

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
            receivers.socket_data_rx,
        );

        self.init_socket(port, &senders, receivers.shutdown_rx);

        self.shutdown_tx = Some(senders.shutdown_tx.clone());
        // Update the existing managed state
        let managed_senders = self.app.state::<ManagedSenders>();
        *managed_senders.inner.lock().unwrap() = senders;
    }

    pub fn init_join(&mut self, _ip: String, _port: u16) {
        self.close_listeners();
        // let mut nm = NetworkManager::new();
        // nm.init_socket(Role::Client { ip, port })
        //     .expect("Failed to init client socket");
        // self.nm = Some(nm);
    }

    pub fn close_listeners(&mut self) {
        // Abort client listener
        if let Some(handle) = self.client_listener.take() {
            handle.abort();
        }

        // Abort network manager listener
        if let Some(handle) = self.nh_listener.take() {
            handle.abort();
        }

        // Abort socket manager listener
        if let Some(handle) = self.sm_listener.take() {
            handle.abort();
        }
        println!("All listeners aborted.");
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            // shutdown_tx is now owned, self.shutdown_tx is None
            let _ = shutdown_tx.send(true); // ignore error if receivers dropped
        }

        println!("Shutdown Called.");
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

        self.client_listener = Some(client_handle);
    }

    fn init_network(
        &mut self,
        senders: &HostChannelSenders,
        snapshot_rx: UnboundedReceiver<ServerEvent>,
        client_message_rx: UnboundedReceiver<ClientMessage>,
        socket_data: UnboundedReceiver<SocketData>,
    ) {
        // Create the network manager
        let mut nm = NetworkHandler::new(
            senders.game_tx.clone(), // clone sender
            client_message_rx,       // move receiver
            snapshot_rx,
            senders.client_event_tx.clone(),
            socket_data,
        );

        println!("network handler created");

        // Spawn its listener
        let nm_handle: JoinHandle<()> = tokio::spawn(async move {
            nm.start_listening().await;
        });

        // Keep the handle in self
        self.nh_listener = Some(nm_handle);
    }

    fn init_socket(
        &mut self,
        port: u16,
        senders: &HostChannelSenders,
        shutdown_rx: tokio::sync::watch::Receiver<bool>,
    ) {
        let mut sm = SocketManager::new(senders.socket_data_tx.clone());

        // Spawn the hosting task
        let sm_handle: JoinHandle<()> = tokio::spawn(async move {
            if let Err(e) = sm.host(port).await {
                eprintln!("Failed to host socket: {e}");
                return;
            }

            let _ = sm.poll_socket(shutdown_rx).await;
        });

        //todo add polling for socket manager

        // Keep the handle in self
        self.sm_listener = Some(sm_handle);
    }
}
