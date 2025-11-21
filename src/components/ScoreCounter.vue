<script setup lang="ts">
import { ref, computed } from "vue";
import { onMounted } from "vue";
import { GamePayload } from "@/Game/Payload/GamePayload";
import { listen } from "@tauri-apps/api/event";
import { ScoreManager } from "@/Game/Payload/ScoreManager";

//Score
const score = ref({ player1: 0, player2: 0 });

const scoreText = computed(
  () => `${score.value.player1} | ${score.value.player2}`
);

onMounted(async () => {
  await listen<GamePayload>("game-state", (event) => {
    const payload = GamePayload.from(event.payload);
    const scoreManager = ScoreManager.from(payload.score_manager);

    // Assuming exactly two teams
    const [team1, team2] = scoreManager.teams;

    score.value = {
      player1: team1.score,
      player2: team2.score,
    };
  });
});
</script>

<template>
  <div class="absolute top-4 left-4 text-white text-xl">
    {{ scoreText }}
  </div>
</template>
