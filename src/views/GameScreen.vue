<script setup lang="ts">
import { ref, onMounted } from "vue";
import { GamePayload } from "@/Game/Payload/GamePayload";
import { listen } from "@tauri-apps/api/event";
import CountdownClock from "@/components/CountdownClock.vue";
import Lobby from "@/components/Lobby.vue";
import GameHUD from "@/components/GameHUD.vue";

const phase = ref<ReturnType<typeof GamePayload.from>["phase"] | null>(null);

onMounted(async () => {
  await listen<GamePayload>("game-state", (event) => {
    const payload = GamePayload.from(event.payload);
    phase.value = payload.phase;
  });
});
</script>

<template>
  <div class="w-screen h-screen relative">
    <Lobby v-if="phase?.type === 'Waiting'" />
    <GameHUD />
    <CountdownClock :phase="phase" />
  </div>
</template>
