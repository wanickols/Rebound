use crate::game::{input::inputframe::Vec2, state::Shape};

#[derive(Clone)]
pub struct PhysicsState {
    pub pos: Vec2,
    pub vel: Vec2,
    pub angle: f32,
    pub shape: Shape,
    pub mass: f32,
    pub is_static: bool,
    pub friction: f32,
    pub restitution: f32,
}

impl PhysicsState {
    pub fn new() -> Self {
        Self {
            pos: Vec2 { x: 0.0, y: 0.0 },
            vel: Vec2 { x: 0.0, y: 0.0 },
            angle: 0.0,
            shape: Shape::Rectangle { w: 1.0, h: 1.0 },
            mass: 1.0,
            is_static: false,
            friction: 0.5,
            restitution: 0.5,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.apply_friction(dt);
        self.stop_if_tiny();
    }

    //Positions
    pub fn update_position(&mut self, dt: f32) {
        self.pos.x += self.vel.x * dt;
        self.pos.y += self.vel.y * dt;
    }

    pub fn predict_position(&self, dt: f32) -> (f32, f32) {
        (self.pos.x + self.vel.x * dt, self.pos.y + self.vel.y * dt)
    }

    //Collisions
    //returns dx, dy, and overlap x and y
    pub fn find_overlap(a: &PhysicsState, b: &PhysicsState) -> Option<(f32, f32, f32)> {
        // Try to cast both shapes to circles
        let ar = if let Shape::Circle { radius } = a.shape {
            radius
        } else {
            return None;
        };
        let br = if let Shape::Circle { radius } = b.shape {
            radius
        } else {
            return None;
        };

        let dx = b.pos.x - a.pos.x;
        let dy = b.pos.y - a.pos.y;
        let dist = (dx * dx + dy * dy).sqrt();
        let combined_r = ar + br;

        if dist < combined_r {
            let nx = if dist == 0.0 { 1.0 } else { dx / dist };
            let ny = if dist == 0.0 { 0.0 } else { dy / dist };
            let overlap = combined_r - dist;
            Some((nx, ny, overlap))
        } else {
            None
        }
    }

    pub fn check_collision_predicted(
        &self,
        other: &PhysicsState,
        next_x: f32,
        next_y: f32,
    ) -> bool {
        match &self.shape {
            // Rectangle vs Rectangle
            Shape::Rectangle { w, h } => {
                if let Shape::Rectangle { .. } = &other.shape {
                    let (ax1, ay1, ax2, ay2) = (next_x, next_y, next_x + w, next_y + h);
                    let (bx1, by1, bx2, by2) = other.bounds();
                    return ax1 < bx2 && ax2 > bx1 && ay1 < by2 && ay2 > by1;
                }
                // Rectangle vs Circle -> flip to Circle vs Rectangle
                if let Shape::Circle { .. } = &other.shape {
                    return other.check_collision_predicted(self, other.pos.x, other.pos.y);
                }
                false
            }

            // Circle vs Circle
            Shape::Circle { radius } => {
                if let Shape::Circle { radius: br } = &other.shape {
                    let dx = (next_x) - other.pos.x;
                    let dy = (next_y) - other.pos.y;
                    return dx * dx + dy * dy < (radius + br).powi(2);
                }

                // Circle vs Rectangle
                if let Shape::Rectangle { w, h } = &other.shape {
                    let closest_x = next_x.clamp(other.pos.x, other.pos.x + w);
                    let closest_y = next_y.clamp(other.pos.y, other.pos.y + h);

                    let dx = next_x - closest_x;
                    let dy = next_y - closest_y;

                    return dx * dx + dy * dy < radius.powi(2);
                }

                false
            }

            _ => false,
        }
    }

    //Physics helper Functions
    fn bounds(&self) -> (f32, f32, f32, f32) {
        match &self.shape {
            Shape::Rectangle { w, h } => (self.pos.x, self.pos.y, self.pos.x + w, self.pos.y + h),
            Shape::Circle { radius } => {
                // if you want to approximate the circle with a bounding box
                let d = radius * 2.0;
                (self.pos.x, self.pos.y, self.pos.x + d, self.pos.y + d)
            }
        }
    }

    //tick helpers
    fn apply_friction(&mut self, dt: f32) {
        self.vel.x *= 1.0 - self.friction * dt;
        self.vel.y *= 1.0 - self.friction * dt;
    }

    fn stop_if_tiny(&mut self) {
        if self.vel.x.abs() < 0.01 {
            self.vel.x = 0.0;
        }
        if self.vel.y.abs() < 0.01 {
            self.vel.y = 0.0;
        }
    }
}
