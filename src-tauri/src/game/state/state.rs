pub mod physicsstate;
pub mod renderstate;

#[path = "entityid.rs"]
pub mod entityid;

use serde::{Deserialize, Serialize};

use crate::game::eventqueue::{EventQueue, GameEvent};
use crate::game::input::inputframe::Vec2;
use crate::game::input::playercontroller::PlayerController;
use crate::game::physics::Physics;
use crate::game::state::entityid::EntityId;
use crate::game::state::physicsstate::PhysicsState;
use crate::game::util::Util;

#[derive(Serialize, Copy, Clone, Debug)]
pub enum Kind {
    Player,
    Brick,
    Wall,
    Ball,
    Goal,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Shape {
    Circle { radius: f32 },
    Rectangle { w: f32, h: f32 },
}

#[derive(Clone)]
pub struct State {
    pub physics_state: PhysicsState,
    pub is_static: bool,
    pub is_enabled: bool,
    pub is_trigger: bool,
    pub is_alive: bool,
    pub kind: Kind,
    pub entity_id: EntityId,
    pub time_to_live: Option<u16>,
    pub team_id: Option<u8>,
    pub held_by: Option<EntityId>,
    pub owner_id: Option<EntityId>,
    pub player_controller: Option<PlayerController>,
}

impl State {
    //Internal update function for states
    pub fn tick(&mut self, dt: f32, events: &mut EventQueue) {
        if !self.is_alive {
            return;
        }

        self.physics_state.tick(dt);

        if let Some(ttl) = self.time_to_live {
            //println!("dying counter: {}", ttl);

            if ttl <= 1 {
                self.time_to_live = None;
                self.die(events);
            } else {
                self.time_to_live = Some(ttl - 1);
            }
        }

        if let Some(pc) = &mut self.player_controller {
            pc.tick(dt);
        }
    }

    //Tick Helpers
    fn die(&mut self, events: &mut EventQueue) {
        self.is_alive = false;
        self.is_static = true;
        if let Some(owner_id) = self.owner_id {
            events.push(GameEvent::Die {
                owner_id: owner_id,
                brick_id: self.entity_id,
            });
        } else {
            println!("Entity {:?} died without an owner", self.entity_id);
        }
    }

    pub fn handle_collision(states: &mut Vec<State>, i: usize, j: usize, events: &mut EventQueue) {
        let (a, b) = Util::two_mut(states, i, j);

        // Declare variables here
        let (nx, ny, overlap) = match (&a.physics_state.shape, &b.physics_state.shape) {
            (Shape::Circle { .. }, Shape::Circle { .. }) => {
                // For circle-circle, use full overlap
                if let Some((nx, ny, overlap)) =
                    PhysicsState::find_overlap(&a.physics_state, &b.physics_state)
                {
                    (nx, ny, overlap)
                } else {
                    // No collision; early return
                    return;
                }
            }
            _ => {
                // Everything else: simple arcade bounce
                a.compute_min_axis_overlap(b)
            }
        };

        // Trigger handling
        if a.is_trigger {
            a.handle_trigger_collision(b, events);
        } else if b.is_trigger {
            b.handle_trigger_collision(a, events);
        } else {
            // Resolve physics
            Physics::resolve_pair(&mut a.physics_state, &mut b.physics_state, nx, ny, overlap);
        }
    }

    pub fn handle_pure_trigger(
        states: &mut Vec<State>,
        i: usize,
        j: usize,
        events: &mut EventQueue,
    ) {
        let (a, b) = Util::two_mut(states, i, j);

        match (&a.physics_state.shape, &b.physics_state.shape) {
            (Shape::Circle { .. }, Shape::Circle { .. }) => {
                if let Some((nx, ny, overlap)) =
                    PhysicsState::find_overlap(&a.physics_state, &b.physics_state)
                {
                    (nx, ny, overlap)
                } else {
                    return; // no overlap
                }
            }
            _ => a.compute_min_axis_overlap(b),
        };

        if a.is_trigger {
            a.handle_trigger_collision(b, events);
        }
        if b.is_trigger {
            b.handle_trigger_collision(a, events);
        }
    }

