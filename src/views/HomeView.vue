<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { User } from '../types/bindings';
import { avatarEmoji } from '../types';
import UserService from '../services/user.service';
import router from '../router';
import { SessionStore } from '../storage';
import { routes } from '../constants';
import AppLogo from '../components/AppLogo.vue';

const users = ref<User[]>([]);

function getAvatar(user: User): string {
  return avatarEmoji(user.avatar);
}

function selectUser(user: User) {
  SessionStore.setUser(user);
  router.push({ path: routes.lessons });
}

onMounted(async () => {
  try {
    users.value = await UserService.getUsers();
  } catch (error) {
    console.error(error);
  }
});
</script>

<template>
  <div class="min-h-screen bg-background flex flex-col items-center justify-center px-6">
    <!-- Brand -->
    <div class="flex flex-col items-center mb-10">
      <AppLogo size="lg" class="mb-4" />
      <p class="text-muted-foreground text-sm mt-3 text-center max-w-xs font-display">
        Focus, practice, and master typing. Select your profile to continue.
      </p>
    </div>

    <!-- Profile Cards -->
    <div class="flex flex-wrap gap-4 justify-center max-w-xl">
      <button
        v-for="user in users"
        :key="user.id"
        @click="selectUser(user)"
        class="flex flex-col items-center gap-2 w-36 py-6 px-4 rounded-2xl bg-card border border-border hover:border-primary hover:bg-surface-elevated transition-all"
      >
        <span class="text-4xl leading-none">{{ getAvatar(user) }}</span>
        <span class="text-sm font-medium text-foreground capitalize mt-1 font-sans">{{ user.name }}</span>
      </button>

      <!-- New Profile -->
      <button
        @click="router.push({ path: routes.createAccount })"
        class="flex flex-col items-center gap-2 w-36 py-6 px-4 rounded-2xl bg-card border-2 border-dashed border-border hover:border-primary hover:bg-surface-elevated transition-all"
      >
        <div class="w-10 h-10 rounded-full border-dashed border-2 border-border flex items-center justify-center text-muted-foreground text-2xl leading-none">
          +
        </div>
        <span class="text-sm font-medium text-muted-foreground mt-1 font-sans">New Profile</span>
      </button>
    </div>
  </div>
</template>