use crate::game::eventqueue::{EventQueue, GameEvent};
use crate::game::state::PlayerId;
use crate::game::state::{InputState, State};

#[derive(Clone)]
pub struct PlayerController {
    accel: f32,
    max_speed: f32,
    action_toggle: bool,
    mouse_pos: (f32, f32),
    prev_action: bool,
    pub is_holding: bool,
    pub player_id: PlayerId,
    pub input: InputState,
}

impl PlayerController {
    pub fn new(accel: f32, max_speed: f32, index: usize) -> Self {
        Self {
            accel,
            max_speed,
            action_toggle: false,
            is_holding: false,
            prev_action: false,
            mouse_pos: (0.0, 0.0),
            input: InputState::new(),
            player_id: PlayerId::new(index),
        }
    }

    pub fn apply_input(
        &mut self,
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        events: &mut EventQueue,
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

        if let Some(a) = self.handle_mouse((x, y)) {
            angle = Some(a);
        }

        if self.input.action {
            if (self.input.action != self.prev_action) {
                self.prev_action = self.input.action;

                self.handle_action(events);
            }
        } else {
            self.prev_action = false;
        }

        // apply acceleration to velocity
        let mut vx = vx + ax * self.accel;
        let mut vy = vy + ay * self.accel;

        // clamp velocity
        vx = vx.clamp(-self.max_speed, self.max_speed);
        vy = vy.clamp(-self.max_speed, self.max_speed);

        (x, y, vx, vy, angle)
    }

    fn handle_action(&mut self, events: &mut EventQueue) {
        self.action_toggle = !self.action_toggle;
        if self.action_toggle {
            events.push(GameEvent::TryGrab {
                player_id: self.player_id,
            });
        } else {
            events.push(GameEvent::Shoot {
                player_id: self.player_id,
            });
        }
    }

    fn try_grab(&mut self, x: f32, y: f32, radius: f32, angle: f32, ball: &mut State) {
        let s = State::new_hitcircle(x, y, radius, angle);
        if let Some((nx, ny, overlap)) = State::find_overlap(&s, ball) {
            ball.set_enable(false);
            ball.x = x;
            ball.y = y;
        }
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
