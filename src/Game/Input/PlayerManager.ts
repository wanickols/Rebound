import { reactive, computed } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export type PlayerId = [number, number];

interface Player {
  id: PlayerId; // still the full tuple
  controllerIndex: number | null;
  keys: Record<string, boolean>;
  mouseDown?: boolean;
}

export class ReactivePlayerManager {
  private state = reactive<{ players: Map<number, Player> }>({
    players: new Map(), // key = id[0]
  });

  constructor() {
    listen<PlayerId[]>("player-list", (event) => {
      this.updateFromBackend(event.payload);
    });
  }

  private updateFromBackend(newList: PlayerId[]) {
    const newKeys = new Set(newList.map((id) => id[0]));

    // Remove players that disappeared
    for (const key of this.state.players.keys()) {
      if (!newKeys.has(key)) this.state.players.delete(key);
    }

    // Add new players
    for (const id of newList) {
      if (!this.state.players.has(id[0])) {
        this.state.players.set(id[0], {
          id, // keep the full tuple
          controllerIndex: null,
          keys: {},
          mouseDown: false,
        });
      }
    }
  }

  async assignController(index: number) {
    // Request player ID from backend
    const id = await invoke<PlayerId | null>("request_player_id");
    if (!id) {
      console.warn("Failed to get player ID for controller", index);
      return;
    }

    const player = this.state.players.get(id[0]);
    if (player) player.controllerIndex = index;

    console.log("Controller assigned:", { index, id });
  }

  async removeController(index: number) {
    const player = this.getPlayerByController(index);
    if (player) {
      player.controllerIndex = null;
      console.log("Controller removed from player:", player.id);
    } else {
      console.log("Tried to remove non existent controller at index: ", index);
    }
  }

  getPlayerByController(index: number) {
    return Array.from(this.state.players.values()).find(
      (p) => p.controllerIndex === index
    );
  }

  get players() {
    return computed(() => Array.from(this.state.players.values()));
  }
}
