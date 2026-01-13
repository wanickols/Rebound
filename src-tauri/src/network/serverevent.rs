use crate::{
    game::{gamepayload::GamePayload, state::entityid::EntityId},
    network::clientid::ClientId,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ServerEvent {
    Joined { client_id: Option<ClientId> },
    WorldSnapshot { snapshot: GamePayload },
    AddedPlayer { entity: EntityId, client: ClientId },
}
