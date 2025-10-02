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

        if up {
            ay -= 1.0;
        }
        if down {
            ay += 1.0;
        }
        if left {
            ax -= 1.0;
        }
        if right {
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

        // now you can safely use a and b mutably
        if b.is_static {
            if a.vx != 0.0 {
                a.vx = -a.vx * a.restitution;
            }
            if a.vy != 0.0 {
                a.vy = -a.vy * a.restitution;
            }
        } else {
            std::mem::swap(&mut a.vx, &mut b.vx);
            std::mem::swap(&mut a.vy, &mut b.vy);
        }
    }
}
