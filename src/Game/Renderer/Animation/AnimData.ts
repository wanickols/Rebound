export class AnimData {
  readonly frameWidth: number;
  readonly frameHeight: number;
  readonly frameCount: number;
  readonly rowIndex: number;
  readonly frameDurationMs: number;
  readonly loop: boolean;
  readonly image: HTMLImageElement;

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

  getSourceRect(frameIndex: number): {
    x: number;
    y: number;
    w: number;
    h: number;
  } {
    return {
      x: frameIndex * this.frameWidth,
      y: this.rowIndex * this.frameHeight,
      w: this.frameWidth,
      h: this.frameHeight,
    };
  }
}
