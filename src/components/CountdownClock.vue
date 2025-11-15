<script setup lang="ts">
import { computed } from "vue";
import type { GamePhase } from "@/Game/Payload/GamePhase";

const props = defineProps<{
  phase: GamePhase | null;
}>();

// The displayed number (null if not in countdown)
const displayNumber = computed(() => {
  if (props.phase?.type === "Countdown") {
    return Math.ceil(props.phase.time_left!);
  }
  return null;
});
</script>

<template>
  <transition name="fade-scale">
    <!-- key forces re-animation when number changes -->
    <div
      v-if="displayNumber !== null"
      :key="displayNumber"
      class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-white text-7xl font-bold pointer-events-none"
    >
      {{ displayNumber }}
    </div>
  </transition>
</template>

<style scoped>
.fade-scale-enter-active,
.fade-scale-leave-active {
  transition: opacity 0.25s ease, transform 0.6s ease;
}

.fade-scale-enter-from {
  opacity: 0;
  transform: scale(0.5);
}

.fade-scale-leave-to {
  opacity: 0;
  transform: scale(1.5);
}
</style>
