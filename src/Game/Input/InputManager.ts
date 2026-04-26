import { GamepadData } from "./ControllerManager";
import { listen } from "@tauri-apps/api/event";
import {
  FACE,
  InputFrame,
  isInputFrameEqual,
  SHOULDER,
  Vec2,
} from "./InputFrame";
import { sendClientRequest } from "./ClientRequest";
import { reactive } from "vue";
import { InputEventBus } from "./InputEventBus";

export type PlayerId = number;

export class InputManager {
  //Movement
  lastMove: Record<number, { x: number; y: number }> = {};
  lastInput: Map<number, InputFrame> = new Map();

  //Events
  private bus: InputEventBus;

  ///Initialization
  constructor(bus: InputEventBus) {
    this.bus = bus;
    // async IIFE because constructors can’t be async
    (async () => {
      const unlisten = await listen<PlayerId>("added_player", (event) => {
        const newPlayerId = event.payload;
        console.log("New player added:", newPlayerId);

        const freeController = Array.from(this.bindings.entries()).find(
          ([_, bound]) => bound === null,
        );
        if (freeController) {
          this.bindings.set(freeController[0], newPlayerId);
          this.players.add(newPlayerId);
        }

        console.log(this.players);
      });

      this.unsubAddedPlayer = unlisten;
    })();
  }

  ///Controller Management (TODO: Move to its own component)

  ///TODO: Mov to it's own component
  // Player/Controller Management
  private players = new Set<PlayerId>();
  private bindings = new Map<number, PlayerId | null>();
  private unsubAddedPlayer?: () => void;

  async onControllerConnected(index: number) {
    // register controller as known but unbound
    this.bindings.set(index, null);

    // try to bind to an already existing unassigned player
    const freePlayer = this.findUnassignedPlayer();
    if (freePlayer !== null) {
      this.bindings.set(index, freePlayer);
      return;
    }

    sendClientRequest({
      type: "Add",
    });
  }

  private findUnassignedPlayer(): PlayerId | null {
    for (const playerId of this.players) {
      let taken = false;
      for (const bound of this.bindings.values()) {
        if (bound === playerId) {
          taken = true;
          break;
        }
      }
      if (!taken) return playerId;
    }
    return null;
  }

  onControllerDisconnected(index: number) {
    this.bindings.delete(index);
  }

  ///Handling Input
  handleGamepadEvent(pad: GamepadData) {
    const playerId = this.bindings.get(pad.index);
    if (!playerId) return;

    const frame = this.buildFrame(pad);

    // 🔥 Always process events (even if frame is same)
    this.handleFrameEvents(playerId, frame);

    // 🚫 Only send if changed
    if (!this.shouldSendFrame(playerId, frame)) return;

    this.lastInput.set(playerId, frame);

    sendClientRequest({
      type: "Input",
      entity_id: playerId,
      frame,
    });
  }

  ///Handling Input Helpers

  private buildFrame(pad: GamepadData): InputFrame {
    const move_axis = this.updateMove(pad.index, pad.axes[0], pad.axes[1]);

    const deadzone = 0.2;
    const rx = pad.axes[2];
    const ry = pad.axes[3];

    const look = {
      x: Math.abs(rx) < deadzone ? 0 : rx,
      y: Math.abs(ry) < deadzone ? 0 : ry,
    };

    const frame: InputFrame = {
      move_axis,
      look,
      buttons: {
        grab: pad.buttons[SHOULDER.RIGHT_TRIGGER],
        dash: pad.buttons[FACE.LEFT],
        place: pad.buttons[FACE.RIGHT],
      },
    };

    return frame;
  }

  private handleFrameEvents(playerId: number, frame: InputFrame) {
    const last = this.lastInput.get(playerId);

    if (!last) return;

    // Kick (edge trigger)
    if (!last.buttons.grab && frame.buttons.grab) {
      this.bus.emit({ type: "kick" });
    }

    // Movement start/stop
    const wasMoving = this.isMoving(last);
    const isMoving = this.isMoving(frame);

    if (!wasMoving && isMoving) {
      this.bus.emit({ type: "move_start" });
    }

    if (wasMoving && !isMoving) {
      this.bus.emit({ type: "move_stop" });
    }
  }

  private updateMove(index: number, x: number, y: number): Vec2 {
    const deadzone = 0.2;
    const mx = Math.abs(x) < deadzone ? 0 : x;
    const my = Math.abs(y) < deadzone ? 0 : y;

    const last = this.lastMove[index] || { x: 0, y: 0 };

    if (last.x !== mx || last.y !== my) {
      this.lastMove[index] = { x: mx, y: my };
    }

    return this.lastMove[index];
  }

  ///Helpers
  private shouldSendFrame(playerId: number, frame: InputFrame): boolean {
    const last = this.lastInput.get(playerId);
    return !last || !isInputFrameEqual(last, frame);
  }

  private isMoving(frame: InputFrame): boolean {
    return frame.move_axis.x !== 0 || frame.move_axis.y !== 0;
  }

  ///Cleanup
  destroy() {
    this.unsubAddedPlayer?.();
    this.bindings.clear();
    this.players.clear();
  }
}
