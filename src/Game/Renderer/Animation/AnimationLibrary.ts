import { AnimationState, Kind } from "@/Game/State";
import { AnimData } from "./AnimData";
import { parseKind, parseAnimationState } from "./animutil";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";

class AnimationLibrary {
  private animations = new Map<Kind, Map<AnimationState, AnimData>>();
  rootPath = "resources/assets/animations";
  private loaded = false;

  async loadAllAnimations(): Promise<void> {
    if (this.loaded) return;
    this.loaded = true;

    // Iterate through each Kind
    for (const kind of Object.values(Kind)) {
      try {
        const kindMap = await this.loadAnimation(kind);
        if (!kindMap) continue;

        this.animations.set(kind, kindMap);
      } catch (e) {
        console.log(`Failed to load animations for kind ${kind}:`, e);
      }
    }
  }

  private async loadAnimation(
    kind: Kind,
  ): Promise<Promise<Map<AnimationState, AnimData>> | null> {
    const kindMap = new Map<AnimationState, AnimData>();

    // Load the spritesheet once per kind
    const spritePath = convertFileSrc(
      `${this.rootPath}/${kind.toString().toLowerCase()}/sprite.png`,
    );
    console.log("Loading sprite from", spritePath);

    let image: HTMLImageElement;
    try {
      image = new Image();
      await new Promise((resolve, reject) => {
        image.onload = () => {
          console.log(`Successfully loaded: ${spritePath}`);
          resolve(null);
        };
        image.onerror = (e) => {
          console.error(`Image load error for ${spritePath}:`, e);
          reject(new Error(`Failed to load image: ${spritePath}`));
        };
        console.log(`Attempting to load: ${spritePath}`);
        image.src = spritePath;
      });
    } catch (e) {
      console.log(`Failed to load image for kind ${kind}:`, e);
      return null;
    }

    // Iterate through each AnimationState
    for (const state of Object.values(AnimationState)) {
      await this.loadState(kind, state, image).then((anim) => {
        if (anim) {
          kindMap.set(state, anim);
        }
      });
    }

    return kindMap;
  }

  private async loadState(
    kind: Kind,
    state: AnimationState,
    image: HTMLImageElement,
  ): Promise<AnimData | undefined> {
    try {
      const jsonPath = convertFileSrc(
        `${this.rootPath}/${kind.toString().toLowerCase()}/${state.toString().toLowerCase()}.json`,
      );
      const meta = await this.loadMeta(jsonPath);
      console.log(`Loaded animation meta for ${kind}/${state}:`, meta);
      const anim = new AnimData(
        {
          frameWidth: meta.frame_width,
          frameHeight: meta.frame_height,
          frameCount: meta.frame_count,
          rowIndex: meta.row_index,
          frameDurationMs: meta.frame_duration_ms,
          loop: meta.loop,
        },
        image,
      );

      return anim;
    } catch (e) {
      // Animation doesn't exist for this kind/state combo, skip it
      console.log(`No animation for ${kind}/${state}`);
    }
  }

  private async loadMeta(path: string): Promise<any> {
    const res = await fetch(path);
    if (!res.ok) throw new Error("Failed to load meta.json");
    return res.json();
  }

  get(kind: Kind, state: AnimationState): AnimData | undefined {
    return this.animations.get(kind)?.get(state);
  }

  clear(): void {
    this.animations.clear();
  }
}

export const animationLibrary = new AnimationLibrary();
