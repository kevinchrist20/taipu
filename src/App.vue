<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router';

</script>

<template>
  <nav>
    <RouterLink to="/">Go to Home</RouterLink>
    <RouterLink to="/lesson-area">Go to About</RouterLink>
  </nav>
  <main>
    <RouterView />
  </main>
</template>

<style>
</style>
