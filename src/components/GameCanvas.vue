<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";
import { GameRenderer } from "@/Game/Renderer/GameRenderer";
import { listen } from "@tauri-apps/api/event";
import { GamePayload } from "@/Game/GamePayload";
import { InputManager } from "@/utils/InputManager";

const canvas = ref<HTMLCanvasElement | null>(null);

const GAME_WIDTH = 1920;
const GAME_HEIGHT = 1080;

var renderer: GameRenderer;

const props = defineProps<{
  inputManager: InputManager; // or use the proper type
}>();

onMounted(async () => {
  if (!canvas.value) return;

  // set canvas resolution
  canvas.value.width = GAME_WIDTH;
  canvas.value.height = GAME_HEIGHT;

  window.addEventListener("resize", resizeCanvas);
  window.addEventListener("mousemove", (e) => onMouseMove(e));
  resizeCanvas(); // initial call

  const ctx = canvas.value.getContext("2d");
  if (!ctx) return;

  renderer = new GameRenderer(ctx, canvas.value);

  // listen for backend state updates
  await listen<GamePayload>("game-state", (event) => {
    const payload = GamePayload.from(event.payload);
    renderer.updateState(payload.states);
  });
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
  console.log("Current scale is: " + scale);

  canvas.value.style.width = `${GAME_WIDTH * scale}px`;
  canvas.value.style.height = `${GAME_HEIGHT * scale}px`;
}

function onMouseMove(e: MouseEvent) {
  if (!canvas.value) return;
  const rect = canvas.value.getBoundingClientRect();
  const x = e.clientX - rect.left;
  const y = e.clientY - rect.top;

  //let scale = renderer.getScale();
  //props.inputManager.setScale(scale);
  props.inputManager.handleMouseMove(x, y);
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
