<script setup lang="ts">
import { ref, computed } from "vue";
import GameCanvas from "@components/GameCanvas.vue";
// Minimal score tracking
const score = ref({ player1: 0, player2: 0 });
const scoreText = computed(
  () => `${score.value.player1} | ${score.value.player2}`
);
</script>

<template>
  <div class="w-screen h-screen relative">
    <!-- Game canvas -->
    <GameCanvas />

    <!-- HUD overlay -->
    <div class="absolute top-4 left-4 text-white text-xl">
      {{ scoreText }}
    </div>
  </div>
</template>
