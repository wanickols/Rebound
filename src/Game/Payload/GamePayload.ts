import { AnimationState, State } from "../State";
import { GamePhase } from "./GamePhase";
import { ScoreManager } from "./ScoreManager";

export class GamePayload {
  constructor(
    public states: State[],
    public score_manager: ScoreManager,
    public phase: GamePhase,
  ) {}

  static from(obj: any): GamePayload {
    const states = obj.render_states.map((s: any, _i: number) => {
      return new State(
        s.x,
        s.y,
        s.vx,
        s.vy,
        s.shape,
        s.is_holding,
        AnimationState.Idle,
        s.angle,
        s.is_static,
        s.kind,
        s.player_id,
        s.team_id,
      );
    });

    const score_manager = ScoreManager.from(obj.score_manager);
    const phase = GamePhase.from(obj.game_phase);
    return new GamePayload(states, score_manager, phase);
  }
}
