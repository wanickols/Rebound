import { ReactivePlayerManager } from "./PlayerManager";
import { InputManager } from "./InputManager";

export class KeyboardManager {
  playerManager: ReactivePlayerManager;
  inputManager: InputManager;

  keys: Record<string, boolean> = {};
  public scale = 1.0;
  mouseDown = false;
  mousePos = { x: 0, y: 0 };

  constructor(
    inputManager: InputManager,
    playerManager: ReactivePlayerManager
  ) {
    this.playerManager = playerManager;
    this.inputManager = inputManager;
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

  handleMouseMove(x: number, y: number) {
    if (!this.mouseDown) return;

    const scaledX = x * this.scale;
    const scaledY = y * this.scale;

    // Only send if the position changed
    if (scaledX === this.mousePos.x && scaledY === this.mousePos.y) return;

    this.mousePos.x = scaledX;
    this.mousePos.y = scaledY;

    const kb = this.playerManager.getPlayerByController(-1);
    if (!kb) return;
    this.inputManager.sendActionToServer(kb.id, "aim", {
      Vec2: { x: scaledX, y: scaledY },
    });
  }

  onKey(e: KeyboardEvent, down: boolean) {
    const key = e.key.toLowerCase();
    this.keys[key] = down;

    const player = this.playerManager.getPlayerByController(-1);
    console.log("Keyboard check?");
    console.log(player);
    if (!player) return;

    // compute move vector from WASD
    const x = (this.keys["d"] ? 1 : 0) - (this.keys["a"] ? 1 : 0);
    const y = (this.keys["s"] ? 1 : 0) - (this.keys["w"] ? 1 : 0);

    this.inputManager.updateMove(player.id, -1, x, y); // -1 = keyboard index

    if (key === " ") {
      this.inputManager.sendActionToServer(player.id, "action", { Bool: down });
    } else if (key === "escape") {
      this.inputManager.sendActionToServer(player.id, "pause", { Bool: down });
    }
  }
}
