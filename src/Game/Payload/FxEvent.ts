export type FxEvent =
  | { type: "BallWallHit"; intensity: number }
  | { type: "GoalScored"; team_id: number };
