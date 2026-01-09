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
    pub fn send_request(&self, req: ClientRequest) {
        let _ = self.client_request_tx.send(req);
    }

    //Listen for network and frontend
    pub async fn start_listening(&mut self) {
        while let Some(evt) = self.server_event_rx.recv().await {
            self.handle_server_event(evt).await;
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
        }
    }
}
