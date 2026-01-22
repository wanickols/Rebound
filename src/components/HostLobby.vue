<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";

const router = useRouter();

const onPlay = async () => {
  invoke("start_game", {});
};

const goBack = () => {
  invoke("quit_game", {});
  router.push("/");
};
</script>

<template>
  <!-- Buttons (only in Waiting state) -->
  <div
    class="card-actions justify-center absolute bottom-4 left-1/2 -translate-x-1/2"
  >
    <button class="btn btn-secondary" @click="goBack">Back</button>
    <button class="btn btn-primary" @click="onPlay">Play</button>
  </div>
</template>
