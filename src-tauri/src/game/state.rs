use serde::Serialize;

use crate::game::playerid::PlayerId;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub action: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
}

#[derive(Serialize, Clone)]
pub struct State {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub w: f32,
    pub h: f32,
    pub mass: f32,
    pub is_static: bool,
    pub friction: f32,
    pub restitution: f32,
    pub kind: String,
    pub player_id: Option<PlayerId>,
    pub input: InputState,
}

impl State {
    pub fn apply_friction(&mut self, dt: f32) {
        self.vx *= 1.0 - self.friction * dt;
        self.vy *= 1.0 - self.friction * dt;
    }

    pub fn stop_if_tiny(&mut self) {
        if self.vx.abs() < 0.01 {
            self.vx = 0.0;
        }
        if self.vy.abs() < 0.01 {
            self.vy = 0.0;
        }
    }

    pub fn predict_position(&self, dt: f32) -> (f32, f32) {
        (self.x + self.vx * dt, self.y + self.vy * dt)
    }

    pub fn update_position(&mut self, dt: f32) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
    }

    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.x + self.w, self.y + self.h)
    }

    pub fn new_wall(x: f32, y: f32, w: f32, h: f32) -> Self {
        State {
            x,
            y,
            w,
            h,
            mass: 1000.0,
            vx: 0.0,
            vy: 0.0,
            is_static: true,
            friction: 0.0,
            restitution: 0.8,
            kind: "wall".to_string(),
            player_id: None,
            input: InputState {
                up: false,
                down: false,
                left: false,
                right: false,
                action: false,
                mouse_x: 0.0,
                mouse_y: 0.0,
            },
        }
    }

    pub fn new_player(x: f32, y: f32) -> Self {
        State {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            w: 20.0, // arbitrary paddle/ball size for now
            h: 20.0,
            mass: 100.0,
            is_static: false,
            friction: 0.1,
            restitution: 0.6,
            kind: "player".to_string(),
            player_id: Some(PlayerId::new()),
            input: InputState {
                up: false,
                down: false,
                left: false,
                right: false,
                action: false,
                mouse_x: 0.0,
                mouse_y: 0.0,
            },
        }
    }

    pub fn new_ball(x: f32, y: f32) -> Self {
        State {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            w: 12.0, // smaller than player
            h: 12.0,
            mass: 1.0,
            is_static: false,
            friction: 0.01,
            restitution: 0.9,
            kind: "ball".to_string(),
            player_id: None,
            input: InputState {
                up: false,
                down: false,
                left: false,
                right: false,
                action: false,
                mouse_x: 0.0,
                mouse_y: 0.0,
            },
        }
    }
}
