import { InputEventBus, InputEvent } from "../Input/InputEventBus";
import { FxEvent } from "../Backend/FxEvent";
import { FxEventBus } from "../Backend/FxEventBus";
import { audio } from "./AudioManager";

export class AudioSystem {
  private stop?: () => void;
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
    this.stop?.();
    this.fxb.unsubscribe();
  }
}
