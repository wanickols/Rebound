// InputManager.ts
import { PlayerId, ReactivePlayerManager } from "@/Game/PlayerManager";
import { invoke } from "@tauri-apps/api/core";

export type GameAction = "move" | "action" | "aim" | "look" | "pause";

type InputValue =
  | { Bool: boolean }
  | { Vec2: { x: number; y: number } }
  | { Float: number }
  | { None: Record<string, never> };

export class InputManager {
  keys: Record<string, boolean> = {};
  lastMove: Record<number, { x: number; y: number }> = {};
  mouseDown = false;
  private scale = 1.0;
  mousePos = { x: 0, y: 0 };
  playerManager: ReactivePlayerManager;

  constructor() {
    this.playerManager = new ReactivePlayerManager();

    window.addEventListener("keydown", (e) => this.onKey(e, true));
    window.addEventListener("keyup", (e) => this.onKey(e, false));
    window.addEventListener("gamepadconnected", (e) =>
      this.addController(e.gamepad.index)
    );
    window.addEventListener("gamepaddisconnected", (e) =>
      this.removeController(e.gamepad.index)
    );

    this.addController(-1); // keyboard/mouse
    this.pollGamepads();

    window.addEventListener("mousedown", (e) => {
      if (e.button === 0) this.mouseDown = true;
    });
    window.addEventListener("mouseup", (e) => {
      if (e.button === 0) this.mouseDown = false;
    });
  }

  handleMouseMove(x: number, y: number) {
    if (!this.mouseDown) return;

    const scaledX = x * this.scale;
    const scaledY = y * this.scale;

    // Only send if the position changed
    if (scaledX === this.mousePos.x && scaledY === this.mousePos.y) return;

    this.mousePos.x = scaledX;
    this.mousePos.y = scaledY;

    const kb = this.playerManager.getPlayerByController(-1);
    if (!kb) return;
    this.sendActionToServer(kb.id, "aim", {
      Vec2: { x: scaledX, y: scaledY },
    });
  }

  setScale(scale: number) {
    this.scale = scale;
  }

  async addController(index: number) {
    // Request player ID from backend
    const id = await invoke<PlayerId | null>("request_player_id");
    if (!id) {
      console.warn("Failed to get player ID for controller", index);
      return;
    }

    // Assign controller to player
    this.playerManager.assignController(id, index);
    console.log("Controller assigned:", { index, id });
  }

  removeController(index: number) {
    const player = this.playerManager.getPlayerByController(index);
    if (player) {
      player.controllerIndex = null;
      console.log("Controller removed from player:", player.id);
    }
  }

  onGamepad(e: GamepadEvent) {
    const gp = e.gamepad;
    this.addController(gp.index);
  }

  onGamepadDisconnect(e: GamepadEvent) {
    let index = e.gamepad.index;
    const player = this.playerManager.getPlayerByController(index);
    if (!player) return;
    this.removeController(index);
    console.log(`Gamepad disconnected: ${index}`);
  }

  onKey(e: KeyboardEvent, down: boolean) {
    const key = e.key.toLowerCase();
    this.keys[key] = down;

    const player = this.playerManager.getPlayerByController(-1);
    console.log("Keyboard check?");
    console.log(player);
    if (!player) return;

    // compute move vector from WASD
    const x = (this.keys["d"] ? 1 : 0) - (this.keys["a"] ? 1 : 0);
    const y = (this.keys["s"] ? 1 : 0) - (this.keys["w"] ? 1 : 0);

    this.updateMove(player.id, -1, x, y); // -1 = keyboard index

    if (key === " ") {
      this.sendActionToServer(player.id, "action", { Bool: down });
    } else if (key === "escape") {
      this.sendActionToServer(player.id, "pause", { Bool: down });
    }
  }

  pollGamepads() {
    const loop = () => {
      const pads = navigator.getGamepads();
      for (const pad of pads) {
        if (!pad) continue;

        const controller = this.playerManager.getPlayerByController(pad.index);
        if (!controller) continue;

        const x = pad.axes[0];
        const y = pad.axes[1];

        this.updateMove(controller.id, pad.index, x, y);

        const pressed = pad.buttons[0].pressed;
        this.sendActionToServer(controller.id, "action", { Bool: pressed });

        const rx = pad.axes[2];
        const ry = pad.axes[3];
        this.sendActionToServer(controller.id, "look", {
          Vec2: { x: rx, y: ry },
        });
      }

      requestAnimationFrame(loop);
    };
    loop();
  }

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

  sendActionToServer(id: [number, number], action: string, value: InputValue) {
    console.log("sending stuff" + id + action + value);
    invoke("input_event", { id, action, value }).catch((err) => {
      console.warn("Failed to send input event:", err);
    });
  }
}
