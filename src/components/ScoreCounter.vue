<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { gameClient } from "@/Game/Payload/GameClient";

//Score
const score = ref({ player1: 0, player2: 0 });

const scoreText = computed(
  () => `${score.value.player1} | ${score.value.player2}`,
);

/// Update score when game client state changes
watch(
  () => gameClient.state.value,
  (payload) => {
    if (!payload) return;

    const [team1, team2] = payload.score_manager.teams;

    score.value = {
      player1: team1.score,
      player2: team2.score,
    };
  },
  { immediate: true },
);
</script>

<template>
  <div class="absolute top-4 left-4 text-white text-xl">
    {{ scoreText }}
  </div>
</template>
