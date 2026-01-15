use std::{collections::btree_map, net::SocketAddr};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::network::{
    clientid::{self, ClientId},
    clientrequest::{ClientMessage, ClientRequest},
    serverevent::ServerEvent,
    socketmanager::{self, SocketData},
};

pub struct ClientNetworkHandler {
    //Client
    incoming_client_request: UnboundedReceiver<ClientMessage>,
    outgoing_server_event: UnboundedSender<ServerEvent>,

    //Socket
    incoming_socket_data: UnboundedReceiver<SocketData>,
    outgoing_socket_data: UnboundedSender<SocketData>,
    host_addr: Option<SocketAddr>,
}

impl ClientNetworkHandler {
    pub fn new(
        incoming_client_request: UnboundedReceiver<ClientMessage>,
        outgoing_server_event: UnboundedSender<ServerEvent>,
        incoming_socket_data: UnboundedReceiver<SocketData>,
        outgoing_socket_data: UnboundedSender<SocketData>,
    ) -> Self {
        Self {
            incoming_client_request,
            outgoing_server_event,
            incoming_socket_data,
            outgoing_socket_data,
            host_addr: None,
        }
    }

    pub fn init_host_addr(&mut self, addr: SocketAddr) {
        self.host_addr = Some(addr);
    }

    pub async fn start_listening(&mut self) {
        loop {
            tokio::select! {
                Some(dta) = self.incoming_socket_data.recv() => self.handle_socket_data(dta).await,
                Some(req) = self.incoming_client_request.recv() => self.handle_client_request(req).await,
                else => break, // all channels closed, shutdown
            }
        }
    }

    async fn handle_socket_data(&mut self, data: SocketData) {
        println!("recieved:");
        let (peer_addr, bytes) = data;

        //deserialize
        let msg: ServerEvent = match serde_json::from_slice(&bytes) {
            Ok(m) => m,
            Err(e) => {
                eprintln!(
                    "Failed to deserialize client message from {}: {}",
                    peer_addr, e
                );
                return;
            }
        };

        // --- Handshake path ---
        if matches!(msg, ServerEvent::Joined { .. }) {
            self.handle_joined(peer_addr, msg);
            return;
        }

        // --- Authority enforcement ---
        match self.host_addr {
            Some(host) if host == peer_addr => {
                let _ = self.outgoing_server_event.send(msg);
            }
            _ => {
                // Drop anything not from the host
            }
        }
    }

    fn handle_joined(&mut self, peer_addr: SocketAddr, msg: ServerEvent) {
        match msg {
            ServerEvent::Joined { client_id } => {
                // First contact: establish host
                if self.host_addr.is_none() {
                    self.host_addr = Some(peer_addr);
                    //send client join
                    return;
                }

                // Second contact: finalize identity
                if let Some(id) = client_id {
                    let _ = self.outgoing_server_event.send(ServerEvent::Joined {
                        client_id: Some(id),
                    });
                }
            }
            _ => {}
        }
    }

    //just from client
    async fn handle_client_request(&self, msg: ClientMessage) {
        if self.host_addr.is_none() {
            eprintln!("Host address does not exist");
            return;
        }

        match serde_json::to_vec(&msg) {
            Ok(bytes) => {
                let _ = self
                    .outgoing_socket_data
                    .send((self.host_addr.unwrap(), bytes));
            }
            Err(e) => {
                eprintln!("Failed to serialize ClientMessage: {}", e);
            }
        }
    }
}
