import { GameRenderer } from "@/Game/Renderer/GameRenderer";
import { invoke } from "@tauri-apps/api/core";

export type GameAction =
  | "up"
  | "down"
  | "left"
  | "right"
  | "action"
  | "mouseMove";

type InputValue =
  | { Bool: boolean }
  | { Vec2: { x: number; y: number } }
  | { Float: number }
  | { None: Record<string, never> };

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
  private lastMouse = { x: 0, y: 0 };
  private lastSent = 0;
  private sendInterval = 1000 / 30; // 30Hz update rate
  private scale = 1.0;

  constructor() {
    window.addEventListener("keydown", (e) => this.handleKey(e.key, true));
    window.addEventListener("keyup", (e) => this.handleKey(e.key, false));
  }

  // Called from keydown/keyup
  handleKey(key: string, pressed: boolean) {
    const action = keyMap[key];
    if (!action) return;

    // Only trigger if state actually changes
    this.sendActionToServer(0, action, {
      Bool: pressed,
    });
  }

  handleMouseMove(x: number, y: number) {
    this.lastMouse = { x, y };
  }

  update(now: number) {
    if (now - this.lastSent >= this.sendInterval) {
      this.lastSent = now;
      this.sendMouseToServer();
    }
  }

  setScale(scale: number) {
    this.scale = scale;
  }

  sendMouseToServer() {
    this.sendActionToServer(0, "mouseMove", {
      Vec2: {
        x: this.lastMouse.x * this.scale,
        y: this.lastMouse.y * this.scale,
      },
    });
  }

  sendActionToServer(id: number, action: string, value: InputValue) {
    {
      invoke("input_event", { id, action, value }); // map key
    }
  }
}
