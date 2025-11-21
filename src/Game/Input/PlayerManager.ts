import { reactive, computed } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { Store } from "@tauri-apps/plugin-store";

export type PlayerId = [number, number];

export interface Player {
  id: PlayerId; // still the full tuple
  controllerIndex: number | null;
  keys: Record<string, boolean>;
  mouseDown?: boolean;
}

export class ReactivePlayerManager {
  private store: Store | undefined;
  private state = reactive<{ players: Map<number, Player> }>({
    players: new Map(), // key = id[0]
  });

  constructor() {
    this.initStorage();
    listen<PlayerId[]>("player-list", (event) => {
      this.updateFromBackend(event.payload);
    });
  }

  private async initStorage() {
    this.store = await Store.load(".players.dat");
    const saved = await this.store?.get("players");
    if (Array.isArray(saved)) {
      for (const [id, p] of saved) {
        this.state.players.set(id, p);
      }
    }
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

    this.storeMap();
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

    this.storeMap();
    console.log("Controller assigned:", { index, id });
  }

  async removeController(index: number) {
    const player = this.getPlayerByController(index);
    if (player) {
      player.controllerIndex = null;
      console.log("Controller removed from player:", player.id);
      this.storeMap();
    } else {
      console.log("Tried to remove non existent controller at index: ", index);
    }
  }

  async storeMap() {
    await this.store?.set("players", Array.from(this.state.players.entries()));
    await this.store?.save();
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

export const playerManager = new ReactivePlayerManager();
