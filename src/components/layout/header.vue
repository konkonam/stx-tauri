<script setup lang="ts">
</script>

<template>
  <header class="bg-primary text-white">
    <div class="container mx-auto p-4 flex items-center justify-between">
      <router-link
        to="/"
        class="flex items-center"
      >
        <img
          src="/logo.svg"
          alt="Stacks Toolbox"
          class="w-8 h-8"
        >
        <span
          class="ms-4"
        >
          Stacks Toolbox
        </span>
      </router-link>

      <router-link
        to="/settings"
      >
        Settings
      </router-link>
    </div>
  </header>
</template>
