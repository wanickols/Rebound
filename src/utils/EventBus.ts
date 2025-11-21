type Handler = (payload: any) => void;

class EventBus {
  private listeners: Map<string, Handler[]> = new Map();

  on(event: string, handler: Handler) {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
    }
    this.listeners.get(event)!.push(handler);
  }

  off(event: string, handler: Handler) {
    const arr = this.listeners.get(event);
    if (!arr) return;
    this.listeners.set(
      event,
      arr.filter((h) => h !== handler)
    );
  }

  emit(event: string, payload?: any) {
    const arr = this.listeners.get(event);
    if (!arr) return;
    for (const handler of arr) handler(payload);
  }
}

// Singleton bus
export const bus = new EventBus();
