<script setup lang="ts">
import { ref } from 'vue';
import { AVATARS, difficultyOptions, languageOptions } from '../types';
import UserService from '../services/user.service';
import useAlert from '../composables/useAlert';
import router from '../router';
import AppLogo from '../components/AppLogo.vue';
import AppButton from '../components/AppButton.vue';

const selectedAvatar = ref(0);
const name = ref('');
const language = ref('ENGLISH');
const lessonDifficulty = ref('BEGINNER');
const isLoading = ref(false);

function createAccount() {
  if (!name.value.trim()) {
    useAlert().setAlert({ message: 'Name is required', type: 'danger' });
    return;
  }

  isLoading.value = true;

  UserService.createUser({
    name: name.value.trim(),
    avatar: AVATARS[selectedAvatar.value].slug,
    language: language.value,
    lessonDifficulty: lessonDifficulty.value,
  }).then(() => {
    useAlert().setAlert({ message: 'Profile created!', type: 'success' });
    router.back();
  }).catch((error) => {
    useAlert().setAlert({ message: error, type: 'danger' });
  }).finally(() => {
    isLoading.value = false;
  });
}
</script>

<template>
  <div class="min-h-screen bg-background flex flex-col">
    <!-- Header -->
    <header class="flex items-center justify-between px-5 py-2.5 border-b border-border bg-background/90 backdrop-blur-sm sticky top-0 z-40">
      <AppLogo size="sm" />
      <AppButton variant="ghost" size="sm" @click="router.back()">
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M19 12H5M12 5l-7 7 7 7" />
        </svg>
        Back to selection
      </AppButton>
    </header>

    <!-- Form -->
    <div class="flex-1 flex items-center justify-center px-6 py-10">
      <div
        class="w-full max-w-md bg-card border border-border rounded-2xl p-8"
        style="box-shadow: var(--shadow-elevated)"
      >
        <h1 class="text-2xl font-bold font-display text-foreground mb-1">Create Profile</h1>
        <p class="text-sm text-muted-foreground mb-7 font-sans">Set up your Taipu persona to track your progress.</p>

        <form @submit.prevent="createAccount" class="space-y-6">
          <!-- Avatar Picker -->
          <div>
            <span class="block text-sm font-medium text-foreground mb-3 font-display">Choose Avatar</span>
            <div class="grid grid-cols-6 gap-2">
              <button
                v-for="(avatar, i) in AVATARS"
                :key="avatar.slug"
                type="button"
                @click="selectedAvatar = i"
                :class="[
                  'aspect-square rounded-xl text-2xl flex items-center justify-center transition-all border',
                  selectedAvatar === i
                    ? 'border-primary ring-2 ring-primary bg-surface-elevated'
                    : 'border-border bg-surface hover:bg-surface-elevated'
                ]"
              >
                {{ avatar.emoji }}
              </button>
            </div>
          </div>

          <!-- Name -->
          <div>
            <label for="name" class="block text-sm font-medium text-foreground mb-2 font-display">Name</label>
            <input
              id="name"
              type="text"
              v-model="name"
              placeholder="Enter your name..."
              class="w-full px-4 py-2.5 rounded-xl bg-surface border border-input text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring transition-all font-sans"
            />
          </div>

          <!-- Language -->
          <div>
            <span class="block text-sm font-medium text-foreground mb-2 font-display">Language</span>
            <div class="flex gap-2">
              <button
                v-for="option in languageOptions"
                :key="option.value"
                type="button"
                @click="language = option.value"
                :class="[
                  'flex-1 py-2 rounded-xl text-sm font-medium border transition-all font-sans',
                  language === option.value
                    ? 'bg-primary text-primary-foreground border-primary'
                    : 'bg-surface border-border text-muted-foreground hover:border-primary hover:text-foreground'
                ]"
              >
                {{ option.label }}
              </button>
            </div>
          </div>

          <!-- Difficulty -->
          <div>
            <span class="block text-sm font-medium text-foreground mb-2 font-display">Difficulty</span>
            <div class="flex gap-2">
              <button
                v-for="option in difficultyOptions"
                :key="option.value"
                type="button"
                @click="lessonDifficulty = option.value"
                :class="[
                  'flex-1 py-2 rounded-xl text-sm font-medium border transition-all font-sans',
                  lessonDifficulty === option.value
                    ? 'bg-primary text-primary-foreground border-primary'
                    : 'bg-surface border-border text-muted-foreground hover:border-primary hover:text-foreground'
                ]"
              >
                {{ option.label }}
              </button>
            </div>
          </div>

          <!-- Submit -->
          <AppButton type="submit" size="lg" :loading="isLoading" :full="true">
            Start Typing
          </AppButton>
        </form>
      </div>
    </div>
  </div>
</template>