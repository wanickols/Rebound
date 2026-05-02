import AudioLibrary from "./AudioLibrary";

export enum AudioChannel {
  Music,
  Footsteps,
  SFX,
}

type PlayOptions = {
  volume?: number;
  loop?: boolean;
  tag?: AudioChannel; // for tracking (loops, music, etc.)
};

class AudioManager {
  private library!: AudioLibrary;
  private initialized = false;

  private channels = new Map<
    AudioChannel,
    { source: AudioBufferSourceNode; gain: GainNode }
  >();
  private ctx = new AudioContext();

  getContext() {
    return this.ctx;
  }

  async init(rootPath: string) {
    if (this.initialized) return;

    this.library = new AudioLibrary();
    await this.library.load(rootPath, this.ctx);

    this.initialized = true;
  }

  play(
    name: string,
    opts?: {
      volume?: number;
      loop?: boolean;
      channel?: AudioChannel;
    },
  ) {
    const buffer = this.getBuffer(name);
    if (!buffer) return;

    const source = this.ctx.createBufferSource();
    const gain = this.ctx.createGain();

    source.buffer = buffer;
    source.loop = !!opts?.loop;
    gain.gain.value = opts?.volume ?? 1;

    source.connect(gain);
    gain.connect(this.ctx.destination);

    const now = this.ctx.currentTime;

    // If channel exists, replace it
    if (opts?.channel !== undefined) {
      const existing = this.channels.get(opts.channel);
      if (existing) {
        existing.gain.gain.linearRampToValueAtTime(0, now + 0.05);
        existing.source.stop(now + 0.05);
      }

      this.channels.set(opts.channel, { source, gain });
    }

    source.start();
  }

  getBuffer(name: string): AudioBuffer | null {
    const buffers =
      this.library.effects.get(name) ??
      (this.library.music.get(name) && [this.library.music.get(name)!]);

    if (!buffers || buffers.length === 0) return null;

    return buffers[Math.floor(Math.random() * buffers.length)];
  }

  playEffect(name: string, volume = 1) {
    this.play(name, { volume });
  }

  playMusic(name: string) {
    this.stop(AudioChannel.Music); // ensures only one track
    this.play(name, { volume: 1, loop: true, channel: AudioChannel.Music });
  }

  startLoop(name: string, channel: AudioChannel, volume = 1) {
    this.stop(channel); // avoid duplicates
    this.play(name, { volume, loop: true, channel: channel });
  }

  stop(channel: AudioChannel) {
    const instance = this.channels.get(channel);
    if (!instance) return;

    const { source, gain } = instance;
    const now = this.ctx.currentTime;

    gain.gain.setValueAtTime(gain.gain.value, now);
    gain.gain.linearRampToValueAtTime(0, now + 0.1);

    source.stop(now + 0.1);

    this.channels.delete(channel);
  }
}

export const audio = new AudioManager();
