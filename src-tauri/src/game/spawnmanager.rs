use crate::game::state::entityid::EntityId;
use crate::game::state::{enums::Kind, State};
use crate::game::world::World;
pub const PLAYER_POSITIONS: [(f32, f32, f32); 8] = [
    (50.0, 50.0, 0.0),
    (270.0, 50.0, 3.142),
    (50.0, 130.0, 0.0),
    (270.0, 130.0, 3.142),
    (160.0, 50.0, 0.0),
    (160.0, 130.0, 3.142),
    (90.0, 90.0, 0.0),
    (230.0, 90.0, 3.142),
];

pub struct SpawnManager {
    ball_start: Option<(f32, f32)>,
    ball_id: Option<EntityId>,
    max_player_count: u8,
    pub width: f32,
    pub height: f32,
}

impl SpawnManager {
    //Constructor
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            ball_start: None,
            ball_id: None,
            max_player_count: 1,
            width,
            height,
        }
    }

    pub fn try_add_player(&mut self, world: &mut World) -> Option<EntityId> {
        let count = world.curr_player_count();
        if count >= self.max_player_count.into() {
            println!("Max players reached!");
            return None;
        }

        let (x, y, angle) = PLAYER_POSITIONS[count];
        let player_id = self.add_player(world, x, y, angle);
        Some(player_id)
    }

    pub fn remove_player(&mut self, world: &mut World, player_id: EntityId) {
        world.remove_player(player_id);
    }

    pub fn remove_all(&mut self, world: &mut World) {
        world.remove_all();
        self.max_player_count = 1;
        self.ball_id = None;
    }

    pub fn remove_non_player(&mut self, world: &mut World) {
        world.remove_all_non_players();
        self.ball_id = None;
    }

    ///Public Functions
    pub fn spawn_states(&mut self, world: &mut World) {
        //Borders
        self.create_borders(world);
        // Ball
        self.add_ball(world, 160.0, 90.0); // center
                                           // Goals
        self.add_goal(world, 0.0, 60.0, 0);
        self.add_goal(world, 290.0, 60.0, 1);
    }

    fn create_borders(&mut self, world: &mut World) {
        let thickness = 10.0; // wall thickness

        // Top wall
        world.add_entity(State::new_wall(0.0, -thickness, self.width, thickness));
        // Bottom wall
        world.add_entity(State::new_wall(0.0, self.height, self.width, thickness));

        // Left wall
        world.add_entity(State::new_wall(-thickness, 0.0, thickness, self.height));

        // Right wall
        world.add_entity(State::new_wall(self.width, 0.0, thickness, self.height));
    }

    pub fn reset_states(&self, world: &mut World) {
        let mut player_index = 0;
        for state in world.entities.iter_mut() {
            if state.is_static {
                continue;
            }

            match state.kind {
                Kind::Ball => {
                    if let Some((bx, by)) = self.ball_start {
                        state.physics_state.pos.x = bx;
                        state.physics_state.pos.y = by;
                        state.held_by = None;
                        state.physics_state.vel.x = 0.0;
                        state.physics_state.vel.y = 0.0;
                    }
                }
                Kind::Player => {
                    let (px, py, angle) = PLAYER_POSITIONS[player_index];
                    state.physics_state.pos.x = px;
                    state.physics_state.pos.y = py;
                    state.physics_state.angle = angle;
                    state.held_by = None;
                    state.physics_state.vel.x = 0.0;
                    state.physics_state.vel.y = 0.0;
                    state
                        .player_controller
                        .as_mut()
                        .unwrap()
                        .reset_player(angle);
                    player_index += 1;
                }
                Kind::Brick => {
                    state.time_to_live = Some(0);
                }
                _ => {}
            }
        }
    }

    ///Private
    //Add Functions:
    pub fn add_player(&mut self, world: &mut World, x: f32, y: f32, angle: f32) -> EntityId {
        let player = State::new_player(x, y, angle);

        let id = player.entity_id;
        println!("Added player with ID: {}", id.0);
        world.add_player(player);
        id
    }

    pub fn add_brick(&mut self, world: &mut World, pos: (f32, f32), player_id: EntityId) {
        let brick = State::new_brick(pos.0, pos.1, 8.0, player_id);
        world.add_entity(brick);
    }

    pub fn remove_brick(&mut self, world: &mut World, entityid: EntityId) {
        world.remove_entity(entityid);
    }

    fn add_ball(&mut self, world: &mut World, x: f32, y: f32) {
        let ball = State::new_ball(x, y);
        self.ball_id = Some(ball.entity_id);
        world.add_entity(ball);

        self.ball_start = Some((x, y));
    }

    fn add_goal(&mut self, world: &mut World, x: f32, y: f32, team_id: u8) {
        world.add_entity(State::new_goal(x, y, 30.0, 60.0, team_id));
    }

    ///Getters and Setters
    pub fn set_player_count(&mut self, player_count: u8) {
        self.max_player_count = player_count;
    }

    pub fn get_ball_id(&self) -> Option<EntityId> {
        return self.ball_id;
    }
}
