use std::sync::{Arc, Mutex};

use tauri::{App, AppHandle, Manager};
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use crate::{
    game::gamemanager::GameManager,
    network::{
        channels::{init_channels, HostChannelReceivers, HostChannelSenders},
        clientrequest::ClientRequest,
        networkclient::NetworkClient,
        networkhandler::NetworkHandler,
        serverevent::ServerEvent,
        socketmanager::SocketManager,
    },
};

pub struct StartupManager {
    nm: Option<JoinHandle<()>>,
    gm: Arc<Mutex<GameManager>>,
    sm: Option<JoinHandle<()>>,
    client: Option<JoinHandle<()>>,
    app: AppHandle,
}

impl StartupManager {
    pub fn new(gm: Arc<Mutex<GameManager>>, app: AppHandle) -> Self {
        Self {
            nm: None,
            gm,
            sm: None,
            client: None,
            app,
        }
    }

    pub fn init_host(&mut self, port: u16) {
        //channel creation
        let (senders, receivers) = init_channels();
        self.app.manage(senders.frontend_request_tx.clone());

        self.init_gm(&senders.snapshot_tx, receivers.game_rx);

        self.init_client(
            &senders,
            receivers.client_event_rx,
            receivers.frontend_request_rx,
        );

        self.init_network(&senders, receivers.snapshot_rx, receivers.client_request_rx);

        self.init_socket(port, &senders);
    }

    pub fn init_join(&mut self, ip: String, port: u16) {
        // let mut nm = NetworkManager::new();
        // nm.init_socket(Role::Client { ip, port })
        //     .expect("Failed to init client socket");
        // self.nm = Some(nm);
    }

    pub fn leave(&mut self) {
        //exit code.... ;D
    }

    //Helpers:

    fn init_gm(
        &self,
        snapshot_tx: &UnboundedSender<ServerEvent>,
        game_rx: UnboundedReceiver<ClientRequest>,
    ) {
        let mut gm = self.gm.lock().unwrap(); // okay to panic on poisoned mutex
        gm.init_channels(Some(snapshot_tx.clone()), Some(game_rx));
    }

    pub fn init_client(
        &mut self,
        senders: &HostChannelSenders,
        client_event_rx: tokio::sync::mpsc::UnboundedReceiver<ServerEvent>,
        frontend_request_rx: tokio::sync::mpsc::UnboundedReceiver<ClientRequest>,
    ) {
        let mut client = NetworkClient::new(
            self.app.clone(),
            senders.client_request_tx.clone(), // clone sender
            client_event_rx,                   // move receiver
            frontend_request_rx,               // move receiver
        );

        let client_handle: JoinHandle<()> = tokio::spawn(async move {
            client.start_listening().await;
        });

        self.client = Some(client_handle);
    }

    fn init_network(
        &mut self,
        senders: &HostChannelSenders,
        snapshot_rx: tokio::sync::mpsc::UnboundedReceiver<ServerEvent>,
        client_request_rx: tokio::sync::mpsc::UnboundedReceiver<ClientRequest>,
    ) {
        // Create the network manager
        let mut nm = NetworkHandler::new(
            senders.game_tx.clone(),         // clone sender
            client_request_rx,               // move receiver
            snapshot_rx,                     // move receiver
            senders.client_event_tx.clone(), // clone sender
        );

        println!("network handler created");

        // Spawn its listener
        let nm_handle: JoinHandle<()> = tokio::spawn(async move {
            nm.start_listening().await;
        });

        // Keep the handle in self
        self.nm = Some(nm_handle);
    }

    fn init_socket(&mut self, port: u16, senders: &HostChannelSenders) {
        let mut sm = SocketManager::new(senders.client_request_tx.clone());

        // Spawn the hosting task
        let sm_handle: JoinHandle<()> = tokio::spawn(async move {
            if let Err(e) = sm.host(port).await {
                eprintln!("Failed to host socket: {e}");
            }
        });

        // Keep the handle in self
        self.sm = Some(sm_handle);
    }
}
