<script setup lang="ts">
import { Download, RotateCw, X } from '@lucide/vue'
import { computed } from 'vue'
import { useUpdater } from '../composables/useUpdater'
import AppButton from './AppButton.vue'

const {
  status,
  errorMessage,
  currentVersion,
  newVersion,
  releaseNotes,
  progressPercent,
  isVisible,
  startDownload,
  installNow,
  dismiss,
} = useUpdater()

const title = computed(() => {
  switch (status.value) {
    case 'available':
      return 'Update available'
    case 'downloading':
      return 'Downloading update'
    case 'downloaded':
      return 'Update ready to install'
    case 'installing':
      return 'Updating…'
    case 'error':
      return 'Update failed'
    default:
      return 'Taipu'
  }
})

const ariaLabel = computed(() => `Taipu update: ${title.value}`)
</script>

<template>
  <Transition name="update-dialog">
    <div
      v-if="isVisible"
      class="fixed bottom-4 right-4 z-40 w-88 max-w-[calc(100vw-2rem)]"
      role="dialog"
      :aria-label="ariaLabel"
    >
      <div class="p-4 rounded-2xl border border-border bg-surface-elevated shadow-xl">
        <div class="mb-2 flex items-start justify-between gap-3">
          <p class="text-sm font-semibold text-foreground">
            {{ title }}
          </p>
          <button
            v-if="status !== 'installing'"
            type="button"
            class="-mt-1 -mr-1 p-1.5 rounded-lg text-muted-foreground hover:bg-surface hover:text-foreground transition-colors"
            aria-label="Dismiss update"
            @click="dismiss()"
          >
            <X class="h-4 w-4" />
          </button>
        </div>

        <p v-if="newVersion" class="mb-3 text-xs text-muted-foreground">
          {{ currentVersion }} → {{ newVersion }}
        </p>

        <div
          v-if="status === 'available' && releaseNotes"
          class="mb-3 max-h-24 overflow-auto p-3 rounded-lg border border-border bg-surface text-xs text-muted-foreground whitespace-pre-line"
        >
          {{ releaseNotes }}
        </div>

        <div v-if="status === 'downloading'">
          <div class="mb-2 h-1 w-full overflow-hidden rounded-full bg-surface">
            <div
              class="h-full rounded-full bg-primary transition-all duration-200"
              :style="{ width: `${progressPercent}%` }"
            />
          </div>
          <p class="text-xs text-muted-foreground">
            {{ progressPercent }}%
          </p>
        </div>

        <div v-if="status === 'installing'" class="flex items-center gap-2 text-sm text-muted-foreground">
          <span class="w-4 h-4 rounded-full border-2 border-current border-t-transparent animate-spin shrink-0" />
          Installing the update and restarting…
        </div>

        <div v-if="status === 'error'" class="mb-3 text-sm text-destructive">
          {{ errorMessage }}
        </div>

        <div class="flex items-center justify-end gap-3">
          <AppButton
            v-if="status === 'available' || status === 'error' || status === 'downloaded'"
            variant="ghost"
            size="sm"
            @click="dismiss()"
          >
            Later
          </AppButton>

          <AppButton
            v-if="status === 'available' || status === 'error'"
            variant="primary"
            size="sm"
            @click="startDownload()"
          >
            <Download class="h-4 w-4" />
            {{ status === 'error' ? 'Try again' : 'Download update' }}
          </AppButton>

          <AppButton
            v-if="status === 'downloaded'"
            variant="primary"
            size="sm"
            @click="installNow()"
          >
            <RotateCw class="h-4 w-4" />
            Restart & update
          </AppButton>
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.update-dialog-enter-active,
.update-dialog-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.update-dialog-enter-from,
.update-dialog-leave-to {
  opacity: 0;
  transform: translateY(12px);
}
</style>
