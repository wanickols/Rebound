use crate::game::eventqueue::{EventQueue, GameEvent};
use crate::game::state::entityid::EntityId;
use crate::game::state::{InputState, State};

#[derive(Clone)]
pub struct PlayerController {
    accel: f32,
    last_angle: f32,
    max_speed: f32,
    action_toggle: bool,
    mouse_pos: (f32, f32),
    prev_action: bool,
    curr_brick_count: u8,
    max_bricks: u8,
    pub is_holding: bool,
    pub player_id: EntityId,
    pub input: InputState,
}

impl PlayerController {
    pub fn new(accel: f32, max_speed: f32) -> Self {
        Self {
            accel,
            max_speed,
            action_toggle: false,
            is_holding: false,
            prev_action: false,
            curr_brick_count: 0,
            last_angle: 0.0,
            max_bricks: 3,
            mouse_pos: (0.0, 0.0),
            input: InputState::new(),
            player_id: EntityId::new(),
        }
    }

    // returns (new_x, new_y, new_vx, new_vy, new_angle)
    pub fn apply_input(
        &mut self,
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        events: &mut EventQueue,
    ) -> (f32, f32, f32, f32, Option<f32>) {
        //handle movement
        let ax = self.input.move_axis.0;
        let ay = self.input.move_axis.1;
        let mut angle = self.handle_look(self.input.look_pos);

        //If look does nothing, see if aiming is a thing
        if angle.is_none() {
            angle = self.handle_aim((x, y));
            if angle.is_none() {
                angle = Some(0.0);
            }
            self.last_angle = angle.unwrap();
        }

        //handle actions
        if self.input.action {
            if self.input.action != self.prev_action {
                self.prev_action = self.input.action;

                self.handle_action(events);
            }
        } else {
            self.prev_action = false;
        }

        //Handle Brick Placement
        if self.input.place {
            self.handle_brick_placement(events, (x, y), angle.expect("No angle bro"));
        }

        // apply acceleration to velocity
        let mut vx = vx + ax * self.accel;
        let mut vy = vy + ay * self.accel;

        // clamp velocity
        vx = vx.clamp(-self.max_speed, self.max_speed);
        vy = vy.clamp(-self.max_speed, self.max_speed);

        (x, y, vx, vy, angle)
    }

    pub fn add_brick(&mut self) {
        self.curr_brick_count += 1;
    }

    pub fn remove_brick(&mut self) {
        if self.curr_brick_count > 0 {
            self.curr_brick_count -= 1;
        }
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

    fn handle_aim(&mut self, player_pos: (f32, f32)) -> Option<f32> {
        let mouse_pos = self.input.mouse_pos;
        if mouse_pos == self.mouse_pos {
            return Some(self.last_angle);
        }

        let dx = mouse_pos.0 - player_pos.0;
        let dy = mouse_pos.1 - player_pos.1;
        let angle = dy.atan2(dx);

        self.mouse_pos = mouse_pos;
        println!(
            "[AIM] player=({:.2}, {:.2}) mouse=({:.2}, {:.2}) -> angle={:.3} rad ({:.1}°)",
            player_pos.0,
            player_pos.1,
            mouse_pos.0,
            mouse_pos.1,
            angle,
            angle.to_degrees()
        );

        Some(angle)
    }

    fn handle_look(&mut self, dir: (f32, f32)) -> Option<f32> {
        if dir.0 != 0.0 || dir.1 != 0.0 {
            let angle = dir.1.atan2(dir.0);
            println!(
                "[LOOK] dir=({:.2}, {:.2}) -> angle={:.3} rad ({:.1}°)",
                dir.0,
                dir.1,
                angle,
                angle.to_degrees()
            );
            return Some(angle);
        }
        None
    }

    fn handle_brick_placement(
        &mut self,
        events: &mut EventQueue,
        player_pos: (f32, f32),
        angle: f32,
    ) {
        if self.curr_brick_count >= self.max_bricks {
            return;
        }

        let distance: f32 = 24.0;

        let dir_x = angle.cos();
        let dir_y = angle.sin();

        let brick_x = player_pos.0 + dir_x * distance;
        let brick_y = player_pos.1 + dir_y * distance;

        let brick_pos = (brick_x, brick_y);

        events.push(GameEvent::Place {
            player_id: self.player_id,
            pos: brick_pos,
        });
    }
}
