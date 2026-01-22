<script setup lang="ts">
import { ref, onMounted } from "vue";
import { GamePayload } from "@/Game/Payload/GamePayload";
import { listen } from "@tauri-apps/api/event";
import CountdownClock from "@/components/CountdownClock.vue";
import HostLobby from "@/components/HostLobby.vue";
import GameHUD from "@/components/GameHUD.vue";
import ClientLobby from "@/components/ClientLobby.vue";

const phase = ref<ReturnType<typeof GamePayload.from>["phase"] | null>(null);

defineProps<{
  role: "host" | "client";
}>();

onMounted(async () => {
  await listen<GamePayload>("game-state", (event) => {
    const payload = GamePayload.from(event.payload);
    phase.value = payload.phase;
  });
});
</script>

<template>
  <div class="w-screen h-screen relative">
    <HostLobby v-if="phase?.type === 'Waiting' && role === 'host'" />
    <ClientLobby v-else-if="phase?.type === 'Waiting' && role === 'client'" />
    <GameHUD />
    <CountdownClock :phase="phase" />
  </div>
</template>
