<script setup lang="ts">
import { computed } from 'vue';
import { LayoutDashboard, BarChart2, LogOut, Sun, Moon } from 'lucide-vue-next';
import { useRouter, useRoute } from 'vue-router';
import AppLogo from './AppLogo.vue';
import { SessionStore } from '../storage';
import { avatarEmoji } from '../types';
import { routes } from '../constants';

const router = useRouter();
const route = useRoute();
const user = computed(() => SessionStore.user);
const isDark = computed(() => SessionStore.theme === 'DARK');

function toggleTheme() {
  const next = isDark.value ? 'LIGHT' : 'DARK';
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
    class="flex items-center justify-between px-5 py-2.5 border-b border-border bg-white backdrop-blur-sm shrink-0 sticky top-0 z-40">
    <!-- Left: Logo -->
    <AppLogo size="sm" />

    <!-- Right: Nav + Actions -->
    <div class="flex items-center gap-2">
      <button @click="router.push({ path: routes.lessons })"
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-sm font-medium transition-all" :class="isActive(routes.lessons)
          ? 'bg-surface text-foreground'
          : 'text-muted-foreground hover:text-foreground hover:bg-surface'">
        <LayoutDashboard :size="14" />
        Dashboard
      </button>

      <button
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-sm font-medium transition-all bg-white hover:bg-primary/10">
        <BarChart2 :size="14" />
        Stats
      </button>

      <!-- Divider -->
      <div class="w-px h-5 bg-border mx-1" />

      <!-- User pill (Profile) -->
      <div class="flex items-center gap-2 px-2.5 py-1.5 rounded-lg bg-surface border border-border">
        <span class="text-sm leading-none">{{ user ? avatarEmoji(user.avatar) : '👤' }}</span>
        <span class="text-sm font-medium text-foreground capitalize">{{ user?.name }}</span>
      </div>

      <!-- Theme toggle -->
      <button @click="toggleTheme"
        class="w-8 h-8 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-surface transition-all bg-white"
        :title="isDark ? 'Switch to light mode' : 'Switch to dark mode'">
        <Sun v-if="isDark" :size="16" />
        <Moon v-else :size="16" />
      </button>

      <!-- Logout -->
      <button @click="logout"
        class="w-8 h-8 flex items-center justify-center text-white rounded-lg hover:text-foreground hover:bg-surface transition-all bg-red-600"
        title="Sign out">
        <LogOut :size="15" />
      </button>
    </div>
  </header>
</template>
