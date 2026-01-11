use tauri::Emitter;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::network::{
    clientid::{self, ClientId},
    clientrequest::{ClientMessage, ClientRequest},
    serverevent::ServerEvent,
};

///This is a middle man class to talk between the frontend and the network for the client.
pub struct NetworkClient {
    pub client_request_tx: UnboundedSender<ClientMessage>,
    pub frontend_requend_rx: UnboundedReceiver<ClientRequest>,
    pub server_event_rx: UnboundedReceiver<ServerEvent>,
    app: tauri::AppHandle,
    id: Option<ClientId>,
}

impl NetworkClient {
    pub fn new(
        app: tauri::AppHandle,
        client_request_tx: UnboundedSender<ClientMessage>,
        server_event_rx: UnboundedReceiver<ServerEvent>,
        frontend_requend_rx: UnboundedReceiver<ClientRequest>,
    ) -> Self {
        Self {
            client_request_tx,
            frontend_requend_rx,
            server_event_rx,
            app,
            id: None,
        }
    }

    //init id
    pub fn init_id(&mut self, is_host: bool, id: Option<ClientId>) {
        if is_host {
            self.id = Some(ClientId::new());
        } else {
            self.id = id
        }
    }

    //To Network

    pub async fn send_request(&self, req: ClientRequest) {
        // Make sure the client has an ID
        let client_id = match self.id {
            Some(id) => id,
            None => {
                eprintln!("Client ID not set, cannot send request");
                return;
            }
        };

        // Wrap the request in a ClientMessage
        let msg = ClientMessage {
            client_id,
            request: req,
        };

        // Send it to NetworkManager
        let _ = self.client_request_tx.send(msg);
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
            ServerEvent::AddedPlayer { entity, client } => {
                if client.0 != self.id.unwrap().0 {
                    return;
                }
                println!("Added Player");
                if let Err(err) = self.app.emit("added_player", entity.0) {
                    eprintln!("Failed to add a player to client: {}", err);
                }
            }
        }
    }
}
