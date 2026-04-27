import AudioLibrary from "./AudioLibrary";

class AudioManager {
  private library!: AudioLibrary;
  private initialized = false;

  private loops = new Map<string, HTMLAudioElement>();
  private currentMusic: HTMLAudioElement | null = null;

  async init(rootPath: string) {
    if (this.initialized) return;

    this.library = new AudioLibrary();
    await this.library.load(rootPath);

    this.initialized = true;
  }

  playEffect(name: string) {
    const list = this.library.effects.get(name);
    console.log("Effect list for", name, ":", list);
    if (!list) return;

    console.log(`Playing effect: ${name}`);
    const audio = list[Math.floor(Math.random() * list.length)];
    const sound = audio.cloneNode(true) as HTMLAudioElement;
    sound.play();
  }

  playMusic(name: string) {
    const track = this.library.music.get(name);
    if (!track) return;

    // stop previous music
    if (this.currentMusic) {
      this.currentMusic.pause();
      this.currentMusic.currentTime = 0;
    }

    const instance = track.cloneNode(true) as HTMLAudioElement;
    instance.loop = true;
    instance.play();

    this.currentMusic = instance;
  }

  stopMusic() {
    if (!this.currentMusic) return;

    this.currentMusic.pause();
    this.currentMusic.currentTime = 0;
    this.currentMusic = null;
  }

  startLoop(name: string) {
    if (this.loops.has(name)) return;

    const list = this.library.effects.get(name);
    if (!list) return;

    const audio = list[Math.floor(Math.random() * list.length)];
    const instance = audio.cloneNode(true) as HTMLAudioElement;

    instance.loop = true;
    instance.play().catch((err) => {
      if (err.name !== "AbortError") {
        console.warn("Audio play failed:", err);
      }
    });
    instance.play();

    this.loops.set(name, instance);
  }

  stopLoop(name: string) {
    const instance = this.loops.get(name);
    if (!instance) return;

    instance.pause();
    instance.currentTime = 0;

    this.loops.delete(name);
  }
}

export const audio = new AudioManager();
