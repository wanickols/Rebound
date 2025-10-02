use crate::game::state::State;

pub struct Physics;

impl Physics {
    pub fn apply_input(state: &mut State) {
        // acceleration constants
        let accel = 50.0;
        let max_speed = 200.0;

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

        // Compute overlap
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

        // Determine collision axis
        if overlap_x < overlap_y {
            // Horizontal collision → flip vx
            a.vx = -a.vx * a.restitution;
            if !b.is_static {
                b.vx = -b.vx * b.restitution;
            }

            // Separate objects
            let push = overlap_x;
            if dx > 0.0 {
                a.x -= push;
                if !b.is_static {
                    b.x += push;
                }
            } else {
                a.x += push;
                if !b.is_static {
                    b.x -= push;
                }
            }
        } else {
            // Vertical collision → flip vy
            a.vy = -a.vy * a.restitution;
            if !b.is_static {
                b.vy = -b.vy * b.restitution;
            }

            // Separate objects
            let push = overlap_y;
            if dy > 0.0 {
                a.y -= push;
                if !b.is_static {
                    b.y += push;
                }
            } else {
                a.y += push;
                if !b.is_static {
                    b.y -= push;
                }
            }
        }
    }
}
