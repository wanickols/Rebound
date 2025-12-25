<script setup lang="ts">
import { onMounted, onBeforeUnmount } from "vue";
import { bus } from "@/utils/EventBus";
import { ControllerManager, GamepadData } from "@/Game/Input/ControllerManager";
import { InputManager } from "@/Game/Input/InputManager";
import ScoreCounter from "./ScoreCounter.vue";
import GameCanvas from "@/components/GameCanvas.vue";

const controllerManager = new ControllerManager();
//const keyboardManager = new KeyboardManager();
const inputManager = new InputManager();

const POLL_INTERVAL = 60; // ms, adjust if needed
let intervalId: number;

onMounted(() => {
  bus.on("gamepadEvent", onGamepadEvent);

  bus.on("controllerConnected", onControllerAvailable);
  bus.on("controllerDisconnected", onControllerRemoved);

  intervalId = window.setInterval(() => {
    controllerManager.pollGamepads();
  }, POLL_INTERVAL);
});

onBeforeUnmount(() => {
  bus.off("gamepadEvent", onGamepadEvent);
  clearInterval(intervalId);
});

function onGamepadEvent(gamepad: GamepadData) {
  inputManager.handleGamepadEvent(gamepad);
}

function onControllerAvailable(index: number) {
  inputManager.onControllerConnected(index);
}

function onControllerRemoved(index: number) {
  inputManager.onControllerDisconnected(index);
}
</script>

<template>
  <GameCanvas :input-manager="inputManager" />
  <ScoreCounter />
</template>
