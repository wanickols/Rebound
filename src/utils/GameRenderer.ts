export type GameState = {
  x: number;
  y: number;
};

export class GameRenderer {
  private ctx: CanvasRenderingContext2D;
  private width: number;
  private height: number;
  private boxSize = 40; // size of player box
  private state: GameState = { x: 0, y: 0 };

  constructor(ctx: CanvasRenderingContext2D, width: number, height: number) {
    this.ctx = ctx;
    this.width = width;
    this.height = height;
  }

  updateState(state: GameState) {
    this.state = state;
    this.draw();
  }

  private clear() {
    this.ctx.fillStyle = "black";
    this.ctx.fillRect(0, 0, this.width, this.height);
  }

  private drawBox(x: number, y: number) {
    this.ctx.fillStyle = "lime";
    this.ctx.fillRect(x, y, this.boxSize, this.boxSize);
  }

  private draw() {
    this.clear();
    this.drawBox(this.state.x, this.state.y);
  }
}
