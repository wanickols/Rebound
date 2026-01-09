use tauri::Emitter;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::network::{clientrequest::ClientRequest, serverevent::ServerEvent};

///This is a middle man class to talk between the frontend and the network for the client.
pub struct NetworkClient {
    pub client_request_tx: UnboundedSender<ClientRequest>,
    pub frontend_requend_rx: UnboundedReceiver<ClientRequest>,
    pub server_event_rx: UnboundedReceiver<ServerEvent>,
    app: tauri::AppHandle,
}

impl NetworkClient {
    pub fn new(
        app: tauri::AppHandle,
        client_request_tx: UnboundedSender<ClientRequest>,
        server_event_rx: UnboundedReceiver<ServerEvent>,
        frontend_requend_rx: UnboundedReceiver<ClientRequest>,
    ) -> Self {
        Self {
            client_request_tx,
            frontend_requend_rx,
            server_event_rx,
            app,
        }
    }

    //To Network
    pub async fn send_request(&self, req: ClientRequest) {
        let _ = self.client_request_tx.send(req);
    }

    //Listen for network and frontend
    pub async fn start_listening(&mut self) {
        loop {
            tokio::select! {
                Some(req) = self.frontend_requend_rx.recv() => self.send_request(req).await,
                Some(evt) = self.server_event_rx.recv() => self.handle_server_event(evt).await,
                else => break, // both channels closed, shutdown
            }
        }
    }

    //Emit To Frontend
    async fn handle_server_event(&self, event: ServerEvent) {
        match event {
            ServerEvent::WorldSnapshot { snapshot } => {
                if let Err(err) = self.app.emit("game-state", snapshot.clone()) {
                    eprintln!("Failed to emit game-state: {}", err);
                }
            }
            ServerEvent::AddedPlayer { entity } => {
                if let Err(err) = self.app.emit("added_player", entity.0) {
                    eprintln!("Failed to add a player to client: {}", err);
                }
            }
        }
    }
}
