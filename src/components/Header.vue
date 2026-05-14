<script setup lang="ts">
import { computed } from 'vue';
import { LayoutDashboard, User as UserIcon, LogOut, Sun, Moon } from 'lucide-vue-next';
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
  <header class="flex items-center justify-between px-5 py-2.5 border-b border-border bg-background/90 backdrop-blur-sm shrink-0 sticky top-0 z-40">
    <!-- Left: Logo -->
    <AppLogo size="sm" />

    <!-- Center: Nav tabs -->
    <nav class="flex items-center gap-0.5 bg-surface rounded-xl p-1">
      <button
        @click="router.push({ path: routes.lessons })"
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-sm font-medium transition-all"
        :class="isActive(routes.lessons)
          ? 'bg-surface-elevated text-foreground shadow-sm'
          : 'text-muted-foreground hover:text-foreground'"
      >
        <LayoutDashboard :size="14" />
        Dashboard
      </button>

      <button
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-sm font-medium text-muted-foreground hover:text-foreground transition-all"
      >
        <UserIcon :size="14" />
        Profile
      </button>
    </nav>

    <!-- Right: Actions -->
    <div class="flex items-center gap-2">
      <!-- Theme toggle -->
      <button
        @click="toggleTheme"
        class="w-8 h-8 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-surface transition-all"
        :title="isDark ? 'Switch to light mode' : 'Switch to dark mode'"
      >
        <Sun v-if="isDark" :size="16" />
        <Moon v-else :size="16" />
      </button>

      <!-- User pill -->
      <div class="flex items-center gap-2 px-2.5 py-1.5 rounded-lg bg-surface border border-border">
        <span class="text-sm leading-none">{{ user ? avatarEmoji(user.avatar) : '👤' }}</span>
        <span class="text-sm font-medium text-foreground capitalize">{{ user?.name }}</span>
      </div>

      <!-- Logout -->
      <button
        @click="logout"
        class="w-8 h-8 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-surface transition-all"
        title="Sign out"
      >
        <LogOut :size="15" />
      </button>
    </div>
  </header>
</template>
