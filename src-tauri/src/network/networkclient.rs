use std::sync::Arc;

use tokio::sync::mpsc::UnboundedSender;

use crate::network::clientrequest::ClientRequest;

pub struct NetworkClient {
    pub client_request_sender: Arc<UnboundedSender<ClientRequest>>,
}

impl NetworkClient {
    pub fn send_request(&self, req: ClientRequest) {
        let _ = self.client_request_sender.send(req);
    }
}
