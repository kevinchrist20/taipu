<script setup lang="ts">
import { RouterView, useRoute } from 'vue-router';
import Alert from './components/alert/index.vue'
import Header from './components/Header.vue';
import UtilService from './services/util.service';
import { onMounted, computed } from 'vue';
import { routes } from './constants';
import { SessionStore } from './storage';

onMounted(() => {
  UtilService.checkForUpdates();
  // Apply saved theme on app start
  const isDark = SessionStore.theme === 'DARK';
  document.documentElement.classList.toggle('dark', isDark);
});

const route = useRoute();

// Show the shell nav on authenticated views only
const showShell = computed(() =>
  route.path !== routes.home && route.path !== routes.createAccount
);
</script>

<template>
  <main class="flex flex-col min-h-screen bg-background text-foreground">
    <Alert />
    <Header v-if="showShell" />
    <RouterView />
  </main>
</template>
