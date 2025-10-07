use crate::game::{
    gamemanager::GameManager,
    scoremanager::ScoreManager,
    state::{renderstate::RenderState, state::State},
};

#[derive(serde::Serialize, Clone)]
pub struct GamePayload {
    render_states: Vec<RenderState>,
    score_manager: ScoreManager,
}

impl GamePayload {
    pub fn new(states: &Vec<State>, score_manager: &ScoreManager) -> Self {
        let render_states: Vec<RenderState> = states.iter().map(RenderState::from).collect();

        Self {
            render_states,
            score_manager: score_manager.clone(),
        }
    }
}

impl From<&GameManager> for GamePayload {
    fn from(gm: &GameManager) -> Self {
        GamePayload::new(&gm.states, &gm.score_manager)
    }
}
