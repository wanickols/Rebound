import { AnimData } from "./AnimData";

export class AnimPlayer {
  private data?: AnimData;
  private elapsedMs = 0;
  private currFrame = 0;
  private isDone = false;

  reset(): void {
    this.elapsedMs = 0;
    this.currFrame = 0;
    this.isDone = false;
    console.log("Animation reset");
  }

  getDone(): boolean {
    return this.isDone;
  }

  update(deltaMs: number): void {
    this.elapsedMs += deltaMs;
  }

  getFrameIndex(): number {
    if (this.isDone) {
      return this.currFrame; // Stay on last frame if done
    }

    if (this.elapsedMs >= this.data!.frameDurationMs) {
      this.elapsedMs = 0;
      if (this.currFrame++ >= this.data!.frameCount - 1) {
        if (this.data!.loop) {
          this.currFrame = 0;
        } else {
          this.isDone = true;
          this.currFrame = this.data!.frameCount - 1; // Stay on last frame if done
        }
      }
    }
    return this.currFrame;
  }

  getSourceRect(): { x: number; y: number; w: number; h: number } {
    if (!this.data) {
      throw new Error("No animation data set");
    }
    return this.data!.getSourceRect(this.getFrameIndex());
  }
}
