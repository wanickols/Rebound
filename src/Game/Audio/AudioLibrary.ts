import { convertFileSrc, invoke } from "@tauri-apps/api/core";

class AudioLibrary {
  private loaded = false;

  effects = new Map<string, HTMLAudioElement[]>();
  music = new Map<string, HTMLAudioElement>();

  async load(rootPath: string) {
    if (this.loaded) return;
    this.loaded = true;

    const files: string[] = await invoke("list_audio_files", {
      path: `${rootPath}/effects`,
    });

    if (!files || files.length === 0) {
      console.warn("No audio files found");
      return;
    }

    for (const file of files) {
      const fileName = getFileName(file); // goal.ogg

      const name = fileName.split(".")[0];
      const key = name.split("_")[0];

      const url = convertFileSrc(file);
      const audio = new Audio(url);

      if (!this.effects.has(key)) this.effects.set(key, []);
      this.effects.get(key)!.push(audio);

      console.log(`Loaded audio: ${key} from ${url}`);
      this.effects.get(key)!.push(audio);
    }
  }
}

function getFileName(path: string) {
  return path.split(/[/\\]/).pop()!;
}

export default AudioLibrary;
