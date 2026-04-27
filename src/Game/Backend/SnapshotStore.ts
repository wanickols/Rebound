import { State } from "./Payload/State";
import { GamePayload } from "./Payload/GamePayload";
import { GamePhase } from "./Payload/GamePhase";
import { ScoreManager } from "./Payload/ScoreManager";
import { ref } from "vue";

export class SnapshotStore {
  private payload = ref<GamePayload | null>(null);

  update(payload: GamePayload) {
    this.payload.value = payload;
  }

  reset() {
    this.payload.value = null;
  }

  // ---- state access (renderer / animation / audio) ----
  get states(): State[] {
    return this.payload.value?.states ?? [];
  }

  // ---- UI / flow control ----
  get phase(): GamePhase | undefined {
    return this.payload.value?.phase;
  }

  // ---- UI only ----
  get scoreManager(): ScoreManager | undefined {
    return this.payload.value?.score_manager;
  }

  // optional: debug / inspection
  get hasData(): boolean {
    return this.payload.value !== null;
  }
}

export const snapshotStore = new SnapshotStore();
