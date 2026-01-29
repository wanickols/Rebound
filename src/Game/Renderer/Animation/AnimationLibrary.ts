import { AnimData } from "./AnimData";

export class AnimationLibrary {
  private animations = new Map<string, AnimData>();

  async loadFromFolder(rootPath: string): Promise<void> {
    // iterate child folders
  }

  private async loadAnimationPackage(folderPath: string): Promise<void> {
    // find meta.json
    // find image
    // parse meta
    // load image
    // create AnimData
    // store in map
  }

  get(id: string): AnimData | undefined {
    return this.animations.get(id);
  }

  has(id: string): boolean {
    return this.animations.has(id);
  }

  clear(): void {
    this.animations.clear();
  }
}
