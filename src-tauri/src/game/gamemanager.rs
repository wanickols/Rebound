use crate::game::eventqueue::{EventQueue, GameEvent};
use crate::game::input::InputFrame;
use crate::game::physics::Physics;
use crate::game::scoremanager::{ScoreManager, Team};
use crate::game::spawnmanager::SpawnManager;

use crate::game::state::entityid::EntityId;
use crate::game::world::World;
use crate::network::clientrequest::ClientRequest;
use crate::network::serverevent::ServerEvent;
use std::collections::HashMap;

use tauri::window::Color;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum GamePhase {
    Waiting,
    Countdown { time_left: f32 },
    Playing,
    GameOver,
}

pub struct GameManager {
    pub world: World,
    pending_inputs: HashMap<EntityId, InputFrame>,
    pub snapshot_tx: Option<UnboundedSender<ServerEvent>>,
    pub client_request_rx: Option<UnboundedReceiver<ClientRequest>>,
    pub phase: GamePhase,
    pub event_queue: EventQueue,
    pub score_manager: ScoreManager,
    pub spawn_manager: SpawnManager,
}

pub const GRAB_RADIUS: f32 = 32.0;
pub const DT: f32 = 0.016; // ~0.016

impl GameManager {
    pub fn new(width: f32, height: f32) -> Self {
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
            world: World::new(),
            pending_inputs: HashMap::new(),
            snapshot_tx: None,
            client_request_rx: None,
            phase: GamePhase::Waiting,
            event_queue: EventQueue::new(),
            score_manager: score_manager,
            spawn_manager: SpawnManager::new(width, height),
        };

        gm
    }

    pub fn init_channels(
        &mut self,
        snapshot_tx: Option<UnboundedSender<ServerEvent>>,
        client_request_rx: Option<UnboundedReceiver<ClientRequest>>,
    ) {
        self.snapshot_tx = snapshot_tx;
        self.client_request_rx = client_request_rx;
    }

    pub fn queue_input(&mut self, player: EntityId, frame: InputFrame) {
        self.pending_inputs.insert(player, frame);
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
            //self.update_player_list();
        }
        return id;
    }

    pub fn remove_player(&mut self, id: EntityId) {
        self.spawn_manager.remove_player(&mut self.world, id);
        //self.update_player_list();
    }

    // fn update_player_list(&self) {
    //     let payload = self.world.player_list.clone();
    //     if let Err(err) = self.app.emit("player-list", payload) {
    //         eprintln!("Failed to emit player-list: {}", err);
    //     }
    // }

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
                GameEvent::TryGrab { player_id } => {
                    if let Some(ball_id) = self.spawn_manager.get_ball_id() {
                        if let Some((ball, player)) =
                            self.world.grab_two_entities(ball_id, player_id)
                        {
                            if ball.held_by.is_some() {
                                return;
                            }
                            let dx = ball.physics_state.pos.x - player.physics_state.pos.x;
                            let dy = ball.physics_state.pos.y - player.physics_state.pos.y;

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
                            Physics::apply_impulse(
                                &mut ball.physics_state,
                                player.physics_state.angle,
                                1000.0,
                            );
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
                *time_left -= DT;

                if *time_left <= 0.0 {
                    self.phase = GamePhase::Playing;
                }
            }

            GamePhase::Playing => {
                //Apply physics
                for (player_id, frame) in self.pending_inputs.drain() {
                    if let Some(state) = self
                        .world
                        .entities
                        .iter_mut()
                        .find(|s| s.entity_id == player_id)
                    {
                        if let Some(controller) = &mut state.player_controller {
                            controller.input = frame;
                        }
                    }
                }

                Physics::update(&mut self.world, DT, &mut self.event_queue);
            }

            GamePhase::Waiting => { /* do nothing */ }
            GamePhase::GameOver => {
                self.end_game();
            }
        }
    }
}
