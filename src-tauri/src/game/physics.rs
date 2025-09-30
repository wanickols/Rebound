use crate::game::state::State;

pub struct Physics;

impl Physics {
    pub fn apply_input(state: &mut State, up: bool, down: bool, left: bool, right: bool) {
        // acceleration constants
        let accel = 40.0;
        let max_speed = 400.0;

        // input direction
        let mut ax = 0.0;
        let mut ay = 0.0;

        if up { ay -= 1.0; }
        if down { ay += 1.0; }
        if left { ax -= 1.0; }
        if right { ax += 1.0; }

        // apply acceleration to velocity
        state.vx += ax * accel;
        state.vy += ay * accel;

        // clamp velocity
        state.vx = state.vx.clamp(-max_speed, max_speed);
        state.vy = state.vy.clamp(-max_speed, max_speed);
    }

    pub fn update(state: &mut State, dt: f32) {
        // friction / deceleration
        let friction = 0.99; // tweak until it feels nice
        state.vx *= friction;
        state.vy *= friction;

        // stop tiny velocities
        if state.vx.abs() < 0.01 { state.vx = 0.0; }
        if state.vy.abs() < 0.01 { state.vy = 0.0; }

        // apply velocity to position
        state.x += state.vx * dt;
        state.y += state.vy * dt;
    }
}
