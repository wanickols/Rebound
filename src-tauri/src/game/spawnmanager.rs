use crate::game::state::{self, Kind, State};

pub struct SpawnManager {
    player_starts: Vec<(f32, f32)>,
    ball_start: Option<(f32, f32)>,
    ball_index: Option<usize>,
    player_count: u8,
    target_score: u8,
}

impl SpawnManager {
    //Constructor
    pub fn new() -> Self {
        Self {
            player_starts: Vec::new(),
            ball_start: None,
            ball_index: None,
            player_count: 1,
            target_score: 1,
        }
    }

    ///Public Functions
    pub fn spawn_states(&mut self, states: &mut Vec<State>) {
        // Hardcoded positions for up to 8 players
        let player_positions = [
            (50.0, 50.0),
            (270.0, 50.0),
            (50.0, 130.0),
            (270.0, 130.0),
            (160.0, 50.0),
            (160.0, 130.0),
            (90.0, 90.0),
            (230.0, 90.0),
        ];

        for i in 0..self.player_count.min(8) as usize {
            let (x, y) = player_positions[i];
            self.add_player(states, x, y);
        }

        // Ball
        self.add_ball(states, 160.0, 90.0); // center

        // Goals
        self.add_goal(states, 0.0, 60.0, 0);
        self.add_goal(states, 290.0, 60.0, 1);
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
    fn add_player(&mut self, states: &mut Vec<State>, x: f32, y: f32) {
        let player = State::new_player(x, y, states.len());
        if let Some(id) = player.get_player_id() {
            println!("Added player with ID: {} {}", id.0, id.1);
        } else {
            println!("Player has no ID!");
        }

        states.push(player);
        self.player_starts.push((x, y));
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
    pub fn set_game_settings(&mut self, player_count: u8, target_score: u8) {
        self.player_count = player_count;
        self.target_score = target_score;
    }

    pub fn get_ball_index(&self) -> Option<usize> {
        return self.ball_index;
    }
}
