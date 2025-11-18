use std::ops::Index;

use crate::game::state::{self, Kind, PlayerId, State};
pub const PLAYER_POSITIONS: [(f32, f32); 8] = [
    (50.0, 50.0),
    (270.0, 50.0),
    (50.0, 130.0),
    (270.0, 130.0),
    (160.0, 50.0),
    (160.0, 130.0),
    (90.0, 90.0),
    (230.0, 90.0),
];

pub struct SpawnManager {
    player_starts: Vec<(f32, f32)>,
    ball_start: Option<(f32, f32)>,
    ball_index: Option<usize>,
    player_count: u8,
    curr_player_count: u8,
    pub width: f32,
    pub height: f32,
}

impl SpawnManager {
    //Constructor
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            player_starts: Vec::new(),
            ball_start: None,
            ball_index: None,
            player_count: 1,
            curr_player_count: 0,
            width,
            height,
        }
    }

    pub fn add_single_player(
        &mut self,
        states: &mut Vec<State>,
        player_list: &mut Vec<PlayerId>,
    ) -> Option<PlayerId> {
        let id = self.curr_player_count;
        if id >= self.player_count {
            println!("Max players reached!");
            return None;
        }

        let (x, y) = PLAYER_POSITIONS[id as usize];
        let player_id = self.add_player(states, x, y)?;
        self.curr_player_count += 1;
        player_list.push(player_id);
        Some(player_id)
    }

    pub fn remove_player(
        &mut self,
        states: &mut Vec<State>,
        player_id: PlayerId,
        player_list: &mut Vec<PlayerId>,
    ) {
        states.remove(player_id.1);
        player_list.remove(player_id.1);
        self.curr_player_count -= 1;
    }

    pub fn remove_all(&mut self, states: &mut Vec<State>) {
        states.clear();
        self.player_count = 1;
        self.curr_player_count = 0;
        self.ball_index = None;
    }

    pub fn remove_non_player(&mut self, states: &mut Vec<State>) {
        states.truncate(self.curr_player_count as usize);
        self.player_count = 1;
        self.ball_index = None;
    }

    ///Public Functions
    pub fn spawn_states(&mut self, states: &mut Vec<State>) {
        //Borders
        self.create_borders(states);
        // Ball
        self.add_ball(states, 160.0, 90.0); // center

        // Goals
        self.add_goal(states, 0.0, 60.0, 0);
        self.add_goal(states, 290.0, 60.0, 1);
    }

    fn create_borders(&mut self, states: &mut Vec<State>) {
        let thickness = 10.0; // wall thickness

        // Top wall
        states.push(State::new_wall(0.0, -thickness, self.width, thickness));

        // Bottom wall
        states.push(State::new_wall(0.0, self.height, self.width, thickness));

        // Left wall
        states.push(State::new_wall(-thickness, 0.0, thickness, self.height));

        // Right wall
        states.push(State::new_wall(self.width, 0.0, thickness, self.height));
    }

    pub fn reset_states(&self, states: &mut Vec<State>) {
        for state in states.iter_mut() {
            if state.is_static {
                continue;
            }

            match state.kind {
                Kind::Ball => {
                    if let Some((bx, by)) = self.ball_start {
                        state.x = bx;
                        state.y = by;
                        state.held_by = None;
                        state.vx = 0.0;
                        state.vy = 0.0;
                    }
                }
                Kind::Player => {
                    let idx = state.get_player_id().unwrap().0 as usize;
                    let (px, py) = self.player_starts[idx];
                    state.x = px;
                    state.y = py;
                    state.held_by = None;
                    state.vx = 0.0;
                    state.vy = 0.0;
                }
                _ => {}
            }
        }
    }

    ///Private
    //Add Functions:
    pub fn add_player(&mut self, states: &mut Vec<State>, x: f32, y: f32) -> Option<PlayerId> {
        let player = State::new_player(x, y, states.len());

        if let Some(id) = player.get_player_id() {
            println!("Added player with ID: {} {}", id.0, id.1);
            states.push(player);
            self.player_starts.push((x, y));
            Some(id)
        } else {
            println!("Player has no ID!");
            None
        }
    }

    fn add_ball(&mut self, states: &mut Vec<State>, x: f32, y: f32) {
        states.push(State::new_ball(x, y));
        self.ball_start = Some((x, y));
        self.ball_index = Some(states.len() - 1);
    }

    fn add_goal(&mut self, states: &mut Vec<State>, x: f32, y: f32, team_id: u8) {
        states.push(State::new_goal(x, y, 30.0, 60.0, team_id));
    }

    ///Getters and Setters
    pub fn set_player_count(&mut self, player_count: u8) {
        self.player_count = player_count;
    }

    pub fn get_ball_index(&self) -> Option<usize> {
        return self.ball_index;
    }
}
