import { InputEventBus, InputEvent } from "../Input/InputEventBus";
import { watch } from "vue";
import { FxEventBus } from "../Backend/FxEventBus";
import { audio } from "./AudioManager";
import { FxEvent } from "../Backend/FxEvent";
import { gameClient } from "../Backend/GameClient";
import { ActionState } from "../Backend/Payload/State";
export class AudioSystem {
  private stopWatch?: () => void;
  fxb: FxEventBus;
  inputBus: InputEventBus;

  constructor(fxBus: FxEventBus, inputBus: InputEventBus) {
    this.fxb = fxBus;
    this.inputBus = inputBus;

    // Backend events
    this.fxb.subscribe((e) => {
      this.handleFxEvent(e);
    });

    // Frontend input events
    this.inputBus.subscribe((e) => {
      this.handleInputEvent(e);
    });

    this.stopWatch = watch(
      () => gameClient.snapshot.states,
      (states) => {
        let isMoving = false;
        for (const s of states) {
          if (s.action_state === ActionState.Moving) {
            isMoving = true;
            break;
          }
        }
        if (isMoving) {
          audio.startLoop("step");
        } else {
          audio.stopLoop("step");
        }
      },
      { deep: false },
    );
  }

  handleFxEvent(e: FxEvent) {
    switch (e.type) {
      case "GoalScored":
        audio.playEffect("goal");
        break;
    }
  }

  handleInputEvent(e: InputEvent) {
    switch (e.type) {
      case "kick":
        audio.playEffect("kick");
        break;
    }
  }

  destroy() {
    this.stopWatch?.();
    this.fxb.unsubscribe();
  }
}
