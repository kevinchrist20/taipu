<script setup lang="ts">
import { computed } from 'vue';
import { LayoutDashboard, BarChart2, LogOut, Sun, Moon, Settings } from 'lucide-vue-next';
import { useRouter, useRoute } from 'vue-router';
import AppLogo from './AppLogo.vue';
import { SessionStore } from '../storage';
import { avatarEmoji } from '../types';
import { routes } from '../constants';
import AppButton from './AppButton.vue';
import SettingsService from '../services/settings.service';

const router = useRouter();
const route = useRoute();
const user = computed(() => SessionStore.user);
const isDark = computed(() => SessionStore.theme === 'DARK');

async function toggleTheme() {
  const next = isDark.value ? 'LIGHT' : 'DARK';

  try {
    await SettingsService.setAppTheme(next);
  } catch (error) {
    console.error('Failed to save theme settings:', error);
  }

  SessionStore.setTheme(next);
  document.documentElement.classList.toggle('dark', next === 'DARK');
}

function logout() {
  SessionStore.clearUser();
  router.push({ path: routes.home });
}

function isActive(path: string) {
  return route.path === path || route.path.startsWith(path + '/');
}
</script>

<template>
  <header
    class="flex items-center justify-between px-5 py-2.5 border-b border-border bg-surface-elevated backdrop-blur-sm shrink-0 sticky top-0 z-40">
    <!-- Left: Logo -->
    <AppLogo size="sm" />

    <!-- Right: Nav + Actions -->
    <div class="flex items-center gap-2">
      <AppButton size="sm" @click="router.push({ path: routes.lessons })"
        :variant="isActive(routes.lessons) ? 'primary' : 'secondary'">
        <LayoutDashboard :size="14" />
        Dashboard
      </AppButton>

      <AppButton size="sm" @click="router.push({ path: routes.stats })"
        :variant="isActive(routes.stats) ? 'primary' : 'secondary'">
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

      <AppButton size="sm" @click="router.push({ path: routes.settings })"
        :variant="isActive(routes.settings) ? 'primary' : 'secondary'">
        <Settings :size="14" />
      </AppButton>

      <!-- Theme toggle -->
      <AppButton @click="toggleTheme" variant="secondary" size="sm"
        :title="isDark ? 'Switch to light mode' : 'Switch to dark mode'">
        <Sun v-if="isDark" :size="16" />
        <Moon v-else :size="16" />
      </AppButton>

      <!-- Logout -->
      <AppButton variant="danger" size="sm" @click="logout" title="Sign out">
        <LogOut :size="15" />
      </AppButton>
    </div>
  </header>
</template>
