use std::{collections::HashSet, net::SocketAddr};

use tokio::net::UdpSocket;

use crate::network::{
    clientrequest::ClientRequest, networkmanager::Role, serverevent::ServerEvent,
};

pub struct SocketManager {
    socket: UdpSocket,
    is_host: bool,
    host_addr: Option<SocketAddr>,
    peers: HashSet<SocketAddr>,
}

impl SocketManager {
    pub fn new(role: Role) -> Self {
        //todo join or host to get socket
        Self {
            socket,
            is_host,
            host_addr: None,
            peers: HashSet::new(),
        }
    }

    //todo Join and Host using tok

    pub fn broadcast(&self, event: ServerEvent) {
        match event {
            ServerEvent::WorldSnapshot { snapshot } => {
                // Broadcast world snapshot to clients
            }
        }
    }

    pub fn send_to_host(&self, request: ClientRequest) {}
}
