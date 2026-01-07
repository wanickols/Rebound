use std::sync::Arc;

use tokio::sync::mpsc::UnboundedSender;

use crate::network::clientrequest::ClientRequest;

pub struct NetworkClient {
    pub client_request_tx: Option<UnboundedSender<ClientRequest>>,
}

impl NetworkClient {
    pub fn new() -> Self {
        Self {
            client_request_tx: None,
        }
    }

    pub fn init_sender(&mut self, client_request_tx: UnboundedSender<ClientRequest>) {
        self.client_request_tx = Some(client_request_tx);
    }

    pub fn send_request(&self, req: ClientRequest) {
        if let Some(tx) = &self.client_request_tx {
            // ignore the Result; it only fails if receiver dropped
            let _ = tx.send(req);
        }
        // else: sender doesn't exist yet, silently skip
    }
}
