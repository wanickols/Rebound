<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref } from "vue";
import { bus } from "@/utils/EventBus";
import { ControllerManager, GamepadData } from "@/Game/Input/ControllerManager";
import { InputManager } from "@/Game/Input/InputManager";
import ScoreCounter from "./ScoreCounter.vue";
import GameCanvas from "@/components/GameCanvas.vue";

const controllerManager = ref<ControllerManager | null>(null);
//const keyboardManager = new KeyboardManager();
const inputManager = ref<InputManager | null>(null);

const POLL_INTERVAL = 60; // ms, adjust if needed
let intervalId: number;

onMounted(() => {
  inputManager.value = new InputManager();
  controllerManager.value = new ControllerManager();
  bus.on("gamepadEvent", onGamepadEvent);

  bus.on("controllerConnected", onControllerAvailable);
  bus.on("controllerDisconnected", onControllerRemoved);

  intervalId = window.setInterval(() => {
    controllerManager.value?.pollGamepads();
  }, POLL_INTERVAL);
});

onBeforeUnmount(() => {
  bus.off("gamepadEvent", onGamepadEvent);
  bus.off("controllerConnected", onControllerAvailable);
  bus.off("controllerDisconnected", onControllerRemoved);

  clearInterval(intervalId);
  inputManager.value?.destroy();
  controllerManager.value?.destroy();
});

function onGamepadEvent(gamepad: GamepadData) {
  inputManager.value?.handleGamepadEvent(gamepad);
}

function onControllerAvailable(index: number) {
  inputManager.value?.onControllerConnected(index);
}

function onControllerRemoved(index: number) {
  inputManager.value?.onControllerDisconnected(index);
}
</script>

<template>
  <GameCanvas :inputManager="inputManager" />
  <ScoreCounter />
</template>
