<script setup lang="ts">
import AppButton from './AppButton.vue';

defineProps<{
  message: string;
  retryLabel?: string;
}>();

const emit = defineEmits<{
  retry: [];
}>();
</script>

<template>
  <div class="rounded-2xl border border-destructive/30 bg-destructive/10 p-6 text-destructive">
    <p>{{ message }}</p>
    <div v-if="retryLabel" class="mt-4">
      <AppButton variant="secondary" size="sm" @click="emit('retry')">{{ retryLabel }}</AppButton>
    </div>
  </div>
</template>
