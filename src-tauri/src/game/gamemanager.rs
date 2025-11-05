use crate::game::eventqueue::{EventQueue, GameEvent};
use crate::game::input::{
    playercontroller, GameAction, GameActionEvent, InputValue, PlayerController,
};
use crate::game::physics::Physics;
use crate::game::scoremanager::{ScoreManager, Team};
use crate::game::spawnmanager::SpawnManager;
use crate::game::state::{PlayerId, State};
use crate::game::util::Util;
use std::collections::HashMap;

use tauri::window::Color;
use tauri::AppHandle;

pub struct GameManager {
    pub states: Vec<State>,
    pending_inputs: HashMap<PlayerId, Vec<GameActionEvent>>,
    pub app: AppHandle,
    pub width: f32,
    pub height: f32,
    pub event_queue: EventQueue,
    pub score_manager: ScoreManager,
    pub spawn_manager: SpawnManager,
}

pub const GRAB_RADIUS: f32 = 32.0;

impl GameManager {
    pub fn new(app: &AppHandle, width: f32, height: f32) -> Self {
        let score_manager = ScoreManager::new(
            Team {
                id: 0,
                name: "Blue".into(),
                color: Color(0, 0, 255, 255),
                score: 0,
            },
            Team {
                id: 1,
                name: "Red".into(),
                color: Color(255, 0, 0, 255),
                score: 0,
            },
        );

        let mut gm = Self {
            app: app.clone(),
            states: vec![], // also keep it in the states list
            pending_inputs: HashMap::new(),
            width,
            height,
            event_queue: EventQueue::new(),
            score_manager: score_manager,
            spawn_manager: SpawnManager::new(),
        };

        gm.spawn_manager.spawn_states(&mut gm.states);

        gm.create_borders();

        gm
    }

    fn create_borders(&mut self) {
        let thickness = 10.0; // wall thickness

       
        // Top wall
        self.states
            .push(State::new_wall(0.0, -thickness, self.width, thickness));

        // Bottom wall
        self.states
            .push(State::new_wall(0.0, self.height, self.width, thickness));

        // Left wall
        self.states
            .push(State::new_wall(-thickness, 0.0, thickness, self.height));

        // Right wall
        self.states
            .push(State::new_wall(self.width, 0.0, thickness, self.height));
    }

    pub fn set_input(&mut self, player: PlayerId, action: GameAction, value: InputValue) {
        self.pending_inputs
            .entry(player)
            .or_default()
            .push(GameActionEvent {
                action,
                value,
                timestamp: 0,
            });
    }

    pub fn add_state(&mut self, state: State) {
        self.states.push(state);
    }

    pub fn update(&mut self) {
        //Check Events
        for event in self.event_queue.drain() {
            match event {
                GameEvent::GoalScored { team_id } => {
                    self.score_manager.add_point(team_id);
                    self.spawn_manager.reset_states(&mut self.states);
                    self.score_manager.enable_score();
                }
                GameEvent::ResetScore => {
                    self.score_manager.reset();
                }
                GameEvent::TryGrab { player_id } => {
                    if let Some(ball_idx) = self.spawn_manager.get_ball_index() {
                        let (ball, player) = Util::two_mut(&mut self.states, ball_idx, player_id.1);
                        if (ball.held_by.is_some()) {
                            return;
                        }
                        let dx = ball.x - player.x;
                        let dy = ball.y - player.y;

                        if dx * dx + dy * dy < GRAB_RADIUS.powi(2) {
                            ball.held_by = Some(player_id);
                        }
                    }
                }
                GameEvent::Shoot { player_id } => {
                    if let Some(ball_idx) = self.spawn_manager.get_ball_index() {
                        let (ball, player) = Util::two_mut(&mut self.states, ball_idx, player_id.1);

                        // Only shoot if this player is actually holding the ball
                        if ball.held_by != Some(player_id) {
                            return; // not holding, can't shoot
                        }

                        // Optional: check grab radius as a sanity check
                        let dx = ball.x - player.x;
                        let dy = ball.y - player.y;
                        if dx * dx + dy * dy > GRAB_RADIUS.powi(2) {
                            return; // too far away, can't shoot
                        }

                        // Release the ball and apply impulse
                        ball.held_by = None;
                        player.set_holding(false);
                        Physics::apply_impulse(ball, player.angle, 1000.0); // tune power as needed
                    }
                }
            }
        }

        //Apply physics
        for (&player_id, events) in &self.pending_inputs {
            if let Some(state) = self
                .states
                .iter_mut()
                .find(|s| s.get_player_id() == Some(player_id))
            {
                for event in events {
                    let input = state.input();
                    match event.action {
                        GameAction::Up => input.up = event.value.as_bool(),
                        GameAction::Down => input.down = event.value.as_bool(),
                        GameAction::Left => input.left = event.value.as_bool(),
                        GameAction::Right => input.right = event.value.as_bool(),
                        GameAction::Action => input.action = event.value.as_bool(),
                        GameAction::MouseMove => {
                            input.mouse_pos = event.value.as_vec2();
                        }
                    }
                }
            }
        }
        self.pending_inputs.clear();
        let dt: f32 = 1.0 / 120.0; // ~0.016
        Physics::update(&mut self.states, dt, &mut self.event_queue);
    }
}
