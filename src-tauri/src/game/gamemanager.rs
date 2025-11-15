use crate::end_game;
use crate::game::eventqueue::{EventQueue, GameEvent};
use crate::game::input::{
    playercontroller, GameAction, GameActionEvent, InputValue, PlayerController,
};
use crate::game::physics::Physics;
use crate::game::scoremanager::{self, ScoreManager, Team};
use crate::game::spawnmanager::SpawnManager;
use crate::game::state::{PlayerId, State};
use crate::game::util::Util;
use std::collections::HashMap;

use tauri::window::Color;
use tauri::AppHandle;

#[derive(serde::Serialize, Clone, Debug)]
pub enum GamePhase {
    Waiting,
    Countdown { time_left: f32 },
    Playing,
    GameOver,
}

pub struct GameManager {
    pub states: Vec<State>,
    pending_inputs: HashMap<PlayerId, Vec<GameActionEvent>>,
    pub app: AppHandle,
    pub phase: GamePhase,
    pub event_queue: EventQueue,
    pub score_manager: ScoreManager,
    pub spawn_manager: SpawnManager,
}

pub const GRAB_RADIUS: f32 = 32.0;
pub const dt: f32 = 0.016; // ~0.016

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

        let gm = Self {
            app: app.clone(),
            states: vec![], // also keep it in the states list
            pending_inputs: HashMap::new(),
            phase: GamePhase::Waiting,
            event_queue: EventQueue::new(),
            score_manager: score_manager,
            spawn_manager: SpawnManager::new(width, height),
        };

        gm
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

    pub fn set_game_settings(&mut self, player_count: u8, target_score: u8) {
        self.spawn_manager.set_player_count(player_count);
        self.score_manager.set_target_score(target_score);
    }

    pub fn start_game(&mut self) {
        self.phase = GamePhase::Countdown { time_left: 3.0 };
        self.spawn_manager.spawn_states(&mut self.states);
    }

    pub fn end_game(&mut self) {
        self.spawn_manager.remove_all(&mut self.states);
        self.score_manager.reset();
        self.phase = GamePhase::Waiting;
    }

    pub fn try_get_new_player(&mut self) -> Option<PlayerId> {
        return self.spawn_manager.add_single_player(&mut self.states);
    }

    pub fn update(&mut self) {
        //Check Events
        for event in self.event_queue.drain() {
            match event {
                GameEvent::GoalScored { team_id } => {
                    if self.score_manager.add_point(team_id) {
                        self.phase = GamePhase::GameOver;
                    }
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

        //GamePhase
        match &mut self.phase {
            GamePhase::Countdown { time_left } => {
                *time_left -= dt;

                if *time_left <= 0.0 {
                    self.phase = GamePhase::Playing;
                }
            }

            GamePhase::Playing => {
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
                                GameAction::Move => input.move_axis = event.value.as_vec2(),
                                GameAction::Action => input.action = event.value.as_bool(),
                                GameAction::Aim => input.mouse_pos = event.value.as_vec2(),
                                GameAction::Look => input.look_pos = event.value.as_vec2(),
                            }
                        }
                    }
                }
                self.pending_inputs.clear();

                Physics::update(&mut self.states, dt, &mut self.event_queue);
            }

            GamePhase::Waiting => { /* do nothing */ }
            GamePhase::GameOver => {
                self.end_game();
            }
        }
    }
}
