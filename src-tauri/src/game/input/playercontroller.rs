use crate::game::state::{InputState, State};

#[derive(Clone)]
pub struct PlayerController {
    accel: f32,
    max_speed: f32,
    action_toggle: bool,
    mouse_pos: (f32, f32),
    pub input: InputState,
}

impl PlayerController {
    pub fn new(accel: f32, max_speed: f32) -> Self {
        Self {
            accel,
            max_speed,
            action_toggle: false,
            mouse_pos: (0.0, 0.0),
            input: InputState::new(),
        }
    }

    pub fn apply_input(
        &mut self,
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
    ) -> (f32, f32, f32, f32, Option<f32>) {
        // returns (new_x, new_y, new_vx, new_vy, new_angle)

        let mut ax = 0.0;
        let mut ay = 0.0;
        let mut angle = None;

        // Directional
        if self.input.up {
            ay -= 1.0;
        }
        if self.input.down {
            ay += 1.0;
        }
        if self.input.left {
            ax -= 1.0;
        }
        if self.input.right {
            ax += 1.0;
        }
        if self.input.action {
            self.handle_action();
        }

        if let Some(a) = self.handle_mouse((x, y)) {
            angle = Some(a);
        }

        // apply acceleration to velocity
        let mut vx = vx + ax * self.accel;
        let mut vy = vy + ay * self.accel;

        // clamp velocity
        vx = vx.clamp(-self.max_speed, self.max_speed);
        vy = vy.clamp(-self.max_speed, self.max_speed);

        (x, y, vx, vy, angle)
    }

    fn handle_action(&mut self) {
        self.action_toggle = !self.action_toggle;
    }

    fn handle_mouse(&mut self, pos: (f32, f32)) -> Option<f32> {
        if self.input.mouse_pos == self.mouse_pos || self.input.mouse_pos == (0.0, 0.0) {
            return None;
        }

        self.mouse_pos = self.input.mouse_pos;

        let delta = (self.mouse_pos.0 - pos.0, self.mouse_pos.1 - pos.1);
        let mut angle = delta.1.atan2(delta.0);
        if angle < 0.0 {
            angle += std::f32::consts::TAU; // TAU = 2π
        }
        println!("Angle (radians): {:.3}", angle);
        return Some(angle);
    }
}
