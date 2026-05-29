<script setup lang="ts">
import { RouterView, useRoute } from 'vue-router';
import Alert from './components/alert/index.vue'
import Header from './components/Header.vue';
import UtilService from './services/util.service';
import SettingsService from './services/settings.service';
import { onMounted, computed } from 'vue';
import { routes } from './constants';
import { SessionStore } from './storage';

function applyTheme(theme: string) {
  const normalized = theme.toUpperCase();
  SessionStore.setTheme(normalized);
  document.documentElement.classList.toggle('dark', normalized === 'DARK');
}

onMounted(() => {
  UtilService.checkForUpdates();

  SettingsService.getAppTheme()
    .then((theme) => applyTheme(theme))
    .catch((error) => {
      console.error('Failed to load theme settings:', error);
      applyTheme('LIGHT');
    });
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
