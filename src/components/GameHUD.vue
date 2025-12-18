<script setup lang="ts">
import { onMounted, onBeforeUnmount } from "vue";
import { bus } from "@/utils/EventBus";
import { Player, playerManager } from "@/Game/Input/PlayerManager";
import { ControllerManager, GamepadData } from "@/Game/Input/ControllerManager";
import { KeyboardManager } from "@/Game/Input/KeyboardManager";
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
  bus.on("keyboardEvent", onKeyboardEvent);

  bus.on("controllerAvailable", onControllerAvailable);
  bus.on("controllerRemoved", onControllerRemoved);

  intervalId = window.setInterval(() => {
    controllerManager.pollGamepads();
  }, POLL_INTERVAL);
});

onBeforeUnmount(() => {
  bus.off("gamepadEvent", onGamepadEvent);
  bus.off("keyboardEvent", onKeyboardEvent);
  clearInterval(intervalId);
});

function onGamepadEvent(gamepad: GamepadData) {
  inputManager.handleGamepadEvent(gamepad);
}

function onKeyboardEvent(player: Player) {
  inputManager.handleKeyboardEvent(player);
}

function onControllerAvailable(index: number) {
  playerManager.assignController(index);
}

function onControllerRemoved(index: number) {
  playerManager.removeController(index);
}
</script>

<template>
  <GameCanvas :input-manager="inputManager" />
  <ScoreCounter />
</template>
