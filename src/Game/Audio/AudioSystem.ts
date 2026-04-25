import { InputFrame } from "../Input/InputFrame";
import { inputState } from "../Input/InputManager";
import { FxEventBus } from "../Payload/FxEventBus";
import { audio } from "./AudioManager";
import { watch } from "vue";

export class AudioSystem {
  private stop?: () => void;
  fxb: FxEventBus;

  constructor(fxBus: FxEventBus) {
    this.fxb = fxBus;
    this.fxb.subscribe((e) => {
      if (e.type === "GoalScored") {
        audio.playEffect("goal");
      }
    });

    this.stop = watch(
      () => inputState.frame,
      (frame) => this.handleFrame(frame),
    );
  }

  private handleFrame(frame: InputFrame | null) {
    if (!frame) return;

    if (frame.buttons.grab) {
      audio.playEffect("kick");
    }
  }

  destroy() {
    this.stop?.();
    this.fxb.unsubscribe();
  }
}
