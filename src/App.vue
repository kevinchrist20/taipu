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
    <router-view v-slot="{ Component, route }">
      <keep-alive :include="['Lessons', 'Category Lessons']">
        <component :is="Component" :key="route.path" />
      </keep-alive>
    </router-view>
  </main>
</template>

<style>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
