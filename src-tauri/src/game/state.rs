use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct State {
    pub x: f32,
    pub y: f32,
}
