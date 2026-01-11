use crate::{
    game::{input::InputFrame, state::entityid::EntityId},
    network::clientid::ClientId,
};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ClientRequest {
    Add,
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
    pub client_id: ClientId,
    pub request: ClientRequest,
}