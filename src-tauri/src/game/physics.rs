use crate::game::{eventqueue::EventQueue, state::State, util::Util};

pub struct Physics;

impl Physics {
    pub fn update(states: &mut Vec<State>, dt: f32, events: &mut EventQueue) {
        for i in 0..states.len() {
            if states[i].is_static || Physics::update_held_object(states, i, dt, events) {
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

            s.tick(dt, events);

            let (next_x, next_y) = s.predict_position(dt);

            for j in 0..states.len() {
                if i == j {
                    continue;
                }

                if states[j].held_by.is_some() {
                    if states[i].is_holding() {
                        continue;
                    }
                }

                if !states[i].check_collision_predicted(&states[j], next_x, next_y) {
                    continue;
                }

                State::handle_collision(states, i, j, events);
            }
            states[i].update_position(dt);
        }
    }

    pub fn apply_impulse(state: &mut State, angle: f32, power: f32) {
        state.vx += angle.cos() * power;
        state.vy += angle.sin() * power;
    }

    //Yeah prob not best
    pub fn update_held_object(
        states: &mut Vec<State>,
        i: usize,
        dt: f32,
        events: &mut EventQueue,
    ) -> bool {
        if let Some(holder_id) = states[i].held_by {
            let (held, holder) = Util::two_mut(states, i, holder_id.1);

            let max_distance = 40.0;
            let hold_distance = 20.0; // target in front of player
            let follow_strength = 0.2;
            let velocity_damping = 0.6;

            // compute target position in front of player
            let target_x = holder.x + holder.angle.cos() * hold_distance;
            let target_y = holder.y + holder.angle.sin() * hold_distance;

            // calculate distance to ball
            let dx = target_x - held.x;
            let dy = target_y - held.y;
            let distance_sq = dx * dx + dy * dy;

            // if too far, drop the ball
            if distance_sq > max_distance * max_distance {
                held.held_by = None; // drop
                holder.set_holding(false);
                return false;
            }

            // smooth follow
            held.x += dx * follow_strength;
            held.y += dy * follow_strength;

            // damp velocity
            held.vx *= velocity_damping;
            held.vy *= velocity_damping;

            let (next_x, next_y) = held.predict_position(dt);

            // Check triggers (goal zones, sensors, etc.)
            for j in 0..states.len() {
                if j != i && j != holder_id.1 {
                    if !states[i].check_collision_predicted(&states[j], next_x, next_y) {
                        continue;
                    }

                    State::handle_pure_trigger(states, i, j, events);
                }
            }

            return true;
        }
        false
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
