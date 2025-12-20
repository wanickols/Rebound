import { invoke } from "@tauri-apps/api/core";
import { InputValue } from "./InputTypes";
import { Player, playerManager } from "./PlayerManager";
import { GamepadData } from "./ControllerManager";
export class InputManager {
  //Movement
  lastMove: Record<number, { x: number; y: number }> = {};
  km: any;

  constructor() {}

  updateMove(id: [number, number], index: number, x: number, y: number) {
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

  handleMouseMove(x: number, y: number) {
    if (!this.km.mouseDown) return;

    const scaledX = x * this.km.scale;
    const scaledY = y * this.km.scale;

    // Only send if the position changed
    if (scaledX === this.km.mousePos.x && scaledY === this.km.mousePos.y)
      return;

    this.km.mousePos.x = scaledX;
    this.km.mousePos.y = scaledY;

    const kb = playerManager.getPlayerByController(-1);
    if (!kb) return;
    this.sendActionToServer(kb.id, "aim", {
      Vec2: { x: scaledX, y: scaledY },
    });
  }

  handleKeyboardEvent(player: Player) {
    const keys = player.keys;
    // compute move vector from WASD
    const x = (keys["d"] ? 1 : 0) - (keys["a"] ? 1 : 0);
    const y = (keys["s"] ? 1 : 0) - (keys["w"] ? 1 : 0);

    this.updateMove(player.id, -1, x, y); // -1 = keyboard index

    if (keys[" "]) {
      this.sendActionToServer(player.id, "action", { Bool: true });
    } else if (keys["escape"]) {
      this.sendActionToServer(player.id, "pause", { Bool: true });
    }
  }

  handleGamepadEvent(pad: GamepadData) {
    const controller = playerManager.getPlayerByController(pad.index);
    if (!controller) return;

    // Axes
    const x = pad.axes[0];
    const y = pad.axes[1];

    this.updateMove(controller.id, pad.index, x, y);

    // Action
    var pressed = pad.buttons[0];
    this.sendActionToServer(controller.id, "action", { Bool: pressed });

    //Place
    pressed = pad.buttons[1];
    this.sendActionToServer(controller.id, "place", { Bool: pressed });

    // Look
    const [rx, ry] = [pad.axes[2], pad.axes[3]];
    const deadzone = 0.2;

    const mx = Math.abs(rx) < deadzone ? 0 : rx;
    const my = Math.abs(ry) < deadzone ? 0 : ry;

    this.sendActionToServer(controller.id, "look", {
      Vec2: { x: mx, y: my },
    });
  }

  //Server Side
  public sendActionToServer(
    id: [number, number],
    action: string,
    value: InputValue
  ) {
    invoke("input_event", { id, action, value }).catch((err) => {
      console.warn("Failed to send input event:", err);
    });
  }
}
