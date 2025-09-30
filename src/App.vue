<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="container mx-auto p-6 text-center">
    <h1 class="text-4xl font-bold mb-6 text-primary">Welcome to Tauri + Vue</h1>

    <div class="flex justify-center gap-8 mb-6">
      <a href="https://vite.dev" target="_blank">
        <img src="/vite.svg" class="h-16" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="h-16" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="h-16" alt="Vue logo" />
      </a>
    </div>

    <p class="text-secondary text-xl mb-4">Daisy and tailwind working?</p>

    <form class="flex justify-center gap-4 mb-4" @submit.prevent="greet">
      <input
        id="greet-input"
        v-model="name"
        placeholder="Enter a name..."
        class="input input-bordered w-64"
      />
      <button type="submit" class="btn btn-primary">Greet</button>
    </form>

    <p class="text-base-content">{{ greetMsg }}</p>
  </main>
</template>
