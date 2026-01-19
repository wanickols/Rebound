use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashSet, net::SocketAddr};

use std::io::Result;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::watch;

use crate::network::{
    clientrequest::ClientRequest, networkinfo::NetworkInfo, serverevent::ServerEvent,
};

//Consider using state enum here to make sure bad socketmanager's can't be made :D
pub struct SocketManager {
    socket: Arc<UdpSocket>,
}

pub type SocketData = (SocketAddr, Vec<u8>);

impl Drop for SocketManager {
    fn drop(&mut self) {
        println!("Closing socket");
        Arc::try_unwrap(self.socket.clone())
            .ok()
            .map(|sock| drop(sock));
    }
}

impl SocketManager {
    pub async fn host(port: u16) -> Result<Self> {
        let bind_addr = SocketAddr::from(([0, 0, 0, 0], port));

        // Bind with std first
        let std_socket = std::net::UdpSocket::bind(bind_addr)?;
        std_socket.set_nonblocking(true)?;

        // Convert to Tokio socket
        let socket = UdpSocket::from_std(std_socket)?;

        let local_addr = socket.local_addr()?;
        println!("Hosting on {}", local_addr);

        Ok(Self {
            socket: Arc::new(socket),
        })
    }

    pub async fn join(port: u16) -> Result<Self> {
        // Bind to any available local port
        let bind_addr = SocketAddr::from(([0, 0, 0, 0], 0));
        let std_socket = std::net::UdpSocket::bind(bind_addr)?;
        std_socket.set_nonblocking(true)?;

        let socket = UdpSocket::from_std(std_socket)?;

        let host_addr = SocketAddr::from(([127, 0, 0, 1], port));

        // Fire off a join packet (bare minimum)
        socket.send_to(b"JOIN", host_addr).await?;

        println!("Joining host at {}", host_addr);

        Ok(Self {
            socket: Arc::new(socket),
        })
    }

    pub async fn run(
        &mut self,
        incoming_data_tx: UnboundedSender<SocketData>,
        mut outgoing_data_rx: UnboundedReceiver<SocketData>,
        shutdown_rx: watch::Receiver<bool>,
    ) {
        let socket = self.socket.clone();

        let mut buf = [0u8; 2048];

        loop {
            // --- Shutdown check ---
            if *shutdown_rx.borrow() {
                println!("Main loop shutting down");
                break;
            }

            // --- Receive with timeout ---
            match tokio::time::timeout(Duration::from_millis(50), socket.recv_from(&mut buf)).await
            {
                Ok(Ok((len, addr))) => {
                    let bytes = buf[..len].to_vec();
                    if let Err(e) = incoming_data_tx.send((addr, bytes)) {
                        eprintln!(
                        "incoming_data_tx.send FAILED — receiver dropped. addr={addr}, err={e:?}"
                    );
                    }
                }
                Ok(Err(e)) => eprintln!("recv_from error: {e}"),
                Err(_) => {
                    // Timeout — no data received, that's fine
                }
            }

            // --- Send outgoing messages ---
            while let Ok((addr, bytes)) = outgoing_data_rx.try_recv() {
                self.send_data((addr, bytes)).await;
            }
        }
    }

    pub async fn send_data(&self, data: SocketData) {
        if let Err(e) = self.socket.send_to(&data.1, data.0).await {
            eprintln!("Failed to send to host {}: {e}", data.0);
        }
    }
}
