<script setup lang="ts">
import { useRouter } from "vue-router";
const router = useRouter();
import { exit } from "@tauri-apps/plugin-process";

function onHost() {
  router.push("/gameSetup");
}

function onJoin() {
  router.push("/joinGame");
}

async function onQuit() {
  console.log("Quit pressed");
  await exit(1);
}
</script>

<template>
  <div class="w-screen h-screen flex flex-col justify-center items-center">
    <h1 class="text-6xl font-bold mb-12">Rebound</h1>

    <div class="flex flex-col gap-6">
      <button class="btn btn-primary btn-lg" @click="onHost">Host</button>
      <button class="btn btn-primary btn-lg" @click="onJoin">Join</button>
      <button class="btn btn-secondary btn-lg" @click="onQuit">Quit</button>
    </div>
  </div>
</template>
