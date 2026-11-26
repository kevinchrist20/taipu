<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { RouterView, useRoute } from 'vue-router'
import Alert from './components/alert/index.vue'
import Header from './components/Header.vue'
import UpdateDialog from './components/UpdateDialog.vue'
import { useUpdater } from './composables/useUpdater'
import { routes } from './constants'
import SettingsService from './services/settings.service'
import { SessionStore } from './storage'
import { normalizeTheme } from './utils/ui-formatters'

const updater = useUpdater()

function applyTheme(theme: string) {
  const normalized = normalizeTheme(theme)
  SessionStore.setTheme(normalized)
  document.documentElement.classList.toggle('dark', normalized === 'DARK')
}

onMounted(() => {
  void updater.initialize()

  SettingsService.getAppTheme()
    .then(theme => applyTheme(theme))
    .catch((error) => {
      console.error('Failed to load theme settings:', error)
      applyTheme('LIGHT')
    })
})

const route = useRoute()

// Show the shell nav on authenticated views only
const showShell = computed(() =>
  route.path !== routes.home && route.path !== routes.createAccount,
)
</script>

<template>
  <main class="flex flex-col min-h-screen bg-background text-foreground">
    <Alert />
    <Header v-if="showShell" />
    <RouterView />
  </main>
  <UpdateDialog />
</template>
