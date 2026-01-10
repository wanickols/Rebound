use crate::game::{
    eventqueue::EventQueue,
    input::inputframe::Vec2,
    state::{physicsstate::PhysicsState, State},
    world::World,
};

pub struct Physics;

impl Physics {
    pub fn update(world: &mut World, dt: f32, events: &mut EventQueue) {
        for i in 0..world.entities.len() {
            if world.entities[i].is_static || Physics::update_held_object(world, i, dt, events) {
                continue;
            }

            let s = &mut world.entities[i];

            let ps = &mut s.physics_state;

            //Apply player input
            if let Some(controller) = &mut s.player_controller {
                let (x, y, vx, vy, angle) = controller.apply_input(
                    Vec2 {
                        x: ps.pos.x,
                        y: ps.pos.y,
                    },
                    Vec2 {
                        x: ps.vel.x,
                        y: ps.vel.y,
                    },
                    events,
                    dt,
                );

                ps.pos.x = x;
                ps.pos.y = y;
                ps.vel.x = vx;
                ps.vel.y = vy;
                if angle.is_some() {
                    ps.angle = angle.unwrap();
                }
            }

            let (next_x, next_y) = ps.predict_position(dt);
            s.tick(dt, events);

            for j in 0..world.entities.len() {
                if i == j {
                    continue;
                }

                if world.entities[j].held_by.is_some() {
                    if world.entities[i].is_holding() {
                        continue;
                    }
                }

                if !world.entities[i].physics_state.check_collision_predicted(
                    &world.entities[j].physics_state,
                    next_x,
                    next_y,
                ) {
                    continue;
                }

                State::handle_collision(&mut world.entities, i, j, events);
            }
            world.entities[i].physics_state.update_position(dt);
        }
    }

    pub fn apply_impulse(state: &mut PhysicsState, angle: f32, power: f32) {
        state.vel.x += angle.cos() * power;
        state.vel.y += angle.sin() * power;
    }

    //Yeah prob not best
    pub fn update_held_object(
        world: &mut World,
        i: usize,
        dt: f32,
        events: &mut EventQueue,
    ) -> bool {
        if let Some(holder_id) = world.entities[i].held_by {
            if let Some((held, holder)) =
                world.grab_two_entities(world.entities[i].entity_id, holder_id)
            {
                let max_distance = 40.0;
                let hold_distance = 16.0; // target in front of player
                let follow_strength = 0.2;
                let velocity_damping = 0.6;

                // compute target position in front of player
                let target_x =
                    holder.physics_state.pos.x + holder.physics_state.angle.cos() * hold_distance;
                let target_y =
                    holder.physics_state.pos.y + holder.physics_state.angle.sin() * hold_distance;

                // calculate distance to ball
                let dx = target_x - held.physics_state.pos.x;
                let dy = target_y - held.physics_state.pos.y;
                let distance_sq = dx * dx + dy * dy;

                // if too far, drop the ball
                if distance_sq > max_distance * max_distance {
                    held.held_by = None; // drop
                    holder.set_holding(false);
                    return false;
                }

                // smooth follow
                held.physics_state.pos.x += dx * follow_strength;
                held.physics_state.pos.y += dy * follow_strength;

                // damp velocity
                held.physics_state.vel.x *= velocity_damping;
                held.physics_state.vel.y *= velocity_damping;

                let (next_x, next_y) = held.physics_state.predict_position(dt);

                // Check triggers (goal zones, sensors, etc.)
                for j in 0..world.entities.len() {
                    if j != i && world.entities[j].entity_id != holder_id {
                        if !world.entities[i].physics_state.check_collision_predicted(
                            &world.entities[j].physics_state,
                            next_x,
                            next_y,
                        ) {
                            continue;
                        }

                        State::handle_pure_trigger(&mut world.entities, i, j, events);
                    }
                }

                return true;
            }
        }
        false
    }

    /// Normalize an angle in radians to the range -π..π
    pub fn normalize_angle(angle: f32) -> f32 {
        let mut a = angle;
        while a > std::f32::consts::PI {
            a -= 2.0 * std::f32::consts::PI;
        }
        while a < -std::f32::consts::PI {
            a += 2.0 * std::f32::consts::PI;
        }
        a
    }

    pub fn resolve_pair(
        a: &mut PhysicsState,
        b: &mut PhysicsState,
        nx: f32,
        ny: f32,
        overlap: f32,
    ) {
        // --- IMPULSE RESPONSE ---
        let rvx = b.vel.x - a.vel.x;
        let rvy = b.vel.y - a.vel.y;

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
            a.vel.x -= impulse_x * inv_mass_a;
            a.vel.y -= impulse_y * inv_mass_a;
        }
        if !b.is_static {
            b.vel.x += impulse_x * inv_mass_b;
            b.vel.y += impulse_y * inv_mass_b;
        }

        // --- POSITION CORRECTION ---
        let percent = 0.8; // tweak: how aggressively to separate
        let correction = overlap / (inv_mass_a + inv_mass_b) * percent;
        if !a.is_static {
            a.pos.x -= correction * nx * inv_mass_a;
            a.pos.y -= correction * ny * inv_mass_a;
        }
        if !b.is_static {
            b.pos.x += correction * nx * inv_mass_b;
            b.pos.y += correction * ny * inv_mass_b;
        }
    }
}
