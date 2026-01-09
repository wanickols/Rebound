use std::sync::{Arc, Mutex};

use tauri::{App, AppHandle, Manager};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use crate::{
    game::gamemanager::GameManager,
    network::{
        clientrequest::ClientRequest, networkclient::NetworkClient, networkhandler::NetworkHandler,
        serverevent::ServerEvent, socketmanager::SocketManager,
    },
};

pub struct StartupManager {
    nm: Option<tokio::task::JoinHandle<()>>,
    gm: Arc<Mutex<GameManager>>,
    sm: Option<SocketManager>,
    client: Option<tokio::task::JoinHandle<()>>,
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
        //gm to nm
        let (snapshot_tx, snapshot_rx) = unbounded_channel::<ServerEvent>();
        //nm to gm
        let (game_tx, mut game_rx) = unbounded_channel::<ClientRequest>();

        //client/socket to nm
        let (client_request_tx, client_request_rx) = unbounded_channel::<ClientRequest>();
        //nm to client/socket
        let (client_event_tx, client_event_rx) = unbounded_channel::<ServerEvent>();

        //frontend to client
        let (frontend_request_tx, frontend_request_rx) = unbounded_channel::<ClientRequest>();
        self.app.manage(frontend_request_tx);

        //gm get's sender for snapshot and receiver for incoming client requests
        let gm_arc = self.gm.as_ref().clone();

        let mut gm = gm_arc.lock().unwrap(); // okay to unwrap here if panic is fine on poisoned mutex
        gm.init_channels(Some(snapshot_tx), Some(game_rx));

        let mut client = NetworkClient::new(
            self.app.clone(),
            client_request_tx.clone(),
            client_event_rx,
            frontend_request_rx,
        );

        let client_handle = tokio::spawn(async move {
            client.start_listening().await;
        });

        self.client = Some(client_handle);

        // create the NM
        let mut nm = NetworkHandler::new(game_tx, client_request_rx, snapshot_rx, client_event_tx);

        println!("network handler created");
        let nm_handle = tokio::spawn(async move {
            nm.start_listening().await;
        });

        // StartupManager keeps the handle, not the NM
        self.nm = Some(nm_handle);

        self.sm = Some(SocketManager::new(client_request_tx));

        if let Some(mut sm) = self.sm.take() {
            tokio::spawn(async move {
                if let Err(e) = sm.host(port).await {
                    eprintln!("Failed to host socket: {e}");
                }
            });
        }
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
}
