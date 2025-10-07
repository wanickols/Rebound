const GAME_WIDTH = 320;
const GAME_HEIGHT = 180;
import { spriteLibrary } from "./SpriteLibrary";
import { State } from "../State";

export class GameRenderer {
  private ctx: CanvasRenderingContext2D;
  private width: number;
  private height: number;

  constructor(ctx: CanvasRenderingContext2D, width: number, height: number) {
    this.ctx = ctx;
    this.width = width;
    this.height = height;
  }

  updateState(states: State[]) {
    const canvasWidth = this.width; // already set to canvas.value.width
    const canvasHeight = this.height; // already set to canvas.value.height

    const scaleX = canvasWidth / GAME_WIDTH;
    const scaleY = canvasHeight / GAME_HEIGHT;

    const scale = Math.min(scaleX, scaleY);
    const offsetX = (canvasWidth - GAME_WIDTH * scale) / 2;
    const offsetY = (canvasHeight - GAME_HEIGHT * scale) / 2;

    this.clear();

    for (const s of states) {
      this.draw(s, scale, offsetX, offsetY);
    }
  }

  draw(s: State, scale: number, offsetX: number, offsetY: number) {
    const sprite = spriteLibrary[s.kind.toString()]; // <--- use s.kind to pick the image

    if (sprite && sprite.complete) {
      this.ctx.drawImage(
        sprite,
        s.x * scale + offsetX,
        s.y * scale + offsetY,
        s.w * scale,
        s.h * scale
      );
    } else {
      this.ctx.fillStyle = s.is_static ? "gray" : "lime";
      this.ctx.fillRect(
        s.x * scale + offsetX,
        s.y * scale + offsetY,
        s.w * scale,
        s.h * scale
      );
    }
  }

  resizeCanvas(canvas: HTMLCanvasElement) {
    const { innerWidth: w, innerHeight: h } = window;
    const aspect = 16 / 9;

    let scale;
    if (w / h > aspect) {
      // window is wider than 16:9 -> scale based on height
      scale = h / 1080;
    } else {
      // window is taller than 16:9 -> scale based on width
      scale = w / 1920;
    }

    canvas.style.width = `${1920 * scale}px`;
    canvas.style.height = `${1080 * scale}px`;
  }

  private clear() {
    this.ctx.fillStyle = "black";
    this.ctx.fillRect(0, 0, this.width, this.height);
  }
}