    // Compute the minimum-penetration axis for simple arcade collisions
    fn compute_min_axis_overlap(&self, other: &State) -> (f32, f32, f32) {
        // Extract sizes from shapes
        let (aw, ah, ax_center, ay_center) = match &self.physics_state.shape {
            Shape::Rectangle { w, h } => (
                w,
                h,
                self.physics_state.pos.x + w / 2.0,
                self.physics_state.pos.y + h / 2.0,
            ),
            Shape::Circle { radius } => {
                let d = radius * 2.0;
                (
                    &d.clone(),
                    &d.clone(),
                    self.physics_state.pos.x,
                    self.physics_state.pos.y,
                ) // x,y = center
            }
        };
        let (bw, bh, bx_center, by_center) = match &other.physics_state.shape {
            Shape::Rectangle { w, h } => (
                w,
                h,
                other.physics_state.pos.x + w / 2.0,
                other.physics_state.pos.y + h / 2.0,
            ),
            Shape::Circle { radius } => {
                let d = radius * 2.0;
                (
                    &d.clone(),
                    &d.clone(),
                    other.physics_state.pos.x,
                    other.physics_state.pos.y,
                ) // x,y = center
            }
        };

        // Delta between centers
        let dx = bx_center - ax_center;
        let dy = by_center - ay_center;

        // Combined half sizes
        let combined_half_width = (aw + bw) / 2.0;
        let combined_half_height = (ah + bh) / 2.0;

        // Overlap along each axis
        let overlap_x = combined_half_width - dx.abs();
        let overlap_y = combined_half_height - dy.abs();

        // Pick axis of minimum penetration
        if overlap_x < overlap_y {
            (dx.signum(), 0.0, overlap_x)
        } else {
            (0.0, dy.signum(), overlap_y)
        }
    }

    pub fn handle_trigger_collision(&self, other: &State, events: &mut EventQueue) {
        match (self.kind, other.kind) {
            (Kind::Goal, Kind::Ball) => self.trigger_score(events),
            _ => {}
        }
    }

    ///Handling triggers and interactions
    fn trigger_score(&self, events: &mut EventQueue) {
        println!("Trigggered");
        events.push(GameEvent::GoalScored {
            team_id: self.team_id.expect("Goal state must have a team_id"),
        });
    }

    pub fn set_holding(&mut self, holding: bool) {
        if let Some(pc) = &mut self.player_controller {
            pc.is_holding = holding;
        }
    }

    pub fn is_holding(&self) -> bool {
        self.player_controller
            .as_ref()
            .map_or(false, |pc| pc.is_holding)
    }

    //New States
    pub fn new() -> Self {
        State {
            physics_state: PhysicsState::new(),
            is_static: false,
            is_enabled: true,
            is_trigger: false,
            is_alive: true,
            kind: Kind::Ball,
            time_to_live: None,
            entity_id: EntityId::new(),
            owner_id: None,
            team_id: None,
            held_by: None,
            player_controller: None,
        }
    }

    pub fn new_wall(x: f32, y: f32, w: f32, h: f32) -> Self {
        let mut s = State::new(); // base defaults
        s.physics_state.pos.x = x;
        s.physics_state.pos.y = y;
        s.physics_state.shape = Shape::Rectangle { w, h };
        s.physics_state.mass = 1000.0;
        s.physics_state.restitution = 0.4;
        s.physics_state.is_static = true;
        s.kind = Kind::Wall;
        s
    }

    pub fn new_player(x: f32, y: f32) -> Self {
        let mut s = State::new();
        s.physics_state.pos = Vec2 { x, y };
        s.physics_state.shape = Shape::Circle { radius: 5.0 };
        s.physics_state.mass = 100.0;
        s.physics_state.friction = 20.0;
        s.physics_state.restitution = 0.6;
        s.kind = Kind::Player;
        s.player_controller = Some(PlayerController::new(75.0, 400.0, s.entity_id));
        s
    }

    pub fn new_ball(x: f32, y: f32) -> Self {
        let mut s = State::new();
        s.physics_state.pos = Vec2 { x, y };
        s.physics_state.shape = Shape::Circle { radius: 3.0 };
        s.physics_state.mass = 1.0;
        s.physics_state.friction = 8.0;
        s.physics_state.restitution = 0.9;
        s.kind = Kind::Ball;
        s
    }

    pub fn new_brick(x: f32, y: f32, w: f32, entity_id: EntityId) -> Self {
        let mut s = State::new();
        s.physics_state.pos = Vec2 { x, y };
        s.physics_state.shape = Shape::Rectangle { w, h: w };
        s.kind = Kind::Brick;
        s.physics_state.mass = 20.0;
        s.time_to_live = Some(60 * 5); // 5 seconds
        s.is_static = false;
        s.owner_id = Some(entity_id);
        s
    }

    pub fn new_goal(x: f32, y: f32, w: f32, h: f32, team_id: u8) -> Self {
        let mut s = State::new();
        s.physics_state.pos = Vec2 { x, y };
        s.physics_state.shape = Shape::Rectangle { w, h };
        s.kind = Kind::Goal;
        s.is_trigger = true;
        s.is_static = true;

        s.team_id = Some(team_id);
        s
    }
}
