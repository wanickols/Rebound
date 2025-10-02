use crate::game::input::InputState;
use crate::game::physics::Physics;
use crate::game::state::State;

use tauri::AppHandle;

pub struct GameManager {
    pub states: Vec<State>,
    pub input: InputState,
    pub app: AppHandle,
    pub width: f32,
    pub height: f32,
}

impl GameManager {
    pub fn new(app: &AppHandle, width: f32, height: f32) -> Self {
        let _player = State::new_player(0.0, 0.0);

        let mut gm = Self {
            app: app.clone(),
            states: vec![_player], // also keep it in the states list
            input: InputState {
                up: false,
                down: false,
                left: false,
                right: false,
            },
            width,
            height,
        };

        gm.create_borders();

        gm
    }

    fn create_borders(&mut self) {
        let thickness = 10.0; // wall thickness

        // Top wall
        self.states.push(State {
            x: 0.0,
            y: -thickness,
            w: self.width,
            h: thickness,
            vx: 0.0,
            vy: 0.0,
            is_static: true,
            friction: 0.0,
            restitution: 1.0,
            kind: "wall".to_string(),
        });

        // Bottom wall
        self.states.push(State {
            x: 0.0,
            y: self.height,
            w: self.width,
            h: thickness,
            vx: 0.0,
            vy: 0.0,
            is_static: true,
            friction: 0.0,
            restitution: 1.0,
            kind: "wall".to_string(),
        });

        // Left wall
        self.states.push(State {
            x: -thickness,
            y: 0.0,
            w: thickness,
            h: self.height,
            vx: 0.0,
            vy: 0.0,
            is_static: true,
            friction: 0.0,
            restitution: 1.0,
            kind: "wall".to_string(),
        });

        // Right wall
        self.states.push(State {
            x: self.width,
            y: 0.0,
            w: thickness,
            h: self.height,
            vx: 0.0,
            vy: 0.0,
            is_static: true,
            friction: 0.0,
            restitution: 1.0,
            kind: "wall".to_string(),
        });
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

    pub fn add_state(&mut self, state: State) {
        self.states.push(state);
    }

    pub fn update(&mut self) {
        //Apply physics
        Physics::apply_input(
            &mut self.states[0],
            self.input.up,
            self.input.down,
            self.input.left,
            self.input.right,
        );
        let dt: f32 = 1.0 / 120.0; // ~0.016
        Physics::update(&mut self.states, dt);
    }
}
