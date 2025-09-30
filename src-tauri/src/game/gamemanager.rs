use crate::game::state::State;
use crate::game::physics::Physics;

use tauri::{AppHandle, Emitter};


pub struct GameManager {
    pub state: State,
    app: AppHandle,
}

impl GameManager {
    pub fn new(app: &AppHandle) -> Self {
        Self {
            app: app.clone(),
            state: State { x: 0.0, y: 0.0},
        }
    }

    pub fn update(&mut self, up: bool, down: bool, left: bool, right: bool) {
         Physics::apply_input(&mut self.state, up, down, left, right);
         println!("Player position: ({}, {})", self.state.x, self.state.y);
         self.app.emit("game-state", self.state.clone()).unwrap();
    }
}