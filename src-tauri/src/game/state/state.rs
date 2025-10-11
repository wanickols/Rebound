use serde::{Deserialize, Serialize};

use crate::game::eventqueue::{EventQueue, GameEvent};
use crate::game::physics::Physics;
use crate::game::state::playerid::{self, PlayerId};
use crate::game::util::Util;

#[derive(Default, Clone, Deserialize)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub action: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            up: false,
            down: false,
            left: false,
            right: false,
            action: false,
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }
}

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
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub shape: Shape,
    pub mass: f32,
    pub is_static: bool,
    pub is_trigger: bool,
    pub friction: f32,
    pub restitution: f32,
    pub kind: Kind,
    pub player_id: Option<PlayerId>,
    pub team_id: Option<u8>,
    pub input: Option<InputState>,
}

impl State {
    //Public Functions
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

    pub fn check_collision_predicted(&self, other: &State, next_x: f32, next_y: f32) -> bool {
        // only handle rectangles; if it's not a rectangle, bail out (here: return false)
        let (w, h) = if let Shape::Rectangle { w, h } = &self.shape {
            (*w, *h)
        } else {
            return false; // or panic!("Circle not supported yet")
        };

        let (ax1, ay1, ax2, ay2) = (next_x, next_y, next_x + w, next_y + h);
        let (bx1, by1, bx2, by2) = other.bounds();
        ax1 < bx2 && ax2 > bx1 && ay1 < by2 && ay2 > by1
    }

    pub fn handle_collision(states: &mut Vec<State>, i: usize, j: usize, events: &mut EventQueue) {
        let (a, b) = Util::two_mut(states, i, j);

        let (dx, dy, overlap_x, overlap_y) = a.find_overlap(b);

        if overlap_x <= 0.0 || overlap_y <= 0.0 {
            return;
        }

        if a.is_trigger || b.is_trigger {
            if a.is_trigger {
                a.handle_trigger_collision(b, events);
            }
            if b.is_trigger {
                b.handle_trigger_collision(a, events);
            }
            return;
        }

        Physics::resolve_pair(a, b, dx, dy, overlap_x, overlap_y);
    }

    //Helper Functions
    fn bounds(&self) -> (f32, f32, f32, f32) {
        match &self.shape {
            Shape::Rectangle { w, h } => (self.x, self.y, self.x + w, self.y + h),
            Shape::Circle { radius } => {
                // if you want to approximate the circle with a bounding box
                let d = radius * 2.0;
                (self.x, self.y, self.x + d, self.y + d)
            }
        }
    }

    //returns dx, dy, and overlap x and y
    fn find_overlap(&self, other: &State) -> (f32, f32, f32, f32) {
        // Extract sizes from shapes
        let (aw, ah) = match &self.shape {
            Shape::Rectangle { w, h } => (*w, *h),
            Shape::Circle { radius } => {
                let d = *radius * 2.0;
                (d, d)
            }
        };

        let (bw, bh) = match &other.shape {
            Shape::Rectangle { w, h } => (*w, *h),
            Shape::Circle { radius } => {
                let d = *radius * 2.0;
                (d, d)
            }
        };

        // Compute centers
        let ax_center = self.x + aw / 2.0;
        let ay_center = self.y + ah / 2.0;
        let bx_center = other.x + bw / 2.0;
        let by_center = other.y + bh / 2.0;

        let dx = bx_center - ax_center;
        let dy = by_center - ay_center;

        let combined_half_width = (aw + bw) / 2.0;
        let combined_half_height = (ah + bh) / 2.0;

        let overlap_x = combined_half_width - dx.abs();
        let overlap_y = combined_half_height - dy.abs();

        (dx, dy, overlap_x, overlap_y)
    }

    fn handle_trigger_collision(&self, other: &State, events: &mut EventQueue) {
        match (self.kind, other.kind) {
            (Kind::Goal, Kind::Ball) => self.trigger_score(events),
            _ => {}
        }
    }

    fn trigger_score(&self, events: &mut EventQueue) {
        println!("Trigggered");
        events.push(GameEvent::GoalScored {
            team_id: self.team_id.expect("Goal state must have a team_id"),
        });
    }

    //New States
    pub fn new() -> Self {
        State {
            x: 0.0,
            y: 0.0,
            shape: Shape::Rectangle { w: 1.0, h: 1.0 },
            vx: 0.0,
            vy: 0.0,
            mass: 1.0,
            is_static: false,
            is_trigger: false,
            friction: 0.0,
            restitution: 0.5,
            kind: Kind::Ball,
            player_id: None,
            team_id: None,
            input: None,
        }
    }

    pub fn new_wall(x: f32, y: f32, w: f32, h: f32) -> Self {
        let mut s = State::new(); // base defaults
        s.x = x;
        s.y = y;
        s.shape = Shape::Rectangle { w, h };
        s.mass = 1000.0;
        s.is_static = true;
        s.kind = Kind::Wall;
        s
    }

    pub fn new_player(x: f32, y: f32) -> Self {
        let mut s = State::new();
        s.x = x;
        s.y = y;
        s.shape = Shape::Rectangle { w: 20.0, h: 20.0 };
        s.mass = 100.0;
        s.friction = 0.1;
        s.restitution = 0.6;
        s.kind = Kind::Player;
        s.player_id = Some(PlayerId::new());
        s.input = Some(InputState::new());
        s
    }

    pub fn new_ball(x: f32, y: f32) -> Self {
        let mut s = State::new();
        s.x = x;
        s.y = y;
        s.shape = Shape::Rectangle { w: 12.0, h: 12.0 };
        s.mass = 1.0;
        s.friction = 0.01;
        s.restitution = 0.9;
        s.kind = Kind::Ball;
        s
    }

    pub fn new_goal(x: f32, y: f32, w: f32, h: f32, team_id: u8) -> Self {
        let mut s = State::new();
        s.x = x;
        s.y = y;
        s.shape = Shape::Rectangle { w, h };
        s.kind = Kind::Goal;
        s.is_trigger = true;
        s.is_static = true;

        s.team_id = Some(team_id);
        s
    }
}
