export class GamePhase {
  constructor(
    public type: "Waiting" | "Playing" | "Countdown",
    public time_left: number | null = null
  ) {}

  static from(obj: any): GamePhase {
    // Unit variants come through as strings
    if (typeof obj === "string") {
      if (obj === "Waiting") return new GamePhase("Waiting");
      if (obj === "Playing") return new GamePhase("Playing");
      throw new Error(`Unknown GamePhase string: ${obj}`);
    }

    // Countdown comes through as an object
    if (obj.Countdown !== undefined) {
      return new GamePhase("Countdown", obj.Countdown.time_left);
    }

    throw new Error("Unknown GamePhase format: " + JSON.stringify(obj));
  }
}
