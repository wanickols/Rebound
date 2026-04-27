import { listen } from "@tauri-apps/api/event";
import { FxEventBus } from "./FxEventBus";
import { SnapshotStore } from "./SnapshotStore";
import { GamePayload } from "./Payload/GamePayload";

class GameClient {
  snapshot = new SnapshotStore();
  fxEventBus = new FxEventBus();

  start() {
    listen<GamePayload>("game-state", (event) => {
      const payload = GamePayload.from(event.payload);
      this.onPayload(payload);
    });
  }

  onPayload(payload: GamePayload) {
    this.snapshot.update(payload);

    this.fxEventBus.emit(payload.fx_events);
  }

  stop() {
    //later
  }
}

export const gameClient = new GameClient();
