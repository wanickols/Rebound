use std::net::SocketAddr;

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct NetworkInfo {
    pub local_addr: SocketAddr,
    pub host_addr: Option<SocketAddr>,
    pub connected_peers: Vec<SocketAddr>,
}
impl NetworkInfo {
    pub fn new(
        local_addr: SocketAddr,
        host_addr: Option<SocketAddr>,
        connected_peers: Vec<SocketAddr>,
    ) -> Self {
        Self {
            local_addr,
            host_addr,
            connected_peers,
        }
    }
}
