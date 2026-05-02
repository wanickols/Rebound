import { audio } from "./AudioManager";

export class StepPlayer {
  private stepTimer = 0;

  update(dt: number, speed: number) {
    if (speed < 0.1) {
      this.stepTimer = 0;
      return;
    }

    // faster movement = faster steps
    const stepInterval = 0.6 / speed;

    this.stepTimer -= dt;

    if (this.stepTimer <= 0) {
      audio.playEffect("steps");
      this.stepTimer = stepInterval;
    }
  }
}
