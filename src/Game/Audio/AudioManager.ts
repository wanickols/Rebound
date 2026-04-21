import AudioLibrary from "./AudioLibrary";

class AudioManager {
  private library!: AudioLibrary;
  private initialized = false;

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

    track.loop = true;
    track.play();
  }
}

export const audio = new AudioManager();
