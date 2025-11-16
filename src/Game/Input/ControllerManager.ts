import { ReactivePlayerManager } from "./PlayerManager";
import { InputManager } from "./InputManager";

export class ControllerManager {
  playerManager: ReactivePlayerManager;
  inputManager: InputManager;

  constructor(
    inputManager: InputManager,
    playerManager: ReactivePlayerManager
  ) {
    this.playerManager = playerManager;
    this.inputManager = inputManager;
    this.pollGamepads();

    window.addEventListener("gamepadconnected", (e) => this.onGamepad(e));
    window.addEventListener("gamepaddisconnected", (e) =>
      this.onGamepadDisconnect(e)
    );
  }

  //Connections
  onGamepad(e: GamepadEvent) {
    this.playerManager.assignController(e.gamepad.index);
  }

  onGamepadDisconnect(e: GamepadEvent) {
    let index = e.gamepad.index;
    this.playerManager.removeController(index);
    console.log(`Gamepad disconnected: ${index}`);
  }

  //Updating
  pollGamepads() {
    const loop = () => {
      const pads = navigator.getGamepads();
      for (const pad of pads) {
        if (!pad) continue;

        const controller = this.playerManager.getPlayerByController(pad.index);
        if (!controller) continue;

        const x = pad.axes[0];
        const y = pad.axes[1];

        this.inputManager.updateMove(controller.id, pad.index, x, y);

        const pressed = pad.buttons[0].pressed;
        this.inputManager.sendActionToServer(controller.id, "action", {
          Bool: pressed,
        });

        //TODO: Set as own function
        const rx = pad.axes[2];
        const ry = pad.axes[3];
        const deadzone = 0.2;
        const mx = Math.abs(rx) < deadzone ? 0 : rx;
        const my = Math.abs(ry) < deadzone ? 0 : ry;
        this.inputManager.sendActionToServer(controller.id, "look", {
          Vec2: { x: mx, y: my },
        });
      }

      requestAnimationFrame(loop);
    };
    loop();
  }
}
