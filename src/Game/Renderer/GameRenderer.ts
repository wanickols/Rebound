const GAME_WIDTH = 320;
const GAME_HEIGHT = 180;
import { spriteLibrary } from "./SpriteLibrary";
import { State } from "../State";
import { animationLibrary } from "./Animation/AnimationLibrary";

export class GameRenderer {
  private ctx: CanvasRenderingContext2D;
  private canvas: HTMLCanvasElement;

  constructor(ctx: CanvasRenderingContext2D, canvas: HTMLCanvasElement) {
    this.ctx = ctx;
    this.canvas = canvas;
    animationLibrary
      .loadAllAnimations()
      .then(() => {
        console.log("Animations loaded");
      })
      .catch((err) => {
        console.error("Failed to load animations", err);
      });
  }

  updateState(states: State[]) {
    const scale = this.getScale();
    const offsetX = (this.canvas.width - GAME_WIDTH * scale) / 2;
    const offsetY = (this.canvas.height - GAME_HEIGHT * scale) / 2;

    this.clear();

    for (const s of states) {
      this.draw(s, scale, offsetX, offsetY);
    }
  }

  getScale() {
    const canvasWidth = this.canvas.width; // already set to canvas.value.width
    const canvasHeight = this.canvas.height; // already set to canvas.value.height
    const scaleX = canvasWidth / GAME_WIDTH;
    const scaleY = canvasHeight / GAME_HEIGHT;

    return Math.min(scaleX, scaleY);
  }

  draw(s: State, scale: number, offsetX: number, offsetY: number) {
    let { x, y, w, h } = this.getDimensionsForShape(s, scale, offsetX, offsetY);

    const anim = animationLibrary.get(s.kind, s.animation_state);
    if (anim) {
      this.drawAnimated(anim, w, h, x, y);
      return;
    }

    const sprite = spriteLibrary[s.kind.toString()];
    if (sprite && sprite.complete) {
      this.drawSprite(s, sprite, w, h, x, y);
    } else {
      this.drawShape(s, scale, w, h, x, y);
    }
  }

  // Compute position & dimensions based on shape
  private getDimensionsForShape(
    s: State,
    scale: number,
    offsetX: number,
    offsetY: number,
  ) {
    let x = s.x * scale + offsetX;
    let y = s.y * scale + offsetY;
    let w = 0;
    let h = 0;
    if (s.shape.type === "rectangle") {
      w = s.shape.w * scale;
      h = s.shape.h * scale;
    } else if (s.shape.type === "circle") {
      const r = s.shape.radius * scale;
      w = h = r * 2;
      x -= r; // Center it properly
      y -= r;
    }
    return { x, y, w, h };
  }

  // --- Animation Handling ---
  private drawAnimated(anim: any, w: number, h: number, x: number, y: number) {
    const sourceRect = anim.getSourceRect();
    const sprite = anim.image;
    //console.log("Drawing animated sprite frame:", sourceRect);

    this.ctx.drawImage(
      sprite,
      sourceRect.x,
      sourceRect.y,
      sourceRect.w,
      sourceRect.h,
      x,
      y,
      w,
      h,
    );
  }

  // --- SPRITE DRAWING if no Anim ---
  private drawSprite(
    s: State,
    sprite: HTMLImageElement,
    w: number,
    h: number,
    x: number,
    y: number,
  ) {
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
  private drawShape(
    s: State,
    scale: number,
    w: number,
    h: number,
    x: number,
    y: number,
  ) {
    if (s.shape.type === "rectangle") {
      this.ctx.fillStyle = s.is_static ? "gray" : "lime";
      this.ctx.fillRect(x, y, w, h);
    } else if (s.shape.type === "circle") {
      this.ctx.beginPath();
      this.ctx.arc(x, y, s.shape.radius * scale, 0, Math.PI * 2);
      this.ctx.fillStyle = s.is_static ? "gray" : "lime";
      this.ctx.fill();
    }
  }

  private clear() {
    this.ctx.fillStyle = "black";
    this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
  }
}
