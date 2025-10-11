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
        match &self.shape {
            // Rectangle vs Rectangle
            Shape::Rectangle { w, h } => {
                if let Shape::Rectangle { .. } = &other.shape {
                    let (ax1, ay1, ax2, ay2) = (next_x, next_y, next_x + w, next_y + h);
                    let (bx1, by1, bx2, by2) = other.bounds();
                    return ax1 < bx2 && ax2 > bx1 && ay1 < by2 && ay2 > by1;
                }
                // Rectangle vs Circle -> flip to Circle vs Rectangle
                if let Shape::Circle { .. } = &other.shape {
                    return other.check_collision_predicted(self, other.x, other.y);
                }
                false
            }

            // Circle vs Circle
            Shape::Circle { radius } => {
                if let Shape::Circle { radius: br } = &other.shape {
                    let dx = (next_x) - other.x;
                    let dy = (next_y) - other.y;
                    return dx * dx + dy * dy < (radius + br).powi(2);
                }

                // Circle vs Rectangle
                if let Shape::Rectangle { w, h } = &other.shape {
                    let closest_x = next_x.clamp(other.x, other.x + w);
                    let closest_y = next_y.clamp(other.y, other.y + h);

                    let dx = next_x - closest_x;
                    let dy = next_y - closest_y;

                    return dx * dx + dy * dy < radius.powi(2);
                }

                false
            }

            _ => false,
        }
    }

    pub fn handle_collision(states: &mut Vec<State>, i: usize, j: usize, events: &mut EventQueue) {
        let (a, b) = Util::two_mut(states, i, j);

        // Declare variables here
        let (nx, ny, overlap) = match (&a.shape, &b.shape) {
            (Shape::Circle { .. }, Shape::Circle { .. }) => {
                // For circle-circle, use full overlap
                if let Some((nx, ny, overlap)) = State::find_overlap(a, b) {
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
            Physics::resolve_pair(a, b, nx, ny, overlap);
        }
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
    pub fn find_overlap(a: &State, b: &State) -> Option<(f32, f32, f32)> {
        // Try to cast both shapes to circles
        let ar = if let Shape::Circle { radius } = a.shape {
            radius
        } else {
            return None;
        };
        let br = if let Shape::Circle { radius } = b.shape {
            radius
        } else {
            return None;
        };

        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let dist = (dx * dx + dy * dy).sqrt();
        let combined_r = ar + br;

        if dist < combined_r {
            let nx = if dist == 0.0 { 1.0 } else { dx / dist };
            let ny = if dist == 0.0 { 0.0 } else { dy / dist };
            let overlap = combined_r - dist;
            Some((nx, ny, overlap))
        } else {
            None
        }
    }

    /// Compute the minimum-penetration axis for simple arcade collisions
    pub fn compute_min_axis_overlap(&self, other: &State) -> (f32, f32, f32) {
        // Extract sizes from shapes
        let (aw, ah, ax_center, ay_center) = match &self.shape {
            Shape::Rectangle { w, h } => (*w, *h, self.x + w / 2.0, self.y + h / 2.0),
            Shape::Circle { radius } => {
                let d = *radius * 2.0;
                (d, d, self.x, self.y) // x,y = center
            }
        };
        let (bw, bh, bx_center, by_center) = match &other.shape {
            Shape::Rectangle { w, h } => (*w, *h, other.x + w / 2.0, other.y + h / 2.0),
            Shape::Circle { radius } => {
                let d = *radius * 2.0;
                (d, d, other.x, other.y) // x,y = center
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

    pub fn radius(&self) -> f32 {
        match self.shape {
            Shape::Circle { radius } => radius,
            _ => 0.0,
        }
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
        s.restitution = 0.4;
        s.is_static = true;
        s.kind = Kind::Wall;
        s
    }

    pub fn new_player(x: f32, y: f32) -> Self {
        let mut s = State::new();
        s.x = x;
        s.y = y;
        s.shape = Shape::Circle { radius: 10.0 };
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
        s.shape = Shape::Circle { radius: 6.0 };
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
