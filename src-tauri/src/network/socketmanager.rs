use std::{collections::HashSet, net::SocketAddr};

use std::io::Result;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::watch;

use crate::network::{
    clientrequest::ClientRequest, networkinfo::NetworkInfo, serverevent::ServerEvent,
};

//Consider using state enum here to make sure bad socketmanager's can't be made :D
pub struct SocketManager {
    socket: Option<UdpSocket>,
    host_addr: Option<SocketAddr>,
    data_tx: UnboundedSender<SocketData>,
}

pub type SocketData = (SocketAddr, Vec<u8>);

impl Drop for SocketManager {
    fn drop(&mut self) {
        println!("Closing socket");
    }
}

impl SocketManager {
    pub fn new(data_tx: UnboundedSender<SocketData>) -> Self {
        Self {
            socket: None,
            host_addr: None,
            data_tx,
        }
    }

    pub async fn host(&mut self, port: u16) -> Result<()> {
        let bind_addr = SocketAddr::from(([0, 0, 0, 0], port));

        // Bind with std first
        let std_socket = std::net::UdpSocket::bind(bind_addr)?;

        // Convert to Tokio socket
        let socket = UdpSocket::from_std(std_socket)?;

        let local_addr = socket.local_addr()?;
        println!("Hosting on {}", local_addr);

        self.socket = Some(socket);
        self.host_addr = None;

        Ok(())
    }

    //listens, and sends to bytes to network manager
    pub async fn poll_socket(&mut self, mut shutdown_rx: watch::Receiver<bool>) -> Result<()> {
        let mut buf = [0u8; 1024];

        // Take the socket out of the Option temporarily to avoid unwraps
        let socket = self.socket.take().expect("Socket not initialized");

        loop {
            tokio::select! {
                // Shutdown signal triggered
                _ = shutdown_rx.changed() => {
                    if *shutdown_rx.borrow() {
                        println!("Socket shutting down");
                        break;
                    }
                }

                // Try to receive from the socket
                result = socket.recv_from(&mut buf) => {
                    match result {
                        Ok((len, addr)) => {
                            let bytes = buf[..len].to_vec();
                            let _ = self.data_tx.send((addr, bytes));
                            tokio::task::yield_now().await;
                        }
                        Err(e) => {
                            eprintln!("Socket recv error: {e}");
                            // Optionally break or continue depending on error
                        }
                    }
                }
            }
        }

        println!("Socket poll loop exited");

        // Put the socket back into self so it can be reused/dropped
        self.socket = Some(socket);

        Ok(())
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
