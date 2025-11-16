// InputManager.ts
import { ReactivePlayerManager } from "@/Game/Input/PlayerManager";
import { invoke } from "@tauri-apps/api/core";
import { InputValue } from "./InputTypes";
import { ControllerManager } from "./ControllerManager";
import { KeyboardManager } from "./KeyboardManager";

export class InputManager {
  playerManager: ReactivePlayerManager;
  controllerManager: ControllerManager;
  keyboardManager: KeyboardManager;

  constructor() {
    this.playerManager = new ReactivePlayerManager();
    this.controllerManager = new ControllerManager(this, this.playerManager);
    this.keyboardManager = new KeyboardManager(this, this.playerManager);
  }

  //Movement
  lastMove: Record<number, { x: number; y: number }> = {};

  public updateMove(id: [number, number], index: number, x: number, y: number) {
    // optional deadzone for analog sticks
    const deadzone = 0.2;
    const mx = Math.abs(x) < deadzone ? 0 : x;
    const my = Math.abs(y) < deadzone ? 0 : y;

    const last = this.lastMove[index] || { x: 0, y: 0 };
    if (last.x !== mx || last.y !== my) {
      this.lastMove[index] = { x: mx, y: my };
      this.sendActionToServer(id, "move", { Vec2: { x: mx, y: my } });
    }
  }

  setScale(scale: number) {
    this.keyboardManager.scale = scale;
  }

  //Server Side
  public sendActionToServer(
    id: [number, number],
    action: string,
    value: InputValue
  ) {
    console.log("sending stuff" + id + action + value);
    invoke("input_event", { id, action, value }).catch((err) => {
      console.warn("Failed to send input event:", err);
    });
  }
}
