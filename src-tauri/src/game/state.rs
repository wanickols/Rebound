use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct State {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub w: f32,
    pub h: f32,
    pub is_static: bool,
    pub friction: f32,
    pub restitution: f32,
    pub kind: String,
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

    pub fn new_player(x: f32, y: f32) -> Self {
        State {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            w: 20.0, // arbitrary paddle/ball size for now
            h: 20.0,
            is_static: false,
            friction: 0.05,
            restitution: 0.9,
            kind: "player".to_string(),
        }
    }
}
