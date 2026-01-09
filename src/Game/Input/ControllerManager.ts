import { bus } from "@/utils/EventBus";

// ControllerManager.ts
export interface GamepadData {
  index: number;
  id: string;
  axes: number[];
  buttons: boolean[];
}
export class ControllerManager {
  private knownPads = new Set<number>();

  pollGamepads() {
    const pads = navigator.getGamepads();

    for (const pad of pads) {
      if (!pad) continue;

      // Newly discovered controller
      if (!this.knownPads.has(pad.index)) {
        this.knownPads.add(pad.index);
        console.log("Controller connected:", pad.index);
        bus.emit("controllerConnected", pad.index);
      }

      // Always emit input
      bus.emit("gamepadEvent", {
        index: pad.index,
        id: pad.id,
        axes: [...pad.axes],
        buttons: pad.buttons.map((b) => b.pressed),
      });
    }

    // Detect disconnects
    for (const idx of [...this.knownPads]) {
      if (!pads[idx]) {
        this.knownPads.delete(idx);
        bus.emit("controllerDisconnected", idx);
      }
    }
  }

  destroy() {
    this.knownPads.clear();
  }
}
