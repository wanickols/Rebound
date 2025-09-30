<script setup>
const savedPath = sessionStorage.getItem("redirect-path");
if (savedPath) {
  sessionStorage.removeItem("redirect-path");
  window.history.replaceState({}, "", savedPath);
}
</script>

<template>
  <div class="min-h-screen flex flex-col">
    <Navbar />
    <!-- This fills the vertical space between nav and footer -->
    <main class="flex-grow flex items-center justify-center">
      <router-view />
    </main>
  </div>
</template>
