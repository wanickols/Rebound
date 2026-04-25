export type FxEvent =
  | { type: "BallWallHit"; intensity: number }
  | { type: "GoalScored"; team_id: number };

export function normalizeFxEvent(e: any): FxEvent {
  const key = Object.keys(e)[0];

  switch (key) {
    case "GoalScored":
      return { type: "GoalScored", ...e[key] };

    case "BallWallHit":
      return { type: "BallWallHit", ...e[key] };

    default:
      throw new Error("Unknown FxEvent: " + key);
  }
}
