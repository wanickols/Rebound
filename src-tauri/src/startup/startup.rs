use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::unbounded_channel;

use crate::{
    game::gamemanager::GameManager,
    network::{
        clientrequest::ClientRequest, networkclient::NetworkClient, networkhandler::NetworkHandler,
        networkmanager::NetworkManager, serverevent::ServerEvent, socketmanager::SocketManager,
    },
};

pub struct StartupManager {
    nm: Option<NetworkHandler>,
    gm: Option<Arc<Mutex<GameManager>>>,
    sm: Option<SocketManager>,
    client: Option<Arc<Mutex<NetworkClient>>>,
}

impl StartupManager {
    pub fn init_host(&mut self) {
        //gm to nm
        let (snapshot_tx, snapshot_rx) = unbounded_channel::<ServerEvent>();

        //frontend/socket to nm
        let (client_request_tx, client_request_rx) = unbounded_channel::<ClientRequest>();

        //nm to gm
        let (game_tx, mut game_rx) = unbounded_channel::<ClientRequest>();

        //gm get's sender for snapshot and receiver for incoming client requests
        let gm_arc = self
            .gm
            .as_ref()
            .expect("GameManager must exist to initialize channels")
            .clone();

        let mut gm = gm_arc.lock().unwrap(); // okay to unwrap here if panic is fine on poisoned mutex
        gm.init_channels(Some(snapshot_tx), Some(game_rx));

        let client_arc = self
            .client
            .as_ref()
            .expect("NetworkClient must exist to initialize sender")
            .clone();

        let mut client = client_arc.lock().unwrap();
        client.init_sender(client_request_tx.clone());

        self.nm = Some(NetworkHandler::new(game_tx, client_request_rx, snapshot_rx));

        self.sm = Some(SocketManager::new(client_request_tx));

        if let Some(mut sm) = self.sm.take() {
            tokio::spawn(async move {
                if let Err(e) = sm.host(8080).await {
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
