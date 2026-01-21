use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    time::sleep,
};

use crate::network::clientid::ClientId;
use std::{collections::HashMap, time::Duration};

pub struct TTLManager {
    clients_by_id: HashMap<ClientId, u32>,
    pub reset_client_rx: UnboundedReceiver<ClientId>,
    pub client_died_tx: UnboundedSender<ClientId>,
}

impl TTLManager {
    pub fn new(
        reset_client_rx: UnboundedReceiver<ClientId>,
        client_died_tx: UnboundedSender<ClientId>,
    ) -> Self {
        Self {
            clients_by_id: HashMap::new(),
            reset_client_rx,
            client_died_tx,
        }
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
                // Handle resets from outside
                Some(client_id) = self.reset_client_rx.recv() => {
                    self.reset(client_id);
                }

                // Tick every second
                _ = sleep(Duration::from_secs(1)) => {
                    self.tick_death();

                    // Optional: check for clients that "died"
                    let mut dead_clients = vec![];
                    for (id, ttl) in self.clients_by_id.iter() {
                        if *ttl > 60 { // example TTL threshold
                            dead_clients.push(*id);
                        }
                    }

                    for id in dead_clients {
                        self.clients_by_id.remove(&id);
                        let _ = self.client_died_tx.send(id);
                    }
                }
            }
        }
    }

    fn tick_death(&mut self) {
        for (clientid, ttl) in self.clients_by_id.iter_mut() {
            *ttl += 1;
        }
    }
    fn reset(&mut self, id: ClientId) {
        self.clients_by_id.insert(id, 0);
    }
}
