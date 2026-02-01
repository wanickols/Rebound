export class AnimData {
  readonly frameWidth: number;
  readonly frameHeight: number;
  readonly frameCount: number;
  readonly rowIndex: number;
  readonly frameDurationMs: number;
  readonly loop: boolean;
  readonly image: HTMLImageElement;

  private elapsedMs = 0;

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
      y: this.rowIndex * this.frameHeight,
      w: this.frameWidth,
      h: this.frameHeight,
    };
  }
}
