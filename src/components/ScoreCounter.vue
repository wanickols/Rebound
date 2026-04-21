<script setup lang="ts">
import { ref, computed } from "vue";
import { onMounted } from "vue";
import { GamePayload } from "@/Game/Payload/GamePayload";
import { listen } from "@tauri-apps/api/event";
import { ScoreManager } from "@/Game/Payload/ScoreManager";
import { audio } from "@/Game/Audio/AudioManager";

//Score
const score = ref({ player1: 0, player2: 0 });

const scoreText = computed(
  () => `${score.value.player1} | ${score.value.player2}`,
);

let prevScore = { player1: 0, player2: 0 };

onMounted(async () => {
  await listen<GamePayload>("game-state", (event) => {
    const payload = GamePayload.from(event.payload);
    const scoreManager = ScoreManager.from(payload.score_manager);

    const [team1, team2] = scoreManager.teams;

    const newScore = {
      player1: team1.score,
      player2: team2.score,
    };

    const scored =
      newScore.player1 !== prevScore.player1 ||
      newScore.player2 !== prevScore.player2;

    if (scored) {
      audio.playEffect("goal");
    }

    score.value = newScore;
    prevScore = { ...newScore }; // only once
  });
});
</script>

<template>
  <div class="absolute top-4 left-4 text-white text-xl">
    {{ scoreText }}
  </div>
</template>
