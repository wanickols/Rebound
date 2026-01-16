<template>
  <div class="flex items-center justify-center space-x-2">
    <span
      class="dot bg-primary rounded-full w-4 h-4 animate-bounce-delay-0"
    ></span>
    <span
      class="dot bg-primary rounded-full w-4 h-4 animate-bounce-delay-1"
    ></span>
    <span
      class="dot bg-primary rounded-full w-4 h-4 animate-bounce-delay-2"
    ></span>
  </div>
</template>

<script setup>
// placeholder, no JS needed for simple animation
</script>

<style scoped>
@keyframes bounce-delay {
  0%,
  80%,
  100% {
    transform: scale(0);
  }
  40% {
    transform: scale(1);
  }
}

.dot {
  display: inline-block;
  animation: bounce-delay 1.4s infinite ease-in-out;
}

/* Stagger the delays for each dot */
.animate-bounce-delay-0 {
  animation-delay: 0s;
}
.animate-bounce-delay-1 {
  animation-delay: 0.2s;
}
.animate-bounce-delay-2 {
  animation-delay: 0.4s;
}
</style>
