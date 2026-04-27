import { Team } from "./Team";

export class ScoreManager {
  constructor(public teams: Team[]) {}

  static from(obj: any): ScoreManager {
    return new ScoreManager(
      obj.teams.map((t: any) => new Team(t.id, t.name, t.color, t.score))
    );
  }
}
