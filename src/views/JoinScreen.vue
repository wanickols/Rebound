<template>
  <div class="min-h-screen flex items-center justify-center">
    <div class="card w-80 bg-base-200 shadow-xl rounded-2xl p-6">
      <h1 class="card-title justify-center mb-4">{{ title }}</h1>

      <!-- Port input / Join button -->
      <div v-if="!joined">
        <div class="form-control">
          <label class="label" for="port">
            <span class="label-text">Server Port</span>
          </label>
          <input
            id="port"
            v-model="port"
            type="number"
            placeholder="8080"
            class="input input-bordered w-full"
          />
        </div>
        <button class="btn btn-primary w-full mt-4" @click="joinLobby">
          Join
        </button>
        <button class="btn btn-secondary w-full mt-4" @click="backToHome">
          Back
        </button>
      </div>

      <!-- Waiting / Loading -->
      <div v-else class="flex flex-col items-center justify-center gap-4">
        <LoadingCounter />
        <button class="btn btn-secondary w-full mt-4" @click="backToJoin">
          Back
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import LoadingCounter from "@/components/LoadingCounter.vue";
import router from "@/router";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { onMounted, ref } from "vue";

const port = ref<number | null>(null);
const joined = ref(false);
const title = ref("Join Lobby");

onMounted(async () => {
  await listen<Number>("joined", () => {
    console.log("Pushin");
    router.push({
      name: "game",
      params: { role: "client" },
    });
  });
});

function joinLobby() {
  joined.value = true;
  title.value = "Waiting for host...";
  // ultra-rough for now: just log it
  console.log("Joining on port:", port.value);
  invoke("join_game", { port: 8080 })
    .then(() => {
      console.log("Game hosting started!");
    })
    .catch((err) => {
      console.error("Failed to host game:", err);
    });
}

function backToJoin() {
  joined.value = false;
  title.value = "Join Lobby";
}

function backToHome() {
  router.push("/");
}
</script>
