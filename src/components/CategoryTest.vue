<script setup lang="ts">
import { Check, Trophy, Lock } from 'lucide-vue-next';
import { Lesson } from '../types/bindings';

interface Props {
  tests: Lesson[];
  allLessonsCompleted: boolean;
  isTestCompleted: (testId: number) => boolean;
}

type Emits = {
  'start-test': [test: Lesson];
};

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const handleTestClick = (test: Lesson) => {
  if (props.allLessonsCompleted) {
    emit('start-test', test);
  }
};
</script>

<template>
  <div v-if="tests.length" class="relative flex items-center gap-4 mt-12">
    <!-- Trophy Icon/Node -->
    <div class="relative z-10">
      <button
        v-for="test in tests"
        :key="test.id"
        @click="handleTestClick(test)"
        :disabled="!allLessonsCompleted"
        class="w-28 h-28 rounded-full flex items-center justify-center transition-all duration-300 shadow-2xl"
        :class="{
          'bg-gradient-to-br from-yellow-500 via-yellow-600 to-orange-600 hover:scale-110 cursor-pointer': allLessonsCompleted && !isTestCompleted(test.id),
          'bg-gradient-to-br from-green-500 to-green-600 hover:scale-110 cursor-pointer': isTestCompleted(test.id),
          'bg-gray-700 cursor-not-allowed opacity-50': !allLessonsCompleted
        }"
      >
        <Trophy v-if="isTestCompleted(test.id)" :size="48" class="text-white" />
        <Trophy v-else-if="allLessonsCompleted" :size="48" class="text-white animate-bounce" />
        <Lock v-else :size="36" class="text-gray-400" />
      </button>
    </div>

    <!-- Test Card -->
    <div 
      class="flex-1 bg-gradient-to-br from-yellow-900/30 to-orange-900/30 rounded-2xl p-6 border-2 transition-all duration-300"
      :class="{
        'border-yellow-500 hover:border-yellow-400 cursor-pointer hover:shadow-lg hover:shadow-yellow-500/20': allLessonsCompleted,
        'border-gray-700 opacity-60': !allLessonsCompleted
      }"
      @click="allLessonsCompleted && tests.length ? handleTestClick(tests[0]) : null"
    >
      <div class="flex items-center gap-3 mb-3">
        <div class="w-12 h-12 bg-yellow-500 rounded-xl flex items-center justify-center shadow-lg">
          <Trophy :size="24" class="text-yellow-900" />
        </div>
        <div>
          <h3 class="text-xl font-bold text-white">Unit Test</h3>
          <p class="text-sm text-gray-400">Final challenge</p>
        </div>
      </div>
      
      <p class="text-sm text-gray-300 mb-4">
        {{ allLessonsCompleted 
          ? 'Complete this test to unlock the next category!' 
          : 'Complete all lessons above to unlock this test.' 
        }}
      </p>

      <div v-if="tests.length && isTestCompleted(tests[0].id)" 
        class="flex items-center gap-2 text-green-400 font-medium bg-green-900/30 px-4 py-2 rounded-lg">
        <Check :size="20" />
        Test Passed! Next category unlocked.
      </div>
      <div v-else-if="allLessonsCompleted" 
        class="flex items-center justify-between bg-yellow-500 text-yellow-900 font-bold px-4 py-3 rounded-lg hover:bg-yellow-400 transition-colors">
        <span>Take the Test</span>
        <Trophy :size="20" />
      </div>
      <div v-else 
        class="flex items-center gap-2 text-gray-500 font-medium bg-gray-800 px-4 py-2 rounded-lg">
        <Lock :size="16" />
        Locked
      </div>
    </div>
  </div>
</template>