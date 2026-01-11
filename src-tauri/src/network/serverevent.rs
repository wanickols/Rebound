use crate::{
    game::{gamepayload::GamePayload, state::entityid::EntityId},
    network::clientid::ClientId,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ServerEvent {
    WorldSnapshot { snapshot: GamePayload },
    AddedPlayer { entity: EntityId, client: ClientId },
}
