import { AnimationState, Kind } from "@/Game/State";
import { AnimData } from "./AnimData";
import { parseKind, parseAnimationState } from "./animutil";
import { invoke } from "@tauri-apps/api/core";

class AnimationLibrary {
  private animations = new Map<Kind, Map<AnimationState, AnimData>>();

  private loaded = false;

  async loadFromFolder(rootPath: string): Promise<void> {
    if (this.loaded) {
      throw new Error("AnimationLibrary already loaded");
    }
    this.loaded = true;

    const folders = await this.listDirectories(rootPath);
    console.log("Animation folders:", folders);
    for (const folder of folders) {
      try {
        await this.loadAnimationPackage(`${rootPath}/${folder}`);
      } catch (err) {
        console.warn(`Failed to load animation in ${folder}`, err);
      }
    }
  }

  private async loadAnimationPackage(folderPath: string): Promise<void> {
    const imagePath = await this.findImage(folderPath);
    if (!imagePath) {
      throw new Error(`No image found in ${folderPath}`);
    }

    const image = await this.loadImage(imagePath);

    console.log("Image Files :", imagePath);

    const jsonFiles = await this.tryLoadJson(folderPath);
    console.log("Animation JSON files in", folderPath, ":", jsonFiles);
    for (const jsonPath of jsonFiles) {
      const meta = await this.loadMeta(jsonPath);

      const anim = new AnimData(meta, image);
      const kind = parseKind(meta.kind);
      const state = parseAnimationState(meta.state);

      if (!this.animations.has(kind)) {
        this.animations.set(kind, new Map());
      }

      this.animations.get(kind)!.set(state, anim);
    }
  }

  private async tryLoadJson(path: string): Promise<any | null> {
    try {
      const res = await fetch(path);
      if (!res.ok) return null;
      console.log("Loaded JSON from", path);
      return await res.json();
    } catch {
      return null;
    }
  }

  private async loadMeta(path: string): Promise<any> {
    const res = await fetch(path);
    if (!res.ok) throw new Error("Failed to load meta.json");
    return res.json();
  }

  private loadImage(path: string): Promise<HTMLImageElement> {
    return new Promise((resolve, reject) => {
      const img = new Image();
      img.onload = () => resolve(img);
      img.onerror = reject;
      img.src = path;
    });
  }

  private async findImage(folderPath: string): Promise<string | null> {
    // naive but effective
    const candidates = ["png", "webp", "jpg"];

    for (const ext of candidates) {
      const path = `${folderPath}/sprite.${ext}`;
      const res = await fetch(path, { method: "HEAD" });
      if (res.ok) {
        console.log("Found image:", path);
        return path;
      }
    }
    console.warn("No image found in", folderPath);
    return null;
  }

  private async listDirectories(rootPath: string): Promise<string[]> {
    console.log("Listing directories in", rootPath);
    return await invoke<string[]>("list_animation_folders");
  }
  get(kind: Kind, state: AnimationState): AnimData | undefined {
    return this.animations.get(kind)?.get(state);
  }

  clear(): void {
    this.animations.clear();
  }
}

export const animationLibrary = new AnimationLibrary();
