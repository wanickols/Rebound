import { invoke } from "@tauri-apps/api/core";

export type GameAction = "up" | "down" | "left" | "right" | "action";

// Map keys / buttons to actions
const keyMap: Record<string, GameAction> = {
  w: "up",
  W: "up",
  ArrowUp: "up",
  s: "down",
  ArrowDown: "down",
  a: "left",
  ArrowLeft: "left",
  d: "right",
  ArrowRight: "right",
  " ": "action",
};

// Central input manager
export class InputManager {
  constructor() {
    window.addEventListener("keydown", (e) => this.handleKey(e.key, true));
    window.addEventListener("keyup", (e) => this.handleKey(e.key, false));
  }

  // Called from keydown/keyup
  handleKey(key: string, pressed: boolean) {
    const action = keyMap[key];
    if (!action) return;

    // Only trigger if state actually changes
    this.sendActionToServer(0, action, pressed);
  }

  private sendActionToServer(
    id: number, // TS camelCase
    action: GameAction,
    pressed: boolean
  ) {
    console.log("Sending action:", action, pressed);
    invoke("input_event", { id, action, pressed }); // map key
  }
}
