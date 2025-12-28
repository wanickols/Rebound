use serde::Deserialize;

// Vec2 type for movement/look
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

// Button state
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Buttons {
    pub grab: bool,
    pub place: bool,
    pub dash: bool, //todo: implement dash
}

// InputFrame sent by frontend
#[derive(Debug, Clone, Deserialize)]
pub struct InputFrame {
    pub move_axis: Vec2, // `move` is a keyword in Rust
    pub look: Vec2,
    pub buttons: Buttons,
}

impl InputFrame {
    pub fn new() -> Self {
        Self {
            move_axis: Vec2 { x: 0.0, y: 0.0 },
            look: Vec2 { x: 0.0, y: 0.0 },
            buttons: Buttons {
                grab: false,
                place: false,
                dash: false,
            },
        }
    }
}
