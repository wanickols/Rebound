use std::net::SocketAddr;

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct NetworkInfo {
    pub is_host: bool,
    pub local_addr: SocketAddr,
    pub host_addr: Option<SocketAddr>,
    pub connected_peers: Vec<SocketAddr>,
}
impl NetworkInfo {
    pub fn new(
        is_host: bool,
        local_addr: SocketAddr,
        host_addr: Option<SocketAddr>,
        connected_peers: Vec<SocketAddr>,
    ) -> Self {
        Self {
            is_host,
            local_addr,
            host_addr,
            connected_peers,
        }
    }
}
