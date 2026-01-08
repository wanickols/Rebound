use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::network::{clientrequest::ClientRequest, serverevent::ServerEvent};

pub struct NetworkHandler {
    game_tx: UnboundedSender<ClientRequest>,
    client_requests: UnboundedReceiver<ClientRequest>,
    server_events: UnboundedReceiver<ServerEvent>,
}

impl NetworkHandler {
    pub fn new(
        game_tx: UnboundedSender<ClientRequest>,
        client_requests: UnboundedReceiver<ClientRequest>,
        server_events: UnboundedReceiver<ServerEvent>,
    ) -> Self {
        Self {
            game_tx,
            client_requests,
            server_events,
        }
    }

    pub async fn start_listening(&mut self) {
        loop {
            tokio::select! {
                Some(req) = self.client_requests.recv() => self.handle_client_request(req).await,
                Some(evt) = self.server_events.recv() => self.handle_server_event(evt).await,
                else => break, // both channels closed, shutdown
            }
        }
    }

    async fn handle_client_request(&self, request: ClientRequest) {
        // some point check for missing entity id's or obvious fails here.
        let _ = self.game_tx.send(request);
    }

    async fn handle_server_event(&self, event: ServerEvent) {
        // Send event to all clients
        // Send event to host client
    }
}
