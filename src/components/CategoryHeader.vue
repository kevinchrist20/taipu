<script setup lang="ts">
import CircularProgressIndicator from './CircularProgressIndicator.vue';

defineProps<{
  categoryName: string;
  completed: number;
  total: number;
  percentage: number;
}>();
</script>

<template>
  <div class="bg-surface-elevated rounded-2xl p-6 mb-8 border border-border">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-foreground mb-1">{{ categoryName }}</h1>
        <p class="text-sm text-muted-foreground">{{ completed }} of {{ total }} lessons complete</p>
      </div>
      <CircularProgressIndicator
        :percentage="percentage"
        :diameter="72"
        :stroke-width="6"
      />
    </div>
  </div>
</template>