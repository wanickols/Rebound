<template>
  <div
    class="min-h-screen flex items-center justify-center bg-gray-900 text-gray-100"
  >
    <div class="bg-gray-800 p-6 rounded-2xl shadow-lg w-80">
      <h1 class="text-xl font-semibold mb-4 text-center">Join Game</h1>

      <label class="block text-sm mb-2" for="port">Server Port</label>
      <input
        id="port"
        v-model="port"
        type="number"
        placeholder="3000"
        class="w-full px-3 py-2 rounded-lg bg-gray-700 text-gray-100 focus:outline-none focus:ring"
      />

      <button
        class="mt-4 w-full py-2 rounded-xl bg-blue-600 hover:bg-blue-500 transition"
        @click="join"
      >
        Join
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const port = ref<number | null>(null);

function join() {
  // ultra-rough for now: just log it
  console.log("Joining on port:", port.value);

  // later: emit event / call network manager
}
</script>
