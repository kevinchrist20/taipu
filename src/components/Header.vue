<script setup lang="ts">
import { BarChart2, LayoutDashboard, LogOut, Moon, Settings, Sun } from '@lucide/vue'
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { routes } from '../constants'
import SettingsService from '../services/settings.service'
import { SessionStore } from '../storage'
import { avatarEmoji } from '../types'
import { normalizeTheme } from '../utils/ui-formatters'
import AppButton from './AppButton.vue'
import AppLogo from './AppLogo.vue'

const router = useRouter()
const route = useRoute()
const user = computed(() => SessionStore.user)
const isDark = computed(() => SessionStore.theme === 'DARK')

async function toggleTheme() {
  const next = normalizeTheme(isDark.value ? 'LIGHT' : 'DARK')

  try {
    await SettingsService.setAppTheme(next)
  }
  catch (error) {
    console.error('Failed to save theme settings:', error)
  }

  SessionStore.setTheme(next)
  document.documentElement.classList.toggle('dark', next === 'DARK')
}

function logout() {
  SessionStore.clearUser()
  router.push({ path: routes.home })
}

function isActive(path: string) {
  return route.path === path || route.path.startsWith(`${path}/`)
}
</script>

<template>
  <header
    class="flex items-center justify-between px-5 py-2.5 border-b border-border bg-surface-elevated backdrop-blur-sm shrink-0 sticky top-0 z-40"
  >
    <!-- Left: Logo -->
    <AppLogo size="sm" />

    <!-- Right: Nav + Actions -->
    <div class="flex items-center gap-2">
      <AppButton
        size="sm" :variant="isActive(routes.lessons) ? 'primary' : 'secondary'"
        @click="router.push({ path: routes.lessons })"
      >
        <LayoutDashboard :size="14" />
        Dashboard
      </AppButton>

      <AppButton
        size="sm" :variant="isActive(routes.stats) ? 'primary' : 'secondary'"
        @click="router.push({ path: routes.stats })"
      >
        <BarChart2 :size="14" />
        Stats
      </AppButton>

      <!-- User pill (Profile) -->
      <AppButton variant="ghost" size="sm" @click="router.push({ path: routes.settings })">
        <span class="text-sm leading-none">{{ user ? avatarEmoji(user.avatar) : '👤' }}</span>
        <span class="text-sm font-medium text-foreground capitalize">{{ user?.name }}</span>
      </AppButton>

      <!-- Divider -->
      <div class="w-px h-5 bg-border mx-1" />

      <AppButton
        size="sm" :variant="isActive(routes.settings) ? 'primary' : 'secondary'"
        @click="router.push({ path: routes.settings })"
      >
        <Settings :size="14" />
      </AppButton>

      <!-- Theme toggle -->
      <AppButton
        variant="secondary" size="sm" :title="isDark ? 'Switch to light mode' : 'Switch to dark mode'"
        @click="toggleTheme"
      >
        <Sun v-if="isDark" :size="16" />
        <Moon v-else :size="16" />
      </AppButton>

      <!-- Logout -->
      <AppButton variant="danger" size="sm" title="Sign out" @click="logout">
        <LogOut :size="15" />
      </AppButton>
    </div>
  </header>
</template>
