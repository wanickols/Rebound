<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { gameClient } from "@/Game/Backend/GameClient";
import { ScoreManager } from "@/Game/Backend/Payload/ScoreManager";

//Score
const score = ref({ player1: 0, player2: 0 });

const scoreText = computed(
  () => `${score.value.player1} | ${score.value.player2}`,
);

/// Update score when game client state changes
watch(
  () => gameClient.snapshot.scoreManager,
  (scoreManager) => onScore(scoreManager || null),
  { immediate: true },
);

function onScore(scoreManager: ScoreManager | null) {
  if (!scoreManager) return;

  const [team1, team2] = scoreManager.teams;

  score.value = {
    player1: team1.score,
    player2: team2.score,
  };
}
</script>

<template>
  <div class="absolute top-4 left-4 text-white text-xl">
    {{ scoreText }}
  </div>
</template>
