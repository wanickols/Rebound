use crate::{
    game::{input::InputFrame, state::entityid::EntityId},
    network::clientid::ClientId,
};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ClientRequest {
    Add,
    Joined,
    Remove {
        id: EntityId,
    },
    Input {
        entity_id: EntityId,
        frame: InputFrame,
    },
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ClientMessage {
    pub client_id: Option<ClientId>,
    pub request: ClientRequest,
}

impl ClientMessage {
    pub fn new(client_id: ClientId, request: ClientRequest) -> Self {
        Self {
            client_id: Some(client_id),
            request,
        }
    }
}
