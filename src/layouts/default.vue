<script setup lang="ts">
  const route = useRoute();
</script>

<template>
  <div class="min-h-screen flex flex-col">
    <Header />

    <main class="container mx-auto px-4 pt-4 bg-black:90 grow text-white">
      <Controls
        v-if="route.fullPath !== '/'"
      />

      <RouterView />
    </main>
  </div>
</template>
