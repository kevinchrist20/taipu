<script setup lang="ts">
defineProps<{
  title: string;
  description: string;
}>();
</script>

<template>
  <div class="text-center py-12">
    <h3 class="text-xl font-bold text-muted-foreground mb-2">{{ title }}</h3>
    <p class="text-muted-foreground text-sm">{{ description }}</p>
    <div class="mt-4">
      <slot />
    </div>
  </div>
</template>
