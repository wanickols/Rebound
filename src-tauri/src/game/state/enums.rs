use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Kind {
    Player,
    Brick,
    Wall,
    Ball,
    Goal,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Moving,
    Dashing,
    Shooting,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Shape {
    Circle { radius: f32 },
    Rectangle { w: f32, h: f32 },
}
