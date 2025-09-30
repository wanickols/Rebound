<script setup lang="ts">
import { useRouter } from "vue-router";
const router = useRouter();

function onPlay() {
  router.push("/game");
}

function onQuit() {
  console.log("Quit pressed");
  // TODO: exit app (Tauri command or window close)
}
</script>

<template>
  <div class="w-screen h-screen flex flex-col justify-center items-center">
    <h1 class="text-6xl font-bold mb-12">Rebound</h1>

    <div class="flex flex-col gap-6">
      <button class="btn btn-primary btn-lg" @click="onPlay">Play</button>
      <button class="btn btn-secondary btn-lg" @click="onQuit">Quit</button>
    </div>
  </div>
</template>
