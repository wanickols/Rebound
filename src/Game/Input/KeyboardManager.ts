import { bus } from "@/utils/EventBus";
import { playerManager } from "./PlayerManager";

export class KeyboardManager {
  public scale = 1.0;
  mouseDown = false;
  mousePos = { x: 0, y: 0 };

  constructor() {
    playerManager.assignController(-1); // keyboard/mouse
    this.addListeners();
  }

  private addListeners() {
    window.addEventListener("keydown", (e) => this.onKey(e, true));
    window.addEventListener("keyup", (e) => this.onKey(e, false));

    window.addEventListener("mousedown", (e) => {
      if (e.button === 0) this.mouseDown = true;
    });
    window.addEventListener("mouseup", (e) => {
      if (e.button === 0) this.mouseDown = false;
    });
  }

  onKey(e: KeyboardEvent, down: boolean) {
    const key = e.key.toLowerCase();
    const player = playerManager.getPlayerByController(-1);
    if (!player) return;

    player.keys[key] = down;
    bus.emit("keyboardEvent", player);
  }
}
