use crate::game::gamepayload::GamePayload;

pub enum ServerEvent {
    WorldSnapshot { snapshot: GamePayload },
}
