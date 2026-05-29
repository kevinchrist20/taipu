<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(defineProps<{
  size?: 'sm' | 'md' | 'lg';
  message?: string;
  minHeightClass?: string;
}>(), {
  size: 'md',
  message: '',
  minHeightClass: 'h-48',
});

const spinnerClass = computed(() => {
  if (props.size === 'sm') return 'w-6 h-6';
  if (props.size === 'lg') return 'w-12 h-12';
  return 'w-8 h-8';
});
</script>

<template>
  <div class="flex flex-col justify-center items-center gap-3" :class="minHeightClass">
    <div class="rounded-full border-2 border-primary border-t-transparent animate-spin" :class="spinnerClass" />
    <p v-if="message" class="text-sm text-muted-foreground">{{ message }}</p>
  </div>
</template>
