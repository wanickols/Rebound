use std::{collections::HashMap, net::SocketAddr};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::network::{
    clientid::{self, ClientId},
    clientrequest::{ClientMessage, ClientRequest},
    serverevent::ServerEvent,
    socketmanager::SocketData,
};

pub struct NetworkHandler {
    clients_by_addr: HashMap<SocketAddr, ClientId>,
    clients_by_id: HashMap<ClientId, SocketAddr>,
    game_tx: UnboundedSender<(ClientRequest, ClientId)>,
    client_message: UnboundedReceiver<ClientMessage>,
    server_events: UnboundedReceiver<ServerEvent>,
    client_events: UnboundedSender<ServerEvent>,

    //TTL
    pub reset_client_tx: UnboundedSender<ClientId>,
    pub client_died_rx: UnboundedReceiver<ClientId>,

    //Socket
    incoming_socket_data: UnboundedReceiver<SocketData>,
    outgoing_socket_data: UnboundedSender<SocketData>,
}

impl NetworkHandler {
    pub fn new(
        game_tx: UnboundedSender<(ClientRequest, ClientId)>,
        client_message: UnboundedReceiver<ClientMessage>,
        server_events: UnboundedReceiver<ServerEvent>,
        client_events: UnboundedSender<ServerEvent>,
        reset_client_tx: UnboundedSender<ClientId>,
        client_died_rx: UnboundedReceiver<ClientId>,
        incoming_socket_data: UnboundedReceiver<SocketData>,
        outgoing_socket_data: UnboundedSender<SocketData>,
    ) -> Self {
        Self {
            clients_by_addr: HashMap::new(),
            clients_by_id: HashMap::new(),
            game_tx,
            client_message,
            server_events,
            client_events,
            client_died_rx,
            reset_client_tx,
            incoming_socket_data,
            outgoing_socket_data,
        }
    }

    pub async fn start_listening(&mut self) {
        loop {
            tokio::select! {
                Some(dta) = self.incoming_socket_data.recv() => self.handle_socket_data(dta).await,
                Some(req) = self.client_message.recv() => self.handle_client_request(req).await,
                Some(evt) = self.server_events.recv() => self.handle_server_event(evt).await,
                Some(clt) = self.client_died_rx.recv() => self.handle_client_died(clt).await,
                else => break, // all channels closed, shutdown
            }
        }
    }

    async fn handle_socket_data(&mut self, data: SocketData) {
        println!("Handling");
        let (peer_addr, bytes) = data;

        //deserialize
        let msg: ClientMessage = match serde_json::from_slice(&bytes) {
            Ok(m) => m,
            Err(e) => {
                eprintln!(
                    "Failed to deserialize client message from {}: {:?} {}",
                    peer_addr, bytes, e
                );
                if !self.clients_by_addr.contains_key(&peer_addr) {
                    self.send_blank_join(peer_addr);
                }
                return;
            }
        };
        let request = msg.request;
        match request {
            ClientRequest::Joined => {
                self.handle_client_join(peer_addr);
                return;
            }
            ClientRequest::Idle => {
                let clientid = self.clients_by_addr.get(&data.0);
                if clientid.is_none() {
                    return;
                }
                self.reset_client_tx.send(*clientid.unwrap());
                return;
            }
            _ => {}
        }

        if msg.client_id.is_none() {
            eprintln!("Recieved message from unknown client",);
            return;
        }

        // send to game manager

        let _ = self.game_tx.send((request, msg.client_id.unwrap()));
    }

    fn handle_client_join(&mut self, peer_addr: SocketAddr) {
        // Ignore duplicate joins
        if self.clients_by_addr.contains_key(&peer_addr) {
            return;
        }
        println!("Recieving join req");

        let client_id = ClientId::new();
        self.clients_by_addr.insert(peer_addr, client_id);
        self.clients_by_id.insert(client_id, peer_addr);

        let event = ServerEvent::Joined {
            client_id: Some(client_id),
        };

        let bytes = serde_json::to_vec(&event).unwrap();
        self.send_over_network(peer_addr, bytes);
    }

    async fn handle_client_died(&mut self, id: ClientId) {
        let addr = self.clients_by_id.get(&id).cloned();
        if addr.is_none() {
            println!("Tried to delete client that doesn't exist");
            return;
        }
        self.clients_by_id.remove(&id);
        self.clients_by_addr.remove(&addr.unwrap());
    }

    fn send_blank_join(&self, addr: SocketAddr) {
        let blank_join = ServerEvent::Joined { client_id: None };
        let bytes = match serde_json::to_vec(&blank_join) {
            Ok(b) => b,
            Err(_) => return,
        };
        self.send_over_network(addr, bytes);
        println!("Sending blank");
    }

    //just from client
    async fn handle_client_request(&self, message: ClientMessage) {
        let client_id = message.client_id;
        let request = message.request;

        if client_id.is_none() {
            return;
        }

        // Forward to GameManager
        let _ = self.game_tx.send((request, client_id.unwrap()));
    }

    async fn handle_server_event(&self, event: ServerEvent) {
        match &event {
            ServerEvent::Joined {
                client_id: Some(id),
            } => {
                self.compose_client_send(*id, event);
            }

            ServerEvent::AddedPlayer { client, .. } => {
                self.compose_client_send(*client, event);
            }

            ServerEvent::WorldSnapshot { .. } => {
                self.send_to_all(event);
            }

            _ => {}
        }
    }

    fn compose_client_send(&self, client_id: ClientId, event: ServerEvent) {
        if let Some(addr) = self.clients_by_id.get(&client_id) {
            let bytes = match serde_json::to_vec(&event) {
                Ok(b) => b,
                Err(_) => return,
            };
            self.send_over_network(*addr, bytes);
        } else {
            self.send_to_host_client(event); //host client isn't stored
        }
    }

    fn send_to_all(&self, event: ServerEvent) {
        let bytes = match serde_json::to_vec(&event) {
            Ok(b) => b,
            Err(_) => return,
        };

        for addr in self.clients_by_id.values() {
            self.send_over_network(*addr, bytes.clone());
        }
        self.send_to_host_client(event);
    }

    fn send_over_network(&self, addr: SocketAddr, bytes: Vec<u8>) {
        let _ = self.outgoing_socket_data.send((addr, bytes));
    }

    fn send_to_host_client(&self, event: ServerEvent) {
        let _ = self.client_events.send(event); //host client
    }
}
