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
    incoming_data_tx: UnboundedSender<SocketData>,
    outgoing_data_rx: UnboundedReceiver<SocketData>,
}

pub type SocketData = (SocketAddr, Vec<u8>);

impl Drop for SocketManager {
    fn drop(&mut self) {
        println!("Closing socket");
    }
}

impl SocketManager {
    pub async fn host(
        incoming_data_tx: UnboundedSender<SocketData>,
        outgoing_data_rx: UnboundedReceiver<SocketData>,
        port: u16,
    ) -> Result<Self> {
        let bind_addr = SocketAddr::from(([0, 0, 0, 0], port));

        // Bind with std first
        let std_socket = std::net::UdpSocket::bind(bind_addr)?;

        // Convert to Tokio socket
        let socket = UdpSocket::from_std(std_socket)?;

        let local_addr = socket.local_addr()?;
        println!("Hosting on {}", local_addr);

        Ok(Self {
            socket: Arc::new(socket),
            incoming_data_tx,
            outgoing_data_rx,
        })
    }

    pub async fn join(
        incoming_data_tx: UnboundedSender<SocketData>,
        outgoing_data_rx: UnboundedReceiver<SocketData>,
        port: u16,
    ) -> Result<Self> {
        // Bind to any available local port
        let bind_addr = SocketAddr::from(([0, 0, 0, 0], 0));
        let std_socket = std::net::UdpSocket::bind(bind_addr)?;

        let socket = UdpSocket::from_std(std_socket)?;

        let host_addr = SocketAddr::from(([127, 0, 0, 1], port));

        // Fire off a join packet (bare minimum)
        socket.send_to(b"JOIN", host_addr).await?;

        println!("Joining host at {}", host_addr);

        Ok(Self {
            socket: Arc::new(socket),
            incoming_data_tx,
            outgoing_data_rx,
        })
    }

    pub async fn run(&mut self, shutdown_rx: watch::Receiver<bool>) {
        let mut buf = [0u8; 1024];
        loop {
            // Check shutdown first
            if *shutdown_rx.borrow() {
                break;
            }

            println!("tick");

            // --- Poll the socket ---
            match tokio::time::timeout(Duration::from_millis(50), self.socket.recv_from(&mut buf))
                .await
            {
                Ok(Ok((len, addr))) => {
                    let bytes = buf[..len].to_vec();
                    // Send the incoming data to the handler
                    let _ = self.incoming_data_tx.send((addr, bytes));
                }
                Ok(Err(e)) => {
                    eprintln!("Socket error: {e}");
                }
                Err(_) => {
                    // Timeout, no data received, continue
                }
            }

            println!("tick2");

            // Optional: small sleep to avoid hot spin
            tokio::time::sleep(Duration::from_millis(1)).await;

            // --- Handle outgoing messages ---
            while let Ok(msg) = self.outgoing_data_rx.try_recv() {
                self.send_data(msg).await;
            }

            // Optional: small sleep to avoid hot spin
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }

    pub async fn send_data(&self, data: SocketData) {
        println!("Sending Data");
        if let Err(e) = self.socket.send_to(&data.1, data.0).await {
            eprintln!("Failed to send to host {}: {e}", data.0);
        }
    }
}
