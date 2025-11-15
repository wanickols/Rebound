<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import GameCanvas from "@components/GameCanvas.vue";
import { InputManager } from "@/utils/InputManager";
import { GamePayload } from "@/Game/Payload/GamePayload";
import { listen } from "@tauri-apps/api/event";
import { ScoreManager } from "@/Game/Payload/ScoreManager";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import CountdownClock from "@/components/CountdownClock.vue";

// Minimal score tracking
const score = ref({ player1: 0, player2: 0 });
const scoreText = computed(
  () => `${score.value.player1} | ${score.value.player2}`
);
const inputManager = new InputManager();
const router = useRouter();

const phase = ref<ReturnType<typeof GamePayload.from>["phase"] | null>(null);

onMounted(async () => {
  function gameLoop() {
    //inputManager.update(now);
    requestAnimationFrame(gameLoop);
  }
  gameLoop();

  await listen<GamePayload>("game-state", (event) => {
    const payload = GamePayload.from(event.payload);
    const scoreManager = ScoreManager.from(payload.score_manager);

    phase.value = payload.phase; // <-- HERE

    // Assuming exactly two teams
    const [team1, team2] = scoreManager.teams;

    score.value = {
      player1: team1.score,
      player2: team2.score,
    };
  });
});

const onPlay = async () => {
  invoke("start_game", {});
};

const goBack = () => {
  router.push("/");
};
</script>

<template>
  <div class="w-screen h-screen relative">
    <GameCanvas :inputManager="inputManager" />

    <!-- Score display -->
    <div class="absolute top-4 left-4 text-white text-xl">
      {{ scoreText }}
    </div>

    <CountdownClock :phase="phase" />

    <!-- Buttons (only in Waiting state) -->
    <div
      v-if="phase?.type === 'Waiting'"
      class="card-actions justify-center absolute bottom-4 left-1/2 -translate-x-1/2"
    >
      <button class="btn btn-secondary" @click="goBack">Back</button>
      <button class="btn btn-primary" @click="onPlay">Play</button>
    </div>
  </div>
</template>
