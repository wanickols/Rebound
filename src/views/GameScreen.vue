<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import { GamePayload } from "@/Game/Backend/Payload/GamePayload";
import CountdownClock from "@/components/CountdownClock.vue";
import HostLobby from "@/components/HostLobby.vue";
import GameHUD from "@/components/GameHUD.vue";
import ClientLobby from "@/components/ClientLobby.vue";
import { gameClient } from "@/Game/Backend/GameClient";

const phase = ref<ReturnType<typeof GamePayload.from>["phase"] | null>(null);
const joinedPlayers = ref<number>(0);
const expectedPlayers = ref<number>(0);

defineProps<{
  role: "host" | "client";
}>();

onMounted(() => {
  gameClient.start();
});

watch(
  () => gameClient.snapshot.phase,
  (bphase) => {
    if (!bphase) return;

    phase.value = bphase;
    joinedPlayers.value = gameClient.snapshot.lobby_state?.players.length || 0;
    expectedPlayers.value =
      gameClient.snapshot.lobby_state?.expected_players || 0;
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
    <div v-if="phase?.type === 'Waiting'">
      {{ joinedPlayers }} / {{ expectedPlayers }} players joined
    </div>
    <GameHUD />
    <CountdownClock :phase="phase" />
  </div>
</template>
