use std::{collections::HashMap, net::SocketAddr};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::network::{
    clientid::ClientId,
    clientrequest::{ClientMessage, ClientRequest},
    serverevent::ServerEvent,
};

pub struct NetworkHandler {
    client_map: HashMap<ClientId, SocketAddr>,
    game_tx: UnboundedSender<(ClientRequest, ClientId)>,
    client_message: UnboundedReceiver<ClientMessage>,
    server_events: UnboundedReceiver<ServerEvent>,
    client_events: UnboundedSender<ServerEvent>,
}

impl NetworkHandler {
    pub fn new(
        game_tx: UnboundedSender<(ClientRequest, ClientId)>,
        client_message: UnboundedReceiver<ClientMessage>,
        server_events: UnboundedReceiver<ServerEvent>,
        client_events: UnboundedSender<ServerEvent>,
    ) -> Self {
        Self {
            client_map: HashMap::new(),
            game_tx,
            client_message,
            server_events,
            client_events,
        }
    }

    pub async fn start_listening(&mut self) {
        loop {
            tokio::select! {
                Some(req) = self.client_message.recv() => self.handle_client_request(req).await,
                Some(evt) = self.server_events.recv() => self.handle_server_event(evt).await,
                else => break, // both channels closed, shutdown
            }
        }
    }

    async fn handle_socket_data(&mut self, data: (SocketAddr, Vec<u8>)) {
        let (peer_addr, bytes) = data;

        //deserialize
        let msg: ClientMessage = match serde_json::from_slice(&bytes) {
            Ok(m) => m,
            Err(e) => {
                eprintln!(
                    "Failed to deserialize client message from {}: {}",
                    peer_addr, e
                );
                return;
            }
        };

        //get id from map
        let client_id = self.get_or_assign_client_id(peer_addr);

        // send to game manager
        let request = msg.request;
        let _ = self.game_tx.send((request, client_id));
    }

    //just from client
    async fn handle_client_request(&self, message: ClientMessage) {
        let client_id = message.client_id;
        let request = message.request;

        // Forward to GameManager
        let _ = self.game_tx.send((request, client_id));
    }

    async fn handle_server_event(&self, event: ServerEvent) {
        let _ = self.client_events.send(event);
    }

    fn get_or_assign_client_id(&mut self, addr: SocketAddr) -> ClientId {
        //check in map
        if let Some(&id) = self
            .client_map
            .iter()
            .find_map(|(id, &a)| if a == addr { Some(id) } else { None })
        {
            return id;
        }

        // New client → assign atomic ID
        let new_id = ClientId::new();
        self.client_map.insert(new_id, addr);
        println!("New client {} assigned to {}", new_id.0, addr);

        new_id
    }
}
