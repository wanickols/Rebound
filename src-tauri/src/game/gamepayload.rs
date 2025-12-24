use crate::game::{
    gamemanager::{GameManager, GamePhase},
    scoremanager::ScoreManager,
    state::{renderstate::RenderState, State},
};

#[derive(serde::Serialize, Clone, Debug)]
pub struct GamePayload {
    render_states: Vec<RenderState>,
    score_manager: ScoreManager,
    game_phase: GamePhase,
}

impl GamePayload {
    pub fn new(states: &Vec<State>, score_manager: &ScoreManager, game_phase: &GamePhase) -> Self {
        let render_states: Vec<RenderState> = states.iter().map(RenderState::from).collect();

        Self {
            render_states,
            score_manager: score_manager.clone(),
            game_phase: game_phase.clone(),
        }
    }
}

impl From<&GameManager> for GamePayload {
    fn from(gm: &GameManager) -> Self {
        GamePayload::new(&gm.world.entities, &gm.score_manager, &gm.phase)
    }
}
