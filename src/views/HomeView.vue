<script setup lang="ts">
import type { User } from '../types/bindings'
import { onMounted, ref } from 'vue'
import AppButton from '../components/AppButton.vue'
import AppLogo from '../components/AppLogo.vue'
import EmptyState from '../components/EmptyState.vue'
import ErrorState from '../components/ErrorState.vue'
import LoadingState from '../components/LoadingState.vue'
import { routes } from '../constants'
import router from '../router'
import UserService from '../services/user.service'
import { SessionStore } from '../storage'
import { avatarEmoji } from '../types'
import { extractErrorMessage } from '../utils/error-utils'

const users = ref<User[]>([])
const loading = ref(false)
const errorMessage = ref('')

async function loadUsers() {
  loading.value = true
  errorMessage.value = ''

  try {
    users.value = await UserService.getUsers()
  }
  catch (error) {
    errorMessage.value = extractErrorMessage(error, 'Unable to load profiles right now.')
    console.error('Error loading users:', error)
  }
  finally {
    loading.value = false
  }
}

function getAvatar(user: User): string {
  return avatarEmoji(user.avatar)
}

function selectUser(user: User) {
  SessionStore.setUser(user)
  router.push({ path: routes.lessons })
}

onMounted(loadUsers)
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

    <LoadingState v-if="loading" message="Loading profiles..." min-height-class="h-56" />

    <ErrorState v-else-if="errorMessage" :message="errorMessage" retry-label="Try again" class="w-full max-w-xl" @retry="loadUsers" />

    <EmptyState
      v-else-if="!users.length"
      title="No profiles yet"
      description="Create your first profile to start practicing."
      class="w-full max-w-xl"
    >
      <AppButton @click="router.push({ path: routes.createAccount })">
        Create profile
      </AppButton>
    </EmptyState>

    <!-- Profile Cards -->
    <div v-else class="flex flex-wrap gap-4 justify-center max-w-xl">
      <button
        v-for="user in users"
        :key="user.id"
        class="flex flex-col items-center gap-2 w-36 py-6 px-4 rounded-2xl bg-card border border-border hover:border-primary hover:bg-surface-elevated transition-all"
        @click="selectUser(user)"
      >
        <span class="text-4xl leading-none">{{ getAvatar(user) }}</span>
        <span class="text-sm font-medium text-foreground capitalize mt-1 font-sans">{{ user.name }}</span>
      </button>

      <!-- New Profile -->
      <button
        class="flex flex-col items-center gap-2 w-36 py-6 px-4 rounded-2xl bg-card border-2 border-dashed border-border hover:border-primary hover:bg-surface-elevated transition-all"
        @click="router.push({ path: routes.createAccount })"
      >
        <div class="w-10 h-10 rounded-full border-dashed border-2 border-border flex items-center justify-center text-muted-foreground text-2xl leading-none">
          +
        </div>
        <span class="text-sm font-medium text-muted-foreground mt-1 font-sans">New Profile</span>
      </button>
    </div>
  </div>
</template>
