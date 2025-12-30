use std::{collections::HashSet, net::SocketAddr};

use std::io::Result;
use tokio::net::UdpSocket;

use crate::network::{
    clientrequest::ClientRequest, networkinfo::NetworkInfo, networkmanager::Role,
    serverevent::ServerEvent,
};

pub struct SocketManager {
    socket: UdpSocket,
    is_host: bool,
    host_addr: Option<SocketAddr>,
    peers: HashSet<SocketAddr>,
}

impl SocketManager {
    pub async fn new(role: Role) -> Self {
        match role {
            Role::Host { port } => Self::host(port).await.expect("Failed to host"),
            Role::Client { host_addr } => Self::join(host_addr).await.expect("Failed to join host"),
        }
    }

    pub async fn host(port: u16) -> Result<Self> {
        let bind_addr = SocketAddr::from(([0, 0, 0, 0], port));
        let socket = UdpSocket::bind(bind_addr).await?;

        let local_addr = socket.local_addr()?;
        println!("Hosting on {}", local_addr);

        Ok(Self {
            socket,
            is_host: true,
            host_addr: None,
            peers: HashSet::new(),
        })
    }

    pub async fn join(host_addr: SocketAddr) -> std::io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        let local_addr = socket.local_addr()?;

        println!("Joining host {} from {}", host_addr, local_addr);

        Ok(Self {
            socket,
            is_host: false,
            host_addr: Some(host_addr),
            peers: HashSet::new(),
        })
    }

    pub async fn poll(&mut self, buf: &mut [u8]) -> Result<Option<(usize, SocketAddr)>> {
        let (len, addr) = self.socket.recv_from(buf).await?;

        if self.peers.insert(addr) {
            println!("New peer joined: {}", addr);
        }

        Ok(Some((len, addr)))
    }

    pub fn network_info(&self) -> NetworkInfo {
        NetworkInfo {
            is_host: self.is_host,
            local_addr: self
                .socket
                .local_addr()
                .ok()
                .expect("no local address found"),
            host_addr: self.host_addr,
            connected_peers: self.peers.iter().cloned().collect(),
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
