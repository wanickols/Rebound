<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import NumberSelector from "@/components/NumberSelector.vue";

const router = useRouter();
const playerCount = ref(1);
const scoreCount = ref(1);

const onPlay = () => {
  console.log(`Starting game with ${playerCount.value} players`);
  router.push("/game");
};

const goBack = () => {
  router.push("/");
};
</script>

<template>
  <div class="flex flex-col items-center justify-center min-h-screen">
    <div class="card w-96 shadow-xl">
      <div class="card-body items-center text-center">
        <h2 class="card-title mb-4">Game Setup</h2>

        <NumberSelector
          v-model="playerCount"
          :min="1"
          :default-value="2"
          :max="8"
          label="Players:"
          class="mb-6"
        />
        <NumberSelector
          v-model="scoreCount"
          :min="1"
          :default-value="5"
          :max="99"
          label="Score to Win:"
          class="mb-6"
        />

        <div class="card-actions justify-center">
          <button class="btn btn-secondary" @click="goBack">Back</button>
          <button class="btn btn-primary" @click="onPlay">Play</button>
        </div>
      </div>
    </div>
  </div>
</template>
