use crate::game::state::{Kind, State};

pub struct SpawnManager {
    pub player_starts: Vec<(f32, f32)>,
    pub ball_start: Option<(f32, f32)>,
    pub ball_index: Option<usize>,
}

impl SpawnManager {
    pub fn new() -> Self {
        Self {
            player_starts: Vec::new(),
            ball_start: None,
            ball_index: None,
        }
    }

    pub fn add_player(&mut self, states: &mut Vec<State>, x: f32, y: f32) {
        let player = State::new_player(x, y, states.len());
        if let Some(id) = player.get_player_id() {
            println!("Added player with ID: {}", id.0);
        } else {
            println!("Player has no ID!");
        }

        states.push(player);
        self.player_starts.push((x, y));
    }

    pub fn add_ball(&mut self, states: &mut Vec<State>, x: f32, y: f32) {
        states.push(State::new_ball(x, y));
        self.ball_start = Some((x, y));
        self.ball_index = Some(states.len() - 1);
    }

    pub fn get_ball_mut<'a>(&self, states: &'a mut Vec<State>) -> Option<&'a mut State> {
        self.ball_index.map(move |i| &mut states[i])
    }

    pub fn get_ball_index(&self) -> Option<usize> {
        return self.ball_index;
    }

    pub fn spawn_states(&mut self, states: &mut Vec<State>) {
        self.add_player(states, 100.0, 50.0);
        self.add_ball(states, 100.0, 50.0);
        self.add_player(states, 200.0, 50.0);
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
}
