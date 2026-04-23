import { AnimationState, State } from "../State";
import { FxEvent } from "./FxEvent";
import { GamePhase } from "./GamePhase";
import { ScoreManager } from "./ScoreManager";

export class GamePayload {
  constructor(
    public states: State[],
    public score_manager: ScoreManager,
    public phase: GamePhase,
    public fx_events: FxEvent[],
  ) {}

  static from(obj: any): GamePayload {
    const states = obj.render_states.map((s: any, _i: number) => {
      return new State(
        s.id,
        s.x,
        s.y,
        s.vx,
        s.vy,
        s.shape,
        s.is_holding,
        s.is_held,
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
    const fx_events = obj.fx_events as FxEvent[];
    return new GamePayload(states, score_manager, phase, fx_events);
  }
}
