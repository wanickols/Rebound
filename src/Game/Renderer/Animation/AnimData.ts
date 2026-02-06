export class AnimData {
  readonly frameWidth: number;
  readonly frameHeight: number;
  readonly frameCount: number;
  readonly rowIndex: number;
  readonly frameDurationMs: number;
  readonly loop: boolean;
  readonly image: HTMLImageElement;

  private elapsedMs = 0;
  private currFrame = 0;
  private isDone = false;

  constructor(
    opts: {
      frameWidth: number;
      frameHeight: number;
      frameCount: number;
      rowIndex: number;
      frameDurationMs: number;
      loop?: boolean;
    },
    _image: HTMLImageElement,
  ) {
    this.frameWidth = opts.frameWidth;
    this.frameHeight = opts.frameHeight;
    this.frameCount = opts.frameCount;
    this.rowIndex = opts.rowIndex;
    this.frameDurationMs = opts.frameDurationMs;
    this.loop = opts.loop ?? true;
    this.image = _image;
  }

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
    if (this.elapsedMs >= this.frameDurationMs) {
      this.elapsedMs = 0;
      if (this.currFrame++ >= this.frameCount - 1) {
        if (this.loop) {
          this.currFrame = 0;
        } else {
          this.isDone = true;
        }
      }
    }
    return this.currFrame;
  }

  getSourceRect() {
    const frameIndex = this.getFrameIndex();

    return {
      x: frameIndex * this.frameWidth,
      y: this.rowIndex * this.frameHeight,
      w: this.frameWidth,
      h: this.frameHeight,
    };
  }
}
