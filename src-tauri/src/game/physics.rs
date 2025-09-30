use crate::game::state::State;

pub struct Physics;

impl Physics{
    pub fn apply_input(state: &mut State, up: bool, down: bool, left: bool, right: bool){
        let mut dx = 0.0;
        let mut dy = 0.0;

        if up { dy -= 1.0; }
        if down { dy += 1.0; }
        if left { dx -= 1.0; }
        if right { dx += 1.0; }

        let speed = 2.0;

        state.x += dx * speed;
        state.y += dy * speed;
    }
}