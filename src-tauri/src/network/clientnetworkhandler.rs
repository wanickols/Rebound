use std::{collections::btree_map, net::SocketAddr, time::Duration};

use tauri::http::Request;
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    time,
};

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
        let mut heartbeat = time::interval(Duration::from_secs(5)); // ping every 5s
        loop {
            tokio::select! {
                Some(dta) = self.incoming_socket_data.recv() => {
                    self.handle_socket_data(dta).await;
                }
                Some(req) = self.incoming_client_request.recv() => {
                    self.handle_client_request(req).await;
                }
                _ = heartbeat.tick() => {
                    self.send_request(ClientRequest::Idle).await;
                }
                else => break, // all channels closed, shutdown
            }
        }
    }

    async fn handle_socket_data(&mut self, data: SocketData) {
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
            self.handle_joined(peer_addr, msg).await;
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

    async fn handle_joined(&mut self, peer_addr: SocketAddr, msg: ServerEvent) {
        println!("Recieving Join data");
        match msg {
            ServerEvent::Joined { client_id } => {
                // First contact: establish host
                if self.host_addr.is_none() {
                    self.host_addr = Some(peer_addr);
                    self.send_request(ClientRequest::Joined).await;

                    return;
                }

                // Second contact: finalize identity
                if let Some(id) = client_id {
                    println!("Sending data 2");
                    let _ = self.outgoing_server_event.send(ServerEvent::Joined {
                        client_id: Some(id),
                    });
                }
            }
            _ => {}
        }
    }

    async fn send_request(&self, request: ClientRequest) {
        let msg = ClientMessage::new(ClientId::new(), request);
        self.handle_client_request(msg).await;
    }

    //just from client
    async fn handle_client_request(&self, msg: ClientMessage) {
        if self.host_addr.is_none() {
            eprintln!("Host address does not exist");
            return;
        }
        match serde_json::to_vec(&msg) {
            Ok(bytes) => {
                match self
                    .outgoing_socket_data
                    .send((self.host_addr.unwrap(), bytes))
                {
                    Ok(_) => {
                        println!("Enqueued outgoing data for sending");
                    }
                    Err(e) => {
                        eprintln!("Failed to enqueue outgoing data: {e}");
                    }
                }

                println!("Sending data 1");
            }
            Err(e) => {
                eprintln!("Failed to serialize ClientMessage: {}", e);
            }
        }
    }
}
