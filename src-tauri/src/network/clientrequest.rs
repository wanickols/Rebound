use crate::game::{input::InputFrame, state::entityid::EntityId};

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
