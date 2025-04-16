<script setup lang="ts">
import { RouterView } from 'vue-router';
import Alert from './components/alert/index.vue'

import UtilService from './services/util.service';
import { onMounted } from 'vue';

onMounted(() => {
  UtilService.checkForUpdates();
});

</script>

<template>
  <main>
    <Alert />
    <RouterView />
  </main>
</template>

<style></style>
