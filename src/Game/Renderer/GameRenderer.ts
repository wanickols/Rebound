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
    const scale = this.getScale();
    const offsetX = (this.width - GAME_WIDTH * scale) / 2;
    const offsetY = (this.height - GAME_HEIGHT * scale) / 2;

    this.clear();

    for (const s of states) {
      this.draw(s, scale, offsetX, offsetY);
    }
  }

  getScale() {
    const canvasWidth = this.width; // already set to canvas.value.width
    const canvasHeight = this.height; // already set to canvas.value.height
    const scaleX = canvasWidth / GAME_WIDTH;
    const scaleY = canvasHeight / GAME_HEIGHT;

    return Math.min(scaleX, scaleY);
  }

  draw(s: State, scale: number, offsetX: number, offsetY: number) {
    const sprite = spriteLibrary[s.kind.toString()];
    // Compute position & dimensions based on shape
    let x = s.x * scale + offsetX;
    let y = s.y * scale + offsetY;
    let w = 0;
    let h = 0;
    if (s.shape.type === "rectangle") {
      w = s.shape.w * scale;
      h = s.shape.h * scale;
    } else if (s.shape.type === "circle") {
      // For circles, you might center the sprite around (x, y)
      const r = s.shape.radius * scale;
      w = h = r * 2;
      x -= r; // Center it properly
      y -= r;
    }
    // --- SPRITE DRAWING ---
    if (sprite && sprite.complete) {
      const cx = x + w / 2; // center of sprite
      const cy = y + h / 2;

      this.ctx.save(); // Save current transform
      this.ctx.translate(cx, cy); // Move origin to sprite center

      // Only rotate if the state has an angle
      if (s.angle && s.angle !== 0) {
        this.ctx.rotate(s.angle); // s.angle should be in radians
      }

      // Draw sprite centered on new origin
      this.ctx.drawImage(sprite, -w / 2, -h / 2, w, h);

      this.ctx.restore(); // Restore transform
    }

    // --- FALLBACK (no sprite) ---
    else {
      if (s.shape.type === "rectangle") {
        this.ctx.fillStyle = s.is_static ? "gray" : "lime";
        this.ctx.fillRect(x, y, w, h);
      } else if (s.shape.type === "circle") {
        this.ctx.beginPath();
        this.ctx.arc(
          s.x * scale + offsetX,
          s.y * scale + offsetY,
          s.shape.radius * scale,
          0,
          Math.PI * 2
        );
        this.ctx.fillStyle = s.is_static ? "gray" : "lime";
        this.ctx.fill();
      }
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
