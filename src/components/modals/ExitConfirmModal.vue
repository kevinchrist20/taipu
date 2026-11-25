<script setup lang="ts">
import { OctagonX } from '@lucide/vue'
import AppButton from '../AppButton.vue'
import Modal from './Modal.vue'

interface Props {
  show: boolean
  hasProgress: boolean
}

defineProps<Props>()

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()
</script>

<template>
  <Modal :show="show" size="md" :close-on-escape="true" @close="emit('cancel')">
    <div class="text-center">
      <div class="mb-6">
        <div
          class="w-16 h-16 border border-destructive/25 rounded-2xl flex items-center justify-center mx-auto mb-4"
        >
          <OctagonX class="h-8 w-8 text-destructive" />
        </div>
        <h3 class="text-2xl font-bold text-foreground mb-2 font-display">
          Exit Lesson?
        </h3>
        <p v-if="hasProgress" class="text-muted-foreground text-sm">
          Your progress will be lost if you exit now.
        </p>
        <p v-else class="text-muted-foreground text-sm">
          Are you sure you want to exit?
        </p>
      </div>

      <div class="flex gap-3">
        <AppButton size="md" variant="ghost" class="flex-1 px-5 py-2.5" @click="emit('cancel')">
          Cancel
        </AppButton>
        <AppButton variant="danger" class="flex-1 px-5 py-2.5 " @click="emit('confirm')">
          Exit
        </AppButton>
      </div>
    </div>
  </Modal>
</template>
