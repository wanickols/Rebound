use crate::network::{
    clientrequest::ClientRequest, networkmanager::Role, serverevent::ServerEvent,
};

pub struct SocketManager {}

impl SocketManager {
    pub fn new(role: Role) -> Self {
        SocketManager {}
    }

    pub fn broadcast(&self, event: ServerEvent) {
        match event {
            ServerEvent::WorldSnapshot { snapshot } => {
                // Broadcast world snapshot to clients
            }
        }
    }

    pub fn send_to_host(&self, request: ClientRequest) {}
}
