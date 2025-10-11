use crate::game::{
    eventqueue::EventQueue,
    state::state::{Shape, State},
};

pub struct Physics;

impl Physics {
    pub fn apply_input(state: &mut State) {
        // acceleration constants
        let accel = 50.0;
        let max_speed = 400.0;

        // input direction
        let mut ax = 0.0;
        let mut ay = 0.0;

        let input = &state
            .input
            .as_mut()
            .expect("Applied Input without InputState");

        if input.up {
            ay -= 1.0;
        }
        if input.down {
            ay += 1.0;
        }
        if input.left {
            ax -= 1.0;
        }
        if input.right {
            ax += 1.0;
        }

        // apply acceleration to velocity
        state.vx += ax * accel;
        state.vy += ay * accel;

        // clamp velocity
        state.vx = state.vx.clamp(-max_speed, max_speed);
        state.vy = state.vy.clamp(-max_speed, max_speed);
    }

    pub fn update(states: &mut Vec<State>, dt: f32, events: &mut EventQueue) {
        for i in 0..states.len() {
            if states[i].is_static {
                continue;
            }

            if (states[i].input.is_some()) {
                Physics::apply_input(&mut states[i]);
            }

            states[i].apply_friction(dt);
            states[i].stop_if_tiny();

            let (next_x, next_y) = states[i].predict_position(dt);

            for j in 0..states.len() {
                if i == j {
                    continue;
                }

                if !states[i].check_collision_predicted(&states[j], next_x, next_y) {
                    continue;
                }

                State::handle_collision(states, i, j, events);
            }
            states[i].update_position(dt);
        }
    }

    pub fn resolve_pair(a: &mut State, b: &mut State, nx: f32, ny: f32, overlap: f32) {
        // --- IMPULSE RESPONSE ---
        let rvx = b.vx - a.vx;
        let rvy = b.vy - a.vy;

        let vel_along_normal = rvx * nx + rvy * ny;
        if vel_along_normal > 0.0 {
            return; // already separating
        }

        let e = a.restitution.min(b.restitution); // bounciness
        let inv_mass_a = if a.is_static { 0.0 } else { 1.0 / a.mass };
        let inv_mass_b = if b.is_static { 0.0 } else { 1.0 / b.mass };

        let j = -(1.0 + e) * vel_along_normal / (inv_mass_a + inv_mass_b);

        let impulse_x = j * nx;
        let impulse_y = j * ny;

        if !a.is_static {
            a.vx -= impulse_x * inv_mass_a;
            a.vy -= impulse_y * inv_mass_a;
        }
        if !b.is_static {
            b.vx += impulse_x * inv_mass_b;
            b.vy += impulse_y * inv_mass_b;
        }

        // --- POSITION CORRECTION ---
        let percent = 0.8; // tweak: how aggressively to separate
        let correction = overlap / (inv_mass_a + inv_mass_b) * percent;
        if !a.is_static {
            a.x -= correction * nx * inv_mass_a;
            a.y -= correction * ny * inv_mass_a;
        }
        if !b.is_static {
            b.x += correction * nx * inv_mass_b;
            b.y += correction * ny * inv_mass_b;
        }
    }
}
