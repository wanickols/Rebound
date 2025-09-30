use crate::game::state::State;
use crate::game::physics::Physics;
use crate::game::input::InputState;

use tauri::{AppHandle, };


pub struct GameManager {
    pub state: State,
    pub input: InputState,
    pub app: AppHandle,
}

impl GameManager {
    pub fn new(app: &AppHandle) -> Self {
        Self {
            app: app.clone(),
            state: State { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0},
            input: InputState { up: false, down: false, left: false, right: false }
        }
    }

pub fn set_input(&mut self, key: &str, pressed: bool) {
    match key {
        "ArrowUp" => self.input.up = pressed,
        "ArrowDown" => self.input.down = pressed,
        "ArrowLeft" => self.input.left = pressed,
        "ArrowRight" => self.input.right = pressed,
        _ => {}
    }
}


    pub fn update(&mut self) {
         //Apply physics
         Physics::apply_input(
            &mut self.state,
            self.input.up,
            self.input.down,
            self.input.left,
            self.input.right,
        );
         let dt: f32 = 1.0 / 60.0; // ~0.016
         Physics::update(&mut self.state, dt);

    
    }
}