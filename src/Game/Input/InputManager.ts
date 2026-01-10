import { invoke } from "@tauri-apps/api/core";
import { GamepadData } from "./ControllerManager";
import { listen } from "@tauri-apps/api/event";
import { FACE, InputFrame, isInputFrameEqual, Vec2 } from "./InputFrame";
import {
  sendClientRequest,
  sendClientRequestWithResponse,
} from "./ClientRequest";

export type PlayerId = number;

export class InputManager {
  //Movement
  lastMove: Record<number, { x: number; y: number }> = {};
  lastInput: Map<number, InputFrame> = new Map();

  // players that exist (from backend)
  private players = new Set<PlayerId>();

  // controller index → playerId (or null = unbound)
  private bindings = new Map<number, PlayerId | null>();
  private unsubAddedPlayer?: () => void;

  constructor() {
    // async IIFE because constructors can’t be async
    (async () => {
      const unlisten = await listen<PlayerId>("added_player", (event) => {
        const newPlayerId = event.payload;
        console.log("New player added:", newPlayerId);

        const freeController = Array.from(this.bindings.entries()).find(
          ([_, bound]) => bound === null
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

  handleGamepadEvent(pad: GamepadData) {
    const playerId = this.bindings.get(pad.index);
    if (!playerId) return;

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
        grab: pad.buttons[FACE.BOTTOM],
        dash: pad.buttons[FACE.LEFT],
        place: pad.buttons[FACE.RIGHT],
      },
    };

    // Compare to last frame
    const lastFrame = this.lastInput.get(playerId);
    if (lastFrame && isInputFrameEqual(lastFrame, frame)) {
      // Nothing changed, don't send
      return;
    }

    this.lastInput.set(playerId, frame);

    sendClientRequest({
      type: "Input",
      entity_id: playerId,
      frame: frame,
    });
  }

  updateMove(index: number, x: number, y: number): Vec2 {
    const deadzone = 0.2;
    const mx = Math.abs(x) < deadzone ? 0 : x;
    const my = Math.abs(y) < deadzone ? 0 : y;

    const last = this.lastMove[index] || { x: 0, y: 0 };

    if (last.x !== mx || last.y !== my) {
      this.lastMove[index] = { x: mx, y: my };
    }

    return this.lastMove[index];
  }

  destroy() {
    this.unsubAddedPlayer?.();
    this.bindings.clear();
    this.players.clear();
  }
}
