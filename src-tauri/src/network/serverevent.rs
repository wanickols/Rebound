use crate::game::gamepayload::GamePayload;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ServerEvent {
    WorldSnapshot { snapshot: GamePayload },
}
