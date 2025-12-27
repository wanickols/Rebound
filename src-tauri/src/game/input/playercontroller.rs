use crate::game::eventqueue::{EventQueue, GameEvent};
use crate::game::input::inputframe::Vec2;
use crate::game::input::InputFrame;
use crate::game::state::entityid::EntityId;

const PLACE_COOLDOWN_TICKS: u16 = 10;

#[derive(Clone)]
pub struct PlayerController {
    accel: f32,
    last_angle: f32,
    max_speed: f32,
    action_toggle: bool,
    prev_action: bool,
    curr_brick_count: u8,
    max_bricks: u8,
    place_cooldown: u16,
    pub is_holding: bool,
    pub player_id: EntityId,
    pub input: InputFrame,
}

impl PlayerController {
    pub fn new(accel: f32, max_speed: f32, player_id: EntityId) -> Self {
        Self {
            accel,
            max_speed,
            action_toggle: false,
            is_holding: false,
            prev_action: false,
            curr_brick_count: 0,
            last_angle: 0.0,
            place_cooldown: 0,
            max_bricks: 3,
            input: InputFrame::new(),
            player_id: player_id,
        }
    }

    // returns (new_x, new_y, new_vx, new_vy, new_angle)
    pub fn apply_input(
        &mut self,
        pos: Vec2,
        vel: Vec2,
        events: &mut EventQueue,
    ) -> (f32, f32, f32, f32, Option<f32>) {
        //handle movement
        let delta = self.input.move_axis;
        let mut angle = self.handle_look(self.input.look);

        //If look does nothing, see if aiming is a thing
        if angle.is_some() {
            self.last_angle = angle.unwrap();
        } else {
            angle = Some(self.last_angle);
        }

        //handle actions
        if self.input.buttons.grab {
            if self.input.buttons.grab != self.prev_action {
                self.prev_action = self.input.buttons.grab;

                self.handle_action(events);
            }
        } else {
            self.prev_action = false;
        }

        //Handle Brick Placement
        if self.input.buttons.place {
            self.handle_brick_placement(events, pos, angle.expect("No angle bro"));
        }

        // apply acceleration to velocity
        let mut vx = vel.x + delta.x * self.accel;
        let mut vy = vel.y + delta.y * self.accel;

        // clamp velocity
        vx = vx.clamp(-self.max_speed, self.max_speed);
        vy = vy.clamp(-self.max_speed, self.max_speed);

        (pos.x, pos.y, vx, vy, angle)
    }

    pub fn tick(&mut self, _dt: f32) {
        if self.place_cooldown > 0 {
            self.place_cooldown -= 1;
        }
    }

    pub fn add_brick(&mut self) {
        self.curr_brick_count += 1;
        self.on_place();
    }

    pub fn remove_brick(&mut self) {
        if self.curr_brick_count > 0 {
            self.curr_brick_count -= 1;
        }
    }

    pub fn reset_player(&mut self) {
        self.curr_brick_count = 0;
        self.place_cooldown = 0;
        self.is_holding = false;
        self.action_toggle = false;
        self.prev_action = false;
    }

    //Private
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

    fn handle_look(&mut self, dir: Vec2) -> Option<f32> {
        if dir.x != 0.0 || dir.y != 0.0 {
            let angle = dir.y.atan2(dir.x);
            // println!(
            //     "[LOOK] dir=({:.2}, {:.2}) -> angle={:.3} rad ({:.1}°)",
            //     dir.x,
            //     dir.y,
            //     angle,
            //     angle.to_degrees()
            // );
            return Some(angle);
        }
        None
    }

    fn handle_brick_placement(&mut self, events: &mut EventQueue, player_pos: Vec2, angle: f32) {
        if !self.can_place() {
            return;
        }

        let distance: f32 = 24.0;

        let dir_x = angle.cos();
        let dir_y = angle.sin();

        let brick_x = player_pos.x + dir_x * distance;
        let brick_y = player_pos.y + dir_y * distance;

        let brick_pos = (brick_x, brick_y);

        events.push(GameEvent::Place {
            player_id: self.player_id,
            pos: brick_pos,
        });
    }

    fn can_place(&self) -> bool {
        if self.curr_brick_count >= self.max_bricks {
            return false;
        }

        self.place_cooldown == 0
    }
    fn on_place(&mut self) {
        self.place_cooldown = PLACE_COOLDOWN_TICKS;
    }
}
