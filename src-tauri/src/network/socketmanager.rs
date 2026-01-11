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
    host_addr: Option<SocketAddr>,
}

impl SocketManager {
    pub fn new() -> Self {
        Self {
            socket: None,
            host_addr: None,
        }
    }

    pub async fn host(&mut self, port: u16) -> Result<()> {
        let bind_addr = SocketAddr::from(([0, 0, 0, 0], port));
        let socket = UdpSocket::bind(bind_addr).await?;

        let local_addr = socket.local_addr()?;
        println!("Hosting on {}", local_addr);

        self.socket = Some(socket);
        self.host_addr = None;

        Ok(())
    }

    //listens, and sends to bytes to network manager
    pub fn poll_socket(&mut self, tx: &UnboundedSender<(SocketAddr, Vec<u8>)>) {
        let mut buf = [0u8; 1024];
        while let Some((len, addr)) = self.try_recv_from(&mut buf) {
            let bytes = buf[..len].to_vec();
            let _ = tx.send((addr, bytes));
        }
    }

    //checks for any data on socket
    fn try_recv_from(&mut self, buf: &mut [u8]) -> Option<(usize, SocketAddr)> {
        let socket = match self.socket.as_mut() {
            Some(s) => s,
            None => return None, // socket doesn't exist yet, silently skip
        };

        match socket.try_recv_from(buf) {
            Ok((len, addr)) => Some((len, addr)),
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
