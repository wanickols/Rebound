use crate::game::{
    frontend::fxevent::FxEvent,
    gamemanager::{GameManager, GamePhase},
    scoremanager::ScoreManager,
    state::{renderstate::RenderState, State},
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct GamePayload {
    render_states: Vec<RenderState>,
    score_manager: ScoreManager,
    game_phase: GamePhase,
    pub fx_events: Vec<FxEvent>,
}

impl GamePayload {
    pub fn new(
        states: &Vec<State>,
        score_manager: &ScoreManager,
        game_phase: &GamePhase,
        fx_events: Vec<FxEvent>,
    ) -> Self {
        let render_states: Vec<RenderState> = states.iter().map(RenderState::from).collect();

        Self {
            render_states,
            score_manager: score_manager.clone(),
            game_phase: game_phase.clone(),
            fx_events,
        }
    }
}

impl From<&GameManager> for GamePayload {
    fn from(gm: &GameManager) -> Self {
        GamePayload::new(
            &gm.world.entities,
            &gm.score_manager,
            &gm.phase,
            Vec::new(), // fx events will be drained separately
        )
    }
}
