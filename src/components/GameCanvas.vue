<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";
import { InputManager } from "@/utils/InputManager";
import { GameRenderer } from "@/Renderer/GameRenderer";
import { listen } from "@tauri-apps/api/event";

const canvas = ref<HTMLCanvasElement | null>(null);
const inputManager = new InputManager();

const GAME_WIDTH = 1920;
const GAME_HEIGHT = 1080;

onMounted(async () => {
  if (!canvas.value) return;

  // set canvas resolution
  canvas.value.width = GAME_WIDTH;
  canvas.value.height = GAME_HEIGHT;

  window.addEventListener("resize", resizeCanvas);
  resizeCanvas(); // initial call

  const ctx = canvas.value.getContext("2d");
  if (!ctx) return;

  const renderer = new GameRenderer(
    ctx,
    canvas.value.width,
    canvas.value.height
  );

  // listen for backend state updates
  await listen<State[]>("game-state", (event) => {
    renderer.updateState(event.payload);
  });

  // optional debug loop for input
  function loop() {
    requestAnimationFrame(loop);
  }
  loop();
});

onBeforeUnmount(() => {
  window.removeEventListener("resize", resizeCanvas);
});

function resizeCanvas() {
  if (!canvas.value) return;

  const { innerWidth: w, innerHeight: h } = window;
  const aspect = 16 / 9;

  let scale;
  if (w / h > aspect) {
    scale = h / GAME_HEIGHT;
  } else {
    scale = w / GAME_WIDTH;
  }

  canvas.value.style.width = `${GAME_WIDTH * scale}px`;
  canvas.value.style.height = `${GAME_HEIGHT * scale}px`;
}
</script>

<template>
  <canvas ref="canvas" tabindex="0"></canvas>
</template>

<style scoped>
canvas {
  outline: none;
  will-change: transform;
  backface-visibility: hidden;
}
body,
html {
  margin: 0;
  padding: 0;
  outline: none; /* prevents any focus outlines on body */
  border: 0; /* remove any default borders */
  background: black; /* ensures black bars */
  overflow: hidden; /* removes scrollbars */
}
</style>
