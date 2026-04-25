<script setup lang="ts">
import { ref, computed, onUnmounted, watch } from "vue";
import { onMounted } from "vue";
import { audio } from "@/Game/Audio/AudioManager";
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

/// Listen for score updates from the game client
let unsubscribe: (() => void) | undefined;

onMounted(() => {
  gameClient.fxEventBus.subscribe((event) => {
    if (event.type === "GoalScored") {
      playSound();
      //playAnimation(event.team_id);
    }
  });
});

function playSound() {
  audio.playEffect("goal");
}

onUnmounted(() => {
  unsubscribe?.();
});
</script>

<template>
  <div class="absolute top-4 left-4 text-white text-xl">
    {{ scoreText }}
  </div>
</template>
