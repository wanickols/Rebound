const GAME_WIDTH = 320;
const GAME_HEIGHT = 180;
import { spriteLibrary } from "./SpriteLibrary";
import { AnimationState, State } from "../State";
import { animationLibrary } from "./Animation/AnimationLibrary";
import { InputManager } from "../Input/InputManager";
import { AnimPlayer } from "./Animation/AnimPlayer";

export class GameRenderer {
  private ctx: CanvasRenderingContext2D;
  private canvas: HTMLCanvasElement;
  private states: State[] = [];
  private rafId: number | null = null;
  private inputManager: InputManager;

  private confirmedHolding = false;
  private holdingTimeout: number | null = null;
  private readonly HOLD_CONFIRM_MS = 100;

  constructor(
    ctx: CanvasRenderingContext2D,
    canvas: HTMLCanvasElement,
    inputManager: InputManager,
  ) {
    this.ctx = ctx;
    this.canvas = canvas;
    this.inputManager = inputManager;
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
    this.states = states;
  }

  startRenderLoop() {
    if (this.rafId !== null) return; // Already running

    let lastTime = performance.now();

    const loop = (currentTime: number) => {
      const deltaMs = currentTime - lastTime;
      lastTime = currentTime;

      const scale = this.getScale();
      const offsetX = (this.canvas.width - GAME_WIDTH * scale) / 2;
      const offsetY = (this.canvas.height - GAME_HEIGHT * scale) / 2;

      this.clear();
      // Draw current state
      for (const s of this.states) {
        this.draw(s, scale, offsetX, offsetY, deltaMs);
      }

      this.rafId = requestAnimationFrame(loop);
    };

    this.rafId = requestAnimationFrame(loop);
  }

  stopRenderLoop() {
    if (this.rafId !== null) {
      cancelAnimationFrame(this.rafId);
      this.rafId = null;
    }
  }

  getScale() {
    const canvasWidth = this.canvas.width; // already set to canvas.value.width
    const canvasHeight = this.canvas.height; // already set to canvas.value.height
    const scaleX = canvasWidth / GAME_WIDTH;
    const scaleY = canvasHeight / GAME_HEIGHT;

    return Math.min(scaleX, scaleY);
  }

  draw(
    s: State,
    scale: number,
    offsetX: number,
    offsetY: number,
    deltaMs: number,
  ) {
    let { x, y, w, h } = this.getDimensionsForShape(s, scale, offsetX, offsetY);

    this.applyRotation(s.angle, x + w / 2, y + h / 2);

    this.updateHolding(s);
    const state = this.determineAnimationState(s);
    const anim = animationLibrary.get(s.kind, state);

    if (anim) {
      this.handleAnimPlayers(s, anim, deltaMs);
      this.drawAnimated(s.id, w, h, x, y);
      return;
    }

    const sprite = spriteLibrary[s.kind.toString()];
    if (sprite && sprite.complete) {
      this.drawSprite(s, sprite, w, h, x, y);
    } else {
      this.drawShape(s, scale, w, h, x, y);
    }
  }

  private animPlayers = new Map<number, AnimPlayer>();

  private handleAnimPlayers(s: State, anim: any, deltaMs: number) {
    let player = this.animPlayers.get(s.id);
    if (!player) {
      player = new AnimPlayer();
      this.animPlayers.set(s.id, player);
    }

    player.setLatestAnimData(anim);

    player.update(deltaMs);
  }

  private determineAnimationState(s: State): AnimationState {
    let shooting = this.inputManager.isShooting;
    if (shooting && this.confirmedHolding) {
      return AnimationState.Shooting;
    }

    if (s.vx !== 0 || s.vy !== 0) {
      return AnimationState.Moving;
    }
    return AnimationState.Idle;
  }

  private updateHolding(s: State) {
    if (s.kind != "Ball") return;
    if (s.is_held) {
      if (!this.confirmedHolding && this.holdingTimeout === null) {
        console.log("Holding confirmed");
        this.holdingTimeout = window.setTimeout(() => {
          this.confirmedHolding = true;

          this.holdingTimeout = null;
        }, 1000);
      }
    } else {
      this.holdingTimeout = window.setTimeout(() => {
        this.confirmedHolding = false;

        this.holdingTimeout = null;
      }, 50);
    }
    console.log(
      `Holding: ${s.is_holding}, Confirmed: ${this.confirmedHolding}, Timeout: ${this.holdingTimeout}`,
    );
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
  private drawAnimated(id: number, w: number, h: number, x: number, y: number) {
    const player = this.animPlayers.get(id);
    if (!player) return;

    const sourceRect = player.getSourceRect();
    const sprite = player.getImage();
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
    this.ctx.restore();
  }

  private applyRotation(angle: number, cx: number, cy: number) {
    if (!angle || angle === 0) return; // No rotation needed
    this.ctx.save();
    this.ctx.translate(cx, cy);
    this.ctx.rotate(angle);
    this.ctx.translate(-cx, -cy);
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
    // Draw sprite centered on new origin
    this.ctx.drawImage(sprite, x, y, w, h);
    this.ctx.restore();
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
