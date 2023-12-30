<script setup lang="ts">
  const projects = [
      'Project 1',
      'Project 2',
      'Project 3',
  ];
</script>

<template>
  <div class="flex flex-col">
    <div v-for="project in projects">
      <router-link to="/project/test">
        {{ project }}
      </router-link>
    </div>
  </div>
</template>
