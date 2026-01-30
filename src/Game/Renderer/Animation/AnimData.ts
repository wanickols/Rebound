export class AnimData {
  readonly frameWidth: number;
  readonly frameHeight: number;
  readonly frameCount: number;
  readonly frameDurationMs: number;
  readonly loop: boolean;
  readonly image: HTMLImageElement;

  private elapsedMs = 0;

  constructor(
    opts: {
      frameWidth: number;
      frameHeight: number;
      frameCount: number;
      frameDurationMs: number;
      loop?: boolean;
    },
    _image: HTMLImageElement,
  ) {
    this.frameWidth = opts.frameWidth;
    this.frameHeight = opts.frameHeight;
    this.frameCount = opts.frameCount;
    this.frameDurationMs = opts.frameDurationMs;
    this.loop = opts.loop ?? true;
    this.image = _image;
  }

  reset(): void {
    this.elapsedMs = 0;
  }

  update(deltaMs: number): void {
    this.elapsedMs += deltaMs;
  }

  getFrameIndex(): number {
    const frame = Math.floor(this.elapsedMs / this.frameDurationMs);

    if (this.loop) {
      return frame % this.frameCount;
    }

    return Math.min(frame, this.frameCount - 1);
  }

  getSourceRect() {
    const frameIndex = this.getFrameIndex();

    return {
      x: frameIndex * this.frameWidth,
      y: 0,
      width: this.frameWidth,
      height: this.frameHeight,
    };
  }
}
