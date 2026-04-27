use serde::Serialize;

use crate::game::state::{entityid::EntityId, enums::ActionState, Kind, Shape, State};

#[derive(Serialize, serde::Deserialize, Clone, Debug)]
pub struct RenderState {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub angle: f32,
    pub shape: Shape,

    pub action_state: ActionState,

    pub is_holding: bool,
    pub is_held: bool,
    pub is_static: bool,

    pub kind: Kind,
    pub player_id: EntityId,
    pub team_id: Option<u8>,
}
impl From<&State> for RenderState {
    fn from(state: &State) -> Self {
        Self {
            id: state.entity_id.0,
            x: state.physics_state.pos.x,
            y: state.physics_state.pos.y,
            vx: state.physics_state.vel.x,
            vy: state.physics_state.vel.y,
            angle: state.physics_state.angle,
            shape: state.physics_state.shape,

            action_state: state.action_state,

            is_holding: state.is_holding(),
            is_held: state.held_by.is_some(),
            is_static: state.is_static,

            kind: state.kind,
            player_id: state.entity_id,
            team_id: state.team_id,
        }
    }
}
