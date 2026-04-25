import { FxEvent } from "./FxEvent";

export class FxEventBus {
  private listeners: ((e: FxEvent) => void)[] = [];

  emit(events: FxEvent[]) {
    for (const e of events) {
      for (const l of this.listeners) l(e);
    }
  }

  subscribe(fn: (e: FxEvent) => void) {
    this.listeners.push(fn);
  }
}
