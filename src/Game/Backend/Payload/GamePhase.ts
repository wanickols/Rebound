export class GamePhase {
  constructor(
    public type: "Waiting" | "Playing" | "Countdown" | "GameOver",
    public time_left: number | null = null
  ) {}

  static from(obj: any): GamePhase {
    // Unit variants come through as strings
    if (typeof obj === "string") {
      if (obj === "Waiting") return new GamePhase("Waiting");
      if (obj === "Playing") return new GamePhase("Playing");
      if (obj === "GameOver") return new GamePhase("GameOver");
      throw new Error(`Unknown GamePhase string: ${obj}`);
    }

    // Countdown comes through as an object
    if (obj.Countdown !== undefined) {
      return new GamePhase("Countdown", obj.Countdown.time_left);
    }

    throw new Error("Unknown GamePhase format: " + JSON.stringify(obj));
  }
}
