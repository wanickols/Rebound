use serde::Serialize;

use crate::game::state::{entityid::EntityId, Kind, Shape, State};

#[derive(Serialize, Clone, Debug)]
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

impl RenderState {
    pub fn new(
        x: f32,
        y: f32,
        shape: Shape,
        vx: f32,
        vy: f32,
        angle: f32,
        is_static: bool,
        is_trigger: bool,
        kind: Kind,
        player_id: EntityId,
        team_id: Option<u8>,
    ) -> Self {
        Self {
            x,
            y,
            shape,
            vx,
            vy,
            angle,
            is_static,
            is_trigger,
            kind,
            player_id,
            team_id,
        }
    }
}

impl From<&State> for RenderState {
    fn from(state: &State) -> Self {
        Self {
            x: state.x,
            y: state.y,
            shape: state.shape,
            angle: state.angle,
            vx: state.vx,
            vy: state.vy,
            is_static: state.is_static,
            is_trigger: state.is_trigger,
            kind: state.kind,
            player_id: state.entity_id,
            team_id: state.team_id,
        }
    }
}
