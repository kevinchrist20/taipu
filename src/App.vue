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

<style>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
