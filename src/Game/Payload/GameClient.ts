import { ref } from "vue";
import { GamePayload } from "./GamePayload";
import { listen } from "@tauri-apps/api/event";
import { FxEventBus } from "./FxEventBus";

class GameClient {
  state = ref<GamePayload | null>(null);
  fxEventBus = new FxEventBus();

  start() {
    listen<GamePayload>("game-state", (event) => {
      const payload = GamePayload.from(event.payload);

      // store latest state
      this.state.value = payload;

      // emit FX events
      this.fxEventBus.emit(payload.fx_events);
    });
  }

  stop() {
    //later
  }
}

export const gameClient = new GameClient();
