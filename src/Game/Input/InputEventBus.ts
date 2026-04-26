export type InputEvent =
  | { type: "kick" }
  | { type: "move_start" }
  | { type: "move_stop" };

export class InputEventBus {
  private listeners: ((e: InputEvent) => void)[] = [];

  emit(event: InputEvent) {
    for (const l of this.listeners) {
      l(event);
    }
  }

  subscribe(fn: (e: InputEvent) => void) {
    this.listeners.push(fn);

    return () => {
      this.listeners = this.listeners.filter((l) => l !== fn);
    };
  }
}
