use serde::Serialize;

use crate::game::state::{
    playerid::PlayerId,
    state::{Kind, State},
};

#[derive(Serialize, Clone)]
pub struct RenderState {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub vx: f32,
    pub vy: f32,
    pub is_static: bool,
    pub is_trigger: bool,
    pub kind: Kind,
    pub player_id: Option<PlayerId>,
    pub team_id: u8,
}

impl RenderState {
    pub fn new(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        vx: f32,
        vy: f32,
        is_static: bool,
        is_trigger: bool,
        kind: Kind,
        player_id: Option<PlayerId>,
        team_id: u8,
    ) -> Self {
        Self {
            x,
            y,
            w,
            h,
            vx,
            vy,
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
            w: state.w,
            h: state.h,
            vx: state.vx,
            vy: state.vy,
            is_static: state.is_static,
            is_trigger: state.is_trigger,
            kind: state.kind,
            player_id: state.player_id,
            team_id: state.team_id,
        }
    }
}
