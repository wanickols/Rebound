import { invoke } from "@tauri-apps/api/core";

export class InputManager {
  private state = {
    up: false,
    down: false,
    left: false,
    right: false,
    action: false,
  };

  constructor() {
    window.addEventListener("keydown", (e) => this.onKey(e, true));
    window.addEventListener("keyup", (e) => this.onKey(e, false));
  }

  private onKey(e: KeyboardEvent, pressed: boolean) {
    let triggered = false;

    switch (e.key) {
      case "ArrowUp":
        triggered = this.state.up !== pressed;
        this.state.up = pressed;
        break;
      case "ArrowDown":
        triggered = this.state.down !== pressed;
        this.state.down = pressed;
        break;
      case "ArrowLeft":
        triggered = this.state.left !== pressed;
        this.state.left = pressed;
        break;
      case "ArrowRight":
        triggered = this.state.right !== pressed;
        this.state.right = pressed;
        break;
      case " ":
        triggered = this.state.action !== pressed;
        this.state.action = pressed;
        break;
    }

    if (triggered) {
      this.callServer(e.key, pressed);
    }
  }

  callServer(key: string, pressed: boolean) {
    invoke("input_event", { key, pressed })
      .then(() => console.log(`Sent ${key}: ${pressed}`))
      .catch((err) => console.error(err));
  }

  getState() {
    return { ...this.state };
  }
}
