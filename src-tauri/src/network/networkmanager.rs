use std::sync::{Arc, Mutex, MutexGuard};

use tauri::Emitter;

use crate::game::gamemanager::GameManager;
use crate::game::state::entityid::EntityId;
use crate::network::{
    clientrequest::ClientRequest, serverevent::ServerEvent, socketmanager::SocketManager,
};

#[derive(Clone, Copy)]
pub enum Role {
    Host,
    Client,
}

pub struct NetworkManager {
    role: Role,
    gm: Option<Arc<Mutex<GameManager>>>,
    socket: SocketManager,
}

impl NetworkManager {
    pub fn new(role: Role, gm: Option<Arc<Mutex<GameManager>>>) -> Self {
        Self {
            socket: SocketManager::new(role),
            role, //copy
            gm,
        }
    }
    pub fn process_request(&mut self, request: ClientRequest) -> Option<EntityId> {
        match self.role {
            Role::Host => {
                // Apply directly to GM
                if let Some(gm) = &self.gm {
                    let mut gm = gm.lock().unwrap();
                    match request {
                        ClientRequest::Add => {
                            return gm.try_get_new_player();
                        }
                        ClientRequest::Remove { id } => {
                            gm.remove_player(id);
                            return None;
                        }
                        ClientRequest::Input { entity_id, frame } => {
                            gm.queue_input(entity_id, frame);
                            return None;
                        }
                    }
                }
            }
            Role::Client => {
                // Serialize and send to host
                self.socket.send_to_host(request);
            }
        }
        None
    }

    pub fn send_server_event(&self, gm: &MutexGuard<'_, GameManager>, event: ServerEvent) {
        match event {
            ServerEvent::WorldSnapshot { snapshot } => {
                if let Err(err) = gm.app.emit("game-state", snapshot.clone()) {
                    eprintln!("Failed to emit game-state: {}", err);
                }
                self.socket
                    .broadcast(ServerEvent::WorldSnapshot { snapshot });
            }
        }
    }
}
