use crate::game::eventqueue::{EventQueue, GameEvent};
use crate::game::input::{GameAction, GameActionEvent, InputValue};
use crate::game::physics::Physics;
use crate::game::scoremanager::{ScoreManager, Team};
use crate::game::spawnmanager::SpawnManager;

use crate::game::state::entityid::EntityId;
use crate::game::world::World;
use std::collections::HashMap;

use tauri::window::Color;
use tauri::{AppHandle, Emitter};

#[derive(serde::Serialize, Clone, Debug)]
pub enum GamePhase {
    Waiting,
    Countdown { time_left: f32 },
    Playing,
    GameOver,
}

pub struct GameManager {
    pub world: World,
    pending_inputs: HashMap<EntityId, Vec<GameActionEvent>>,
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
            world: World::new(),
            pending_inputs: HashMap::new(),
            phase: GamePhase::Waiting,
            event_queue: EventQueue::new(),
            score_manager: score_manager,
            spawn_manager: SpawnManager::new(width, height),
        };

        gm
    }

    pub fn set_input(&mut self, player: EntityId, action: GameAction, value: InputValue) {
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
        self.spawn_manager.spawn_states(&mut self.world);
    }

    pub fn end_game(&mut self) {
        self.spawn_manager.remove_non_player(&mut self.world);
        self.score_manager.reset();
        self.phase = GamePhase::Waiting;
    }

    pub fn quit_game(&mut self) {
        self.spawn_manager.remove_all(&mut self.world);
        self.score_manager.reset();
        self.phase = GamePhase::Waiting;
    }

    pub fn try_get_new_player(&mut self) -> Option<EntityId> {
        let id = self.spawn_manager.try_add_player(&mut self.world);
        if id.is_some() {
            println!("Added new player with id {:?}", id);
            self.update_player_list();
        }
        return id;
    }

    pub fn remove_player(&mut self, id: EntityId) {
        self.spawn_manager.remove_player(&mut self.world, id);
        self.update_player_list();
    }

    fn update_player_list(&self) {
        let payload = self.world.player_list.clone();
        if let Err(err) = self.app.emit("player-list", payload) {
            eprintln!("Failed to emit player-list: {}", err);
        }
    }

    pub fn update(&mut self) {
        //Check Events
        for event in self.event_queue.drain() {
            match event {
                GameEvent::GoalScored { team_id } => {
                    if self.score_manager.add_point(team_id) {
                        self.phase = GamePhase::GameOver;
                    }
                    self.spawn_manager.reset_states(&mut self.world);
                    self.score_manager.enable_score();
                }
                GameEvent::ResetScore => {
                    self.score_manager.reset();
                }
                GameEvent::TryGrab { player_id } => {
                    if let Some(ball_id) = self.spawn_manager.get_ball_id() {
                        if let Some((ball, player)) =
                            self.world.grab_two_entities(ball_id, player_id)
                        {
                            if ball.held_by.is_some() {
                                return;
                            }
                            let dx = ball.x - player.x;
                            let dy = ball.y - player.y;

                            if dx * dx + dy * dy < GRAB_RADIUS.powi(2) {
                                ball.held_by = Some(player_id);
                                player.set_holding(true);
                            }
                        }
                    }
                }
                GameEvent::Shoot { player_id } => {
                    if let Some(ball_id) = self.spawn_manager.get_ball_id() {
                        if let Some((ball, player)) =
                            self.world.grab_two_entities(ball_id, player_id)
                        {
                            // Only shoot if this player is actually holding the ball
                            if ball.held_by != Some(player_id) {
                                println!("Ball held by: {:?}, cannot shoot", ball.held_by);
                                return; // not holding, can't shoot
                            }

                            // Release the ball and apply impulse
                            ball.held_by = None;
                            player.set_holding(false);
                            Physics::apply_impulse(ball, player.angle, 1000.0);
                        }
                    }
                }
                GameEvent::Place { player_id, pos } => {
                    self.spawn_manager
                        .add_brick(&mut self.world, pos, player_id);
                    if let Some(player) = &mut self.world.grab_entity(player_id) {
                        println!("Player {:?} placed a brick", player_id);
                        player.player_controller.as_mut().unwrap().add_brick();
                    }
                }
                GameEvent::Die { owner_id, brick_id } => {
                    self.spawn_manager.remove_brick(&mut self.world, brick_id);
                    if let Some(owner) = self.world.grab_entity(owner_id) {
                        owner.player_controller.as_mut().unwrap().remove_brick();
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
                        .world
                        .entities
                        .iter_mut()
                        .find(|s| s.entity_id == player_id)
                    {
                        for event in events {
                            let input = state.input();
                            match event.action {
                                GameAction::Move => input.move_axis = event.value.as_vec2(),
                                GameAction::Action => input.action = event.value.as_bool(),
                                GameAction::Aim => input.mouse_pos = event.value.as_vec2(),
                                GameAction::Look => input.look_pos = event.value.as_vec2(),
                                GameAction::Place => input.place = event.value.as_bool(),
                            }
                        }
                    }
                }
                self.pending_inputs.clear();

                Physics::update(&mut self.world, dt, &mut self.event_queue);
            }

            GamePhase::Waiting => { /* do nothing */ }
            GamePhase::GameOver => {
                self.end_game();
            }
        }
    }
}
