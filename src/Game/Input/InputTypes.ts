export type GameAction = "move" | "action" | "aim" | "look" | "pause" | "place";

export type InputValue =
  | { Bool: boolean }
  | { Vec2: { x: number; y: number } }
  | { Float: number }
  | { None: Record<string, never> };
