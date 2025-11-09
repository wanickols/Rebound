<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import GameCanvas from "@components/GameCanvas.vue";
import { InputManager } from "@/utils/InputManager";
import { GamePayload } from "@/Game/GamePayload";
import { listen } from "@tauri-apps/api/event";
import { ScoreManager } from "@/Game/ScoreManager";
import { Team } from "@/Game/Team";

// Minimal score tracking
const score = ref({ player1: 0, player2: 0 });
const scoreText = computed(
  () => `${score.value.player1} | ${score.value.player2}`
);
const inputManager = new InputManager();

onMounted(async () => {
  function gameLoop() {
    const now = performance.now();
    //inputManager.update(now);
    requestAnimationFrame(gameLoop);
  }
  gameLoop();

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
  <div class="w-screen h-screen relative">
    <!-- Game canvas -->
    <GameCanvas :inputManager="inputManager" />

    <!-- HUD overlay -->
    <div class="absolute top-4 left-4 text-white text-xl">
      {{ scoreText }}
    </div>
  </div>
</template>
