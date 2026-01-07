use std::{collections::HashSet, net::SocketAddr};

use std::io::Result;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::UnboundedSender;

use crate::network::{
    clientrequest::ClientRequest, networkinfo::NetworkInfo, serverevent::ServerEvent,
};

//Consider using state enum here to make sure bad socketmanager's can't be made :D
pub struct SocketManager {
    socket: Option<UdpSocket>,
    is_host: bool,
    host_addr: Option<SocketAddr>,
    peers: HashSet<SocketAddr>,
    client_request_tx: Option<UnboundedSender<ClientRequest>>,
}

impl SocketManager {
    pub fn new(client_request_tx: UnboundedSender<ClientRequest>) -> Self {
        Self {
            socket: None,
            is_host: false,
            host_addr: None,
            peers: HashSet::new(),
            client_request_tx: Some(client_request_tx),
        }
    }

    pub async fn host(&mut self, port: u16) -> Result<()> {
        let bind_addr = SocketAddr::from(([0, 0, 0, 0], port));
        let socket = UdpSocket::bind(bind_addr).await?;

        let local_addr = socket.local_addr()?;
        println!("Hosting on {}", local_addr);

        self.socket = Some(socket);
        self.is_host = true;
        self.host_addr = None;
        self.peers.clear();

        Ok(())
    }

    // pub async fn join(host_addr: SocketAddr) -> std::io::Result<Self> {
    //     let socket = UdpSocket::bind("0.0.0.0:0").await?;
    //     let local_addr = socket.local_addr()?;

    //     println!("Joining host {} from {}", host_addr, local_addr);

    //     Ok(Self {
    //         socket,
    //         is_host: false,
    //         host_addr: Some(host_addr),
    //         peers: HashSet::new(),
    //     })
    // }

    pub fn try_recv_from(&mut self, buf: &mut [u8]) -> Option<(usize, SocketAddr)> {
        let socket = match self.socket.as_mut() {
            Some(s) => s,
            None => return None, // socket doesn't exist yet, silently skip
        };

        match socket.try_recv_from(buf) {
            Ok((len, addr)) => {
                if self.peers.insert(addr) {
                    println!("New peer joined: {}", addr);
                }
                Some((len, addr))
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // no packet available, not an error
                None
            }
            Err(e) => {
                eprintln!("Socket error: {e}");
                None
            }
        }
    }

    pub fn network_info(&self) -> Option<NetworkInfo> {
        let socket = match self.socket.as_ref() {
            Some(s) => s,
            None => return None, // socket doesn't exist yet, silently skip
        };

        Some(NetworkInfo {
            is_host: self.is_host,
            local_addr: socket.local_addr().ok().expect("no local address found"),
            host_addr: self.host_addr,
            connected_peers: self.peers.iter().cloned().collect(),
        })
    }

    //todo Join and Host using tok

    pub async fn broadcast(&self, event: &ServerEvent) {
        let socket = match self.socket.as_ref() {
            Some(s) => s,
            None => return, // socket doesn't exist yet, silently skip
        };

        let bytes = match serde_json::to_vec(event) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Failed to serialize ServerEvent: {e}");
                return;
            }
        };

        for addr in self.peers.iter() {
            if let Err(e) = socket.send_to(&bytes, addr).await {
                eprintln!("Failed to send to {addr}: {e}");
            }
        }
    }

    pub async fn send_to_host(&self, request: ClientRequest) {
        let socket = match self.socket.as_ref() {
            Some(s) => s,
            None => return, // socket doesn't exist yet, silently skip
        };

        if let Some(host_addr) = self.host_addr {
            let bytes = match serde_json::to_vec(&request) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("Failed to serialize ClientRequest: {e}");
                    return;
                }
            };

            if let Err(e) = socket.send_to(&bytes, host_addr).await {
                eprintln!("Failed to send to host {host_addr}: {e}");
            }
        }
    }
}
