use serde::Serialize;

use crate::game::state::{entityid::EntityId, Kind, Shape, State};

#[derive(Serialize, serde::Deserialize, Clone, Debug)]
pub struct RenderState {
    pub x: f32,
    pub y: f32,
    pub shape: Shape,
    pub vx: f32,
    pub vy: f32,
    pub angle: f32,
    pub is_static: bool,
    pub is_trigger: bool,
    pub kind: Kind,
    pub player_id: EntityId,
    pub team_id: Option<u8>,
}
impl From<&State> for RenderState {
    fn from(state: &State) -> Self {
        Self {
            x: state.physics_state.pos.x,
            y: state.physics_state.pos.y,
            shape: state.physics_state.shape,
            angle: state.physics_state.angle,
            vx: state.physics_state.vel.x,
            vy: state.physics_state.vel.y,
            is_static: state.is_static,
            is_trigger: state.is_trigger,
            kind: state.kind,
            player_id: state.entity_id,
            team_id: state.team_id,
        }
    }
}
