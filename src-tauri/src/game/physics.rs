use crate::game::state::State;

pub struct Physics;

impl Physics {
    pub fn apply_input(state: &mut State) {
        // acceleration constants
        let accel = 50.0;
        let max_speed = 400.0;

        // input direction
        let mut ax = 0.0;
        let mut ay = 0.0;

        let input = &state.input;

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

    pub fn update(states: &mut Vec<State>, dt: f32) {
        for i in 0..states.len() {
            if states[i].is_static {
                continue;
            }

            Physics::apply_input(&mut states[i]);

            states[i].apply_friction(dt);
            states[i].stop_if_tiny();

            let (next_x, next_y) = states[i].predict_position(dt);

            for j in 0..states.len() {
                if i == j {
                    continue;
                }

                if !Self::check_collision_predicted(&states[i], &states[j], next_x, next_y) {
                    continue;
                }

                Self::resolve_collision(states, i, j);
            }
            states[i].update_position(dt);
        }
    }

    fn check_collision_predicted(a: &State, b: &State, next_x: f32, next_y: f32) -> bool {
        let (ax1, ay1, ax2, ay2) = (next_x, next_y, next_x + a.w, next_y + a.h);
        let (bx1, by1, bx2, by2) = b.bounds();
        ax1 < bx2 && ax2 > bx1 && ay1 < by2 && ay2 > by1
    }

    fn resolve_collision(states: &mut Vec<State>, i: usize, j: usize) {
        let (a, b) = if i < j {
            let (left, right) = states.split_at_mut(j);
            (&mut left[i], &mut right[0])
        } else {
            let (left, right) = states.split_at_mut(i);
            (&mut right[0], &mut left[j])
        };

        // Compute centers
        let ax_center = a.x + a.w / 2.0;
        let ay_center = a.y + a.h / 2.0;
        let bx_center = b.x + b.w / 2.0;
        let by_center = b.y + b.h / 2.0;

        let dx = bx_center - ax_center;
        let dy = by_center - ay_center;

        let combined_half_width = (a.w + b.w) / 2.0;
        let combined_half_height = (a.h + b.h) / 2.0;

        let overlap_x = combined_half_width - dx.abs();
        let overlap_y = combined_half_height - dy.abs();

        if overlap_x > 0.0 && overlap_y > 0.0 {
            // Pick axis of minimum penetration
            let (nx, ny, overlap) = if overlap_x < overlap_y {
                (dx.signum(), 0.0, overlap_x)
            } else {
                (0.0, dy.signum(), overlap_y)
            };

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
}
