<script setup lang="ts">
import CircularProgressIndicator from './CircularProgressIndicator.vue';

interface Props {
  categoryName: string;
  completed: number;
  total: number;
  percentage: number;
}

defineProps<Props>();
</script>

<template>
  <div class="bg-gradient-to-r from-indigo-600 to-purple-600 rounded-2xl p-6 mb-8 shadow-xl">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-3xl font-bold mb-2 text-white">{{ categoryName }}</h1>
        <p class="text-indigo-100 text-sm">{{ completed }} of {{ total }} lessons</p>
      </div>
      <div class="flex items-center gap-4">
        <CircularProgressIndicator 
          :percentage="percentage"
          :diameter="80"
          :stroke-width="8"
        />
      </div>
    </div>
  </div>
</template>