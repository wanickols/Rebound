import { bus } from "@/utils/EventBus";

// ControllerManager.ts
export interface GamepadData {
  index: number;
  id: string;
  axes: number[];
  buttons: boolean[];
}

export class ControllerManager {
  constructor() {
    window.addEventListener("gamepadconnected", (e) => this.onGamepad(e));
    window.addEventListener("gamepaddisconnected", (e) =>
      this.onGamepadDisconnect(e)
    );
  }
  knownPads: Set<number> = new Set();

  pollGamepads() {
    const pads = navigator.getGamepads();
    for (let i = 0; i < pads.length; i++) {
      const pad = pads[i];
      if (!pad) continue;

      const padData: GamepadData = {
        index: pad.index,
        id: pad.id,
        axes: [...pad.axes],
        buttons: pad.buttons.map((b) => b.pressed),
      };

      if (!this.knownPads.has(pad.index)) {
        this.knownPads.add(pad.index);
        bus.emit("controllerAvailable", pad.index);
      } else {
        bus.emit("gamepadEvent", padData);
      }
    }
  }

  onGamepad(e: GamepadEvent) {
    if (!this.knownPads.has(e.gamepad.index)) {
      this.knownPads.add(e.gamepad.index);
      bus.emit("controllerAvailable", e.gamepad.index);
    }
  }

  onGamepadDisconnect(e: GamepadEvent) {
    const idx = e.gamepad.index;
    if (this.knownPads.has(idx)) {
      this.knownPads.delete(idx);
      bus.emit("controllerRemoved", idx);
    }
  }
}
