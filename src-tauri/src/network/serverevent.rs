use crate::game::{gamepayload::GamePayload, state::entityid::EntityId};

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ServerEvent {
    WorldSnapshot { snapshot: GamePayload },
    AddedPlayer { entity: EntityId },
}
