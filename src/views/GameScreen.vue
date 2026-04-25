<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import { GamePayload } from "@/Game/Payload/GamePayload";
import CountdownClock from "@/components/CountdownClock.vue";
import HostLobby from "@/components/HostLobby.vue";
import GameHUD from "@/components/GameHUD.vue";
import ClientLobby from "@/components/ClientLobby.vue";
import { gameClient } from "@/Game/Payload/GameClient";

const phase = ref<ReturnType<typeof GamePayload.from>["phase"] | null>(null);

defineProps<{
  role: "host" | "client";
}>();

onMounted(() => {
  gameClient.start();
});

watch(
  () => gameClient.state.value,
  (payload) => {
    if (!payload) return;

    phase.value = payload.phase;
  },
  { immediate: true },
);

onUnmounted(() => {
  gameClient.stop();
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
