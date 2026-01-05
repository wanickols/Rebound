use std::sync::{Arc, Mutex, MutexGuard};

use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;

use crate::game::gamemanager::GameManager;
use crate::game::state::entityid::EntityId;
use crate::network::{
    clientrequest::ClientRequest, serverevent::ServerEvent, socketmanager::SocketManager,
};

#[derive(Clone, Copy)]
pub enum Role {
    Host { port: u16 },
    Client { host_addr: std::net::SocketAddr },
}

pub struct NetworkManager {
    role: Role,
    gm: Option<Arc<Mutex<GameManager>>>,
    pub snapshot_receiver: mpsc::UnboundedReceiver<ServerEvent>,
    pub client_request_receiver: mpsc::UnboundedReceiver<ClientRequest>,
    socket: Option<SocketManager>,
    app: tauri::AppHandle,
}

impl NetworkManager {
    pub async fn new(
        role: Role,
        gm: Option<Arc<Mutex<GameManager>>>,
        snapshot_receiver: mpsc::UnboundedReceiver<ServerEvent>,
        client_request_receiver: mpsc::UnboundedReceiver<ClientRequest>,
        app: AppHandle,
    ) -> std::io::Result<Self> {
        Ok(Self {
            socket: None,
            role, //copy
            gm,
            snapshot_receiver,
            client_request_receiver,
            app: app,
        })
    }

    pub async fn init_socket(&mut self, role: Role) -> std::io::Result<()> {
        self.role = role;
        let socket = SocketManager::new(role).await;
        self.socket = Some(socket);
        Ok(())
    }
    pub fn process_request(&self, request: ClientRequest) -> Option<EntityId> {
        println!("Processing request:");

        match self.role {
            Role::Host { port } => {
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
            Role::Client { host_addr } => {
                if let Some(socket) = &self.socket {
                    socket.send_to_host(request);
                }
            }
        }
        None
    }

    pub fn poll(&mut self) {
        let mut buf = [0u8; 1024];
        if let Some(socket) = &mut self.socket {
            if let Ok(Some((len, addr))) = socket.try_recv_from(&mut buf) {
                println!("Received {} bytes from {}", len, addr);
                self.process_packet(&buf[..len], addr);
            }
        }
    }

    fn process_packet(&self, data: &[u8], addr: std::net::SocketAddr) {
        match self.role {
            Role::Host { port } => {
                // Deserialize and process client request
                if let Ok(request) = serde_json::from_slice::<ClientRequest>(data) {
                    self.process_request(request);
                }
            }
            Role::Client { host_addr } => {
                // Deserialize and process server event
                if let Ok(event) = serde_json::from_slice::<ServerEvent>(data) {
                    self.send_server_event(event);
                }
            }
        }
    }

    pub fn send_server_event(&self, event: ServerEvent) {
        if self.socket.is_none() {
            return;
        }

        match event {
            ServerEvent::WorldSnapshot { snapshot } => {
                if let Err(err) = self.app.emit("game-state", snapshot.clone()) {
                    eprintln!("Failed to emit game-state: {}", err);
                }

                self.socket
                    .as_ref()
                    .unwrap()
                    .broadcast(&ServerEvent::WorldSnapshot { snapshot });
            }
        }
    }
}
