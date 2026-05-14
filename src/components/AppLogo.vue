<script setup lang="ts">
import { Keyboard } from 'lucide-vue-next';

defineProps<{
  size?: 'sm' | 'md' | 'lg'
}>()
</script>

<template>
  <div class="flex items-center gap-3" :class="size === 'sm' ? 'gap-2' : size === 'lg' ? 'gap-4' : 'gap-3'">
    <!-- Icon container -->
    <div
      class="rounded-2xl bg-surface-elevated flex items-center justify-center shrink-0"
      :class="size === 'sm' ? 'w-8 h-8 rounded-xl' : size === 'lg' ? 'w-16 h-16 rounded-3xl' : 'w-11 h-11'"
    >
      <Keyboard
        :class="[
          'text-primary',
          size === 'sm' ? 'w-4 h-4' : size === 'lg' ? 'w-9 h-9' : 'w-6 h-6'
        ]"
        :stroke-width="1.5"
      />
    </div>

    <!-- Wordmark -->
    <span
      class="font-bold font-display text-foreground tracking-tight leading-none"
      :class="size === 'sm' ? 'text-lg' : size === 'lg' ? 'text-4xl' : 'text-2xl'"
    >taipu<span class="text-primary">.</span></span>
  </div>
</template>
