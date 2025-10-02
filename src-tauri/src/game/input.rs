use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum GameAction {
    Up,
    Down,
    Left,
    Right,
    Action,
}

pub struct GameActionEvent {
    pub action: GameAction,
    pub pressed: bool,
    pub timestamp: u64, // optional for reconciliation / replay
}
