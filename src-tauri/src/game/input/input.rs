pub mod playercontroller;

pub use playercontroller::PlayerController;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum GameAction {
    Move,
    Action,
    Aim,  //mouse
    Look, //controller
}

pub struct GameActionEvent {
    pub action: GameAction,
    pub value: InputValue,
    pub timestamp: u64, // optional for reconciliation / replay
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum InputValue {
    Bool(bool),
    Vec2 { x: f32, y: f32 },
    Float(f32),
    None,
}

impl InputValue {
    pub fn as_bool(&self) -> bool {
        match self {
            InputValue::Bool(v) => *v,
            _ => false,
        }
    }

    pub fn as_vec2(&self) -> (f32, f32) {
        match self {
            InputValue::Vec2 { x, y } => (*x, *y),
            _ => (0.0, 0.0),
        }
    }
}
