import { invoke } from "@tauri-apps/api/core";
import { InputValue } from "./InputTypes";
import { GamepadData } from "./ControllerManager";
import { listen } from "@tauri-apps/api/event";

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

    console.log("Gamepad event for player", playerId, pad);
    // Axes
    const x = pad.axes[0];
    const y = pad.axes[1];

    this.updateMove(playerId, pad.index, x, y);

    // Action
    var pressed = pad.buttons[0];
    this.sendActionToServer(playerId, "action", { Bool: pressed });
    //Place
    pressed = pad.buttons[1];
    this.sendActionToServer(playerId, "place", { Bool: pressed });

    // Look
    const [rx, ry] = [pad.axes[2], pad.axes[3]];
    const deadzone = 0.2;

    const mx = Math.abs(rx) < deadzone ? 0 : rx;
    const my = Math.abs(ry) < deadzone ? 0 : ry;

    this.sendActionToServer(playerId, "look", {
      Vec2: { x: mx, y: my },
    });
  }

  updateMove(id: PlayerId, index: number, x: number, y: number) {
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

  //Server Side
  public sendActionToServer(id: PlayerId, action: string, value: InputValue) {
    invoke("input_event", { id, action, value }).catch((err) => {
      console.warn("Failed to send input event:", err);
    });
  }
}
