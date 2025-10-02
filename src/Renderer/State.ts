class State {
  constructor(
    public x: number,
    public y: number,
    public vx: number,
    public vy: number,
    public w: number,
    public h: number,
    public is_static: boolean,
    public friction: number,
    public restitution: number,
    public kind: string
  ) {}

  draw(ctx: CanvasRenderingContext2D) {
    const sprite = spriteLibrary[this.kind];
    if (sprite && sprite.complete) {
      ctx.drawImage(sprite, this.x, this.y, this.w, this.h);
    } else {
      // fallback: draw a colored box
      ctx.fillStyle = this.is_static ? "gray" : "lime";
      ctx.fillRect(this.x, this.y, this.w, this.h);
    }
  }
}
