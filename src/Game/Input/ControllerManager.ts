import { bus } from "@/utils/EventBus";

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
    for (const pad of pads) {
      if (!pad) continue;
      if (!this.knownPads.has(pad.index)) {
        this.knownPads.add(pad.index);
        bus.emit("controllerAvailable", pad.index);
      } else {
        bus.emit("gamepadEvent", { pad });
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

export const controllerManager = new ControllerManager();
