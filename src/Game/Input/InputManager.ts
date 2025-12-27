import { invoke } from "@tauri-apps/api/core";
import { GamepadData } from "./ControllerManager";
import { listen } from "@tauri-apps/api/event";
import { FACE, InputFrame, Vec2 } from "./InputFrame";

export type PlayerId = [number];

export class InputManager {
  //Movement
  lastMove: Record<number, { x: number; y: number }> = {};

  // players that exist (from backend)
  private players = new Set<PlayerId>();

  // controller index → playerId (or null = unbound)
  private bindings = new Map<number, PlayerId | null>();

  constructor() {
    listen<PlayerId[]>("player-list", (event) => {
      this.reconcilePlayers(event.payload);
    });
  }

  private reconcilePlayers(newList: PlayerId[]) {
    const incoming = new Set(newList);

    // Remove players that no longer exist
    for (const playerId of this.players) {
      if (!incoming.has(playerId)) {
        this.players.delete(playerId);

        // detach any controller bound to this player
        for (const [idx, bound] of this.bindings) {
          if (bound === playerId) {
            this.bindings.set(idx, null);
          }
        }
      }
    }

    // Add new players
    for (const playerId of incoming) {
      if (!this.players.has(playerId)) {
        this.players.add(playerId);
        // do not auto-bind here unless you explicitly want that policy
      }
    }
  }

  async onControllerConnected(index: number) {
    // register controller as known but unbound
    this.bindings.set(index, null);

    // try to find an existing unbound player
    const freePlayer = this.findUnassignedPlayer();
    if (freePlayer !== null) {
      this.bindings.set(index, freePlayer);
      return;
    }

    // otherwise, ask backend to create one
    const id = await invoke<PlayerId | null>("request_player_id");
    if (id !== null) {
      console.log("Assigned new player ID", id, "to controller", index);
      this.players.add(id);
      this.bindings.set(index, id);
    }
    // else: leave unbound (ignored)
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

    this.sendInputFrame(playerId, frame);
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

  //Server Side
  sendInputFrame(id: PlayerId, frame: InputFrame) {
    invoke("input_frame", { id, frame }).catch((err) => {
      console.warn("Failed to send input frame:", err);
    });
  }
}
