<script setup lang="ts">
import { Keyboard } from 'lucide-vue-next';

defineProps<{
  size?: 'sm' | 'md' | 'lg'
}>()
</script>

<template>
  <div
    class="flex items-center"
    :class="size === 'sm' ? 'gap-2' : size === 'lg' ? 'gap-4' : 'gap-3'"
  >
    <!-- Icon container — always uses brand teal so the logo is consistent across themes -->
    <div
      class="bg-primary flex items-center justify-center shrink-0"
      :class="size === 'sm' ? 'w-8 h-8 rounded-xl' : size === 'lg' ? 'w-16 h-16 rounded-3xl' : 'w-11 h-11 rounded-2xl'"
    >
      <Keyboard
        class="text-primary-foreground"
        :class="size === 'sm' ? 'w-4 h-4' : size === 'lg' ? 'w-9 h-9' : 'w-6 h-6'"
        :stroke-width="2"
      />
    </div>

    <!-- Wordmark -->
    <span
      class="font-bold font-display tracking-tight leading-none text-foreground"
      :class="size === 'sm' ? 'text-lg' : size === 'lg' ? 'text-4xl' : 'text-2xl'"
    >taipu<span class="text-primary">.</span></span>
  </div>
</template>
