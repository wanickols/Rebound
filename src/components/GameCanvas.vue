<script setup lang="ts">
import { ref, onMounted } from "vue";
import { InputManager } from "@/utils/InputManager";
import { GameRenderer, type GameState } from "@/utils/GameRenderer";
import { listen } from "@tauri-apps/api/event";

const canvas = ref<HTMLCanvasElement | null>(null);
const inputManager = new InputManager();

onMounted(async () => {
  if (!canvas.value) return;

  // set canvas resolution
  canvas.value.width = canvas.value.clientWidth;
  canvas.value.height = canvas.value.clientHeight;

  const ctx = canvas.value.getContext("2d");
  if (!ctx) return;

  const renderer = new GameRenderer(
    ctx,
    canvas.value.width,
    canvas.value.height
  );

  // listen for backend state updates
  await listen<GameState>("game-state", (event) => {
    renderer.updateState(event.payload);
  });

  // optional debug loop for input
  function loop() {
    console.log(inputManager.getState());
    requestAnimationFrame(loop);
  }
  loop();
});
</script>

<template>
  <canvas ref="canvas" class="w-full h-full bg-black" tabindex="0"></canvas>
</template>
