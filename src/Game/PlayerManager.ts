import { reactive, computed } from "vue";
import { listen } from "@tauri-apps/api/event";

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

  assignController(id: PlayerId, controllerIndex: number) {
    const player = this.state.players.get(id[0]);
    if (player) player.controllerIndex = controllerIndex;
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
