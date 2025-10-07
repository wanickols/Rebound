use serde::{Deserialize, Serialize};

use crate::game::eventqueue::{EventQueue, GameEvent};
use crate::game::physics::Physics;
use crate::game::state::playerid::PlayerId;
use crate::game::util::Util;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub action: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
}

#[derive(Serialize, Copy, Clone)]
pub enum Kind {
    Player,
    Brick,
    Wall,
    Ball,
    Goal,
}

#[derive(Serialize, Clone)]
pub struct State {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub w: f32,
    pub h: f32,
    pub mass: f32,
    pub is_static: bool,
    pub is_trigger: bool,
    pub friction: f32,
    pub restitution: f32,
    pub kind: Kind,
    pub player_id: Option<PlayerId>,
    pub team_id: u8,
    pub input: InputState,
}

impl State {
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

    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.x + self.w, self.y + self.h)
    }

    pub fn check_collision_predicted(&self, other: &State, next_x: f32, next_y: f32) -> bool {
        let (ax1, ay1, ax2, ay2) = (next_x, next_y, next_x + self.w, next_y + self.h);
        let (bx1, by1, bx2, by2) = other.bounds();
        ax1 < bx2 && ax2 > bx1 && ay1 < by2 && ay2 > by1
    }

    //returns dx, dy, and overlap x and y
    pub fn find_overlap(&self, other: &State) -> (f32, f32, f32, f32) {
        // Compute centers
        let ax_center = self.x + self.w / 2.0;
        let ay_center = self.y + self.h / 2.0;
        let bx_center = other.x + other.w / 2.0;
        let by_center = other.y + other.h / 2.0;

        let dx = bx_center - ax_center;
        let dy = by_center - ay_center;

        let combined_half_width = (self.w + other.w) / 2.0;
        let combined_half_height = (self.h + other.h) / 2.0;

        let overlap_x = combined_half_width - dx.abs();
        let overlap_y = combined_half_height - dy.abs();

        return (dx, dy, overlap_x, overlap_y);
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

    pub fn handle_trigger_collision(&self, other: &State, events: &mut EventQueue) {
        match (self.kind, other.kind) {
            (Kind::Ball, Kind::Goal) => self.trigger_score(events),
            _ => {}
        }
    }

    pub fn trigger_score(&self, events: &mut EventQueue) {
        events.push(GameEvent::GoalScored {
            team_id: self.team_id,
        });
    }

    //New States
    pub fn new_wall(x: f32, y: f32, w: f32, h: f32) -> Self {
        State {
            x,
            y,
            w,
            h,
            mass: 1000.0,
            vx: 0.0,
            vy: 0.0,
            is_static: true,
            is_trigger: false,
            friction: 0.0,
            restitution: 0.8,
            kind: Kind::Wall,
            player_id: None,
            team_id: 0,
            input: InputState {
                up: false,
                down: false,
                left: false,
                right: false,
                action: false,
                mouse_x: 0.0,
                mouse_y: 0.0,
            },
        }
    }

    pub fn new_player(x: f32, y: f32) -> Self {
        State {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            w: 20.0, // arbitrary paddle/ball size for now
            h: 20.0,
            mass: 100.0,
            is_static: false,
            is_trigger: false,
            friction: 0.1,
            restitution: 0.6,
            kind: Kind::Player,
            player_id: Some(PlayerId::new()),
            team_id: 0,
            input: InputState {
                up: false,
                down: false,
                left: false,
                right: false,
                action: false,
                mouse_x: 0.0,
                mouse_y: 0.0,
            },
        }
    }

    pub fn new_ball(x: f32, y: f32) -> Self {
        State {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            w: 12.0, // smaller than player
            h: 12.0,
            mass: 1.0,
            is_static: false,
            is_trigger: false,
            friction: 0.01,
            restitution: 0.9,
            kind: Kind::Ball,
            player_id: None,
            team_id: 0,
            input: InputState {
                up: false,
                down: false,
                left: false,
                right: false,
                action: false,
                mouse_x: 0.0,
                mouse_y: 0.0,
            },
        }
    }
}
