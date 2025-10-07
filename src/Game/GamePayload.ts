import { State } from "./State";
import { ScoreManager } from "./ScoreManager";

export class GamePayload {
  constructor(public states: State[], public score_manager: ScoreManager) {}

  static from(obj: any): GamePayload {
    const states = obj.render_states.map((s: any, i: number) => {
      return new State(
        s.x,
        s.y,
        s.vx,
        s.vy,
        s.w,
        s.h,
        s.is_static,
        s.kind,
        s.player_id,
        s.team_id
      );
    });

    const score_manager = ScoreManager.from(obj.score_manager);

    return new GamePayload(states, score_manager);
  }
}
