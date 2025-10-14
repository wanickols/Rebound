use crate::game::{
    eventqueue::EventQueue,
    state::{PlayerId, State},
    util::Util,
};

pub struct Physics;

impl Physics {
    pub fn update(
        states: &mut Vec<State>,
        dt: f32,
        ball_holder: Option<PlayerId>,
        events: &mut EventQueue,
    ) {
        for i in 0..states.len() {
            if states[i].is_static || Physics::ball_handled(states, i, ball_holder) {
                continue;
            }

            let s = &mut states[i];

            if let Some(controller) = &mut s.player_controller {
                let (x, y, vx, vy, angle) = controller.apply_input(s.x, s.y, s.vx, s.vy, events);

                s.x = x;
                s.y = y;
                s.vx = vx;
                s.vy = vy;
                if angle.is_some() {
                    s.angle = angle.unwrap();
                }
            }

            s.apply_friction(dt);
            s.stop_if_tiny();

            let (next_x, next_y) = s.predict_position(dt);

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

    //Yeah prob not best
    pub fn ball_handled(states: &mut [State], i: usize, ball_holder: Option<PlayerId>) -> bool {
        if !states[i].is_enabled {
            if let Some(holder) = ball_holder {
                if holder.1 != i {
                    let (s, holder_state) = Util::two_mut(states, i, holder.1);
                    s.x = holder_state.x;
                    s.y = holder_state.y;
                } else {
                    let s = &mut states[i];
                    s.x = s.x;
                    s.y = s.y;
                }
            }
            return true; // ball handled
        }
        false // ball not handled
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
