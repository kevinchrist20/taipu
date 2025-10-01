<script setup lang="ts">
import CircularProgressIndicator from './CircularProgressIndicator.vue';
import { Check, LockIcon } from 'lucide-vue-next';
import { TestStatus } from '../types';

defineProps<{
  categoryName: string;
  completed: number;
  total: number;
  percentage: number;
  testStatus: TestStatus;
  isAvailable: boolean;
  isCompleted: boolean;
}>();

const emit = defineEmits<{
  click: []
}>();
</script>

<template>
  <div
    class="bg-gray-900 rounded-xl p-6 border border-gray-700 transition-all duration-300 cursor-pointer group"
    :class="{
      'hover:border-indigo-500 hover:shadow-lg hover:shadow-indigo-500/20': isAvailable,
      'opacity-60 cursor-not-allowed': !isAvailable
    }"
    @click="emit('click')"
  >
    <!-- Category Header with Progress Indicator -->
    <div class="flex justify-between items-start mb-4">
      <div class="flex-1">
        <h3 class="text-xl font-bold text-white group-hover:text-indigo-400 transition-colors mb-2">
          {{ categoryName }}
        </h3>
        <div class="text-sm text-gray-400">
          {{ percentage }}% • {{ completed }}/{{ total }} lessons
        </div>
      </div>
      
      <!-- Progress Circle on the Right -->
      <div class="ml-4 flex-shrink-0">
        <div v-if="!isAvailable" class="w-12 h-12 flex items-center justify-center">
          <LockIcon color="#fcba03" />
        </div>
        <div v-else-if="isCompleted"
          class="w-12 h-12 bg-green-600 rounded-full flex items-center justify-center">
          <Check class="text-white" />
        </div>
        <CircularProgressIndicator 
          v-else 
          :percentage="percentage"
          :diameter="64"
          :stroke-width="4"
        />
      </div>
    </div>

    <!-- Test Status -->
    <div class="flex items-center justify-between mt-5">
      <span class="text-sm font-medium text-gray-300">Test:</span>
      <span 
        class="px-3 py-1 rounded-full text-xs font-medium"
        :class="{
          'bg-green-900 text-green-400': testStatus === 'Passed',
          'bg-indigo-900 text-indigo-400': testStatus === 'Ready',
          'bg-gray-700 text-gray-400': testStatus === 'Locked'
        }"
      >
        {{ testStatus }}
      </span>
    </div>

    <!-- Unlock Message -->
    <div v-if="!isAvailable" class="mt-3 text-xs text-gray-500 italic">
      Next unlock: Pass previous category test
    </div>
  </div>
</template>