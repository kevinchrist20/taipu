<script setup lang="ts">
import { Check, BookOpen, Lock, Star } from 'lucide-vue-next';
import { Lesson } from '../types/bindings';

interface Props {
  lesson: Lesson;
  index: number;
  isCompleted: boolean;
  isLocked: boolean;
}

type Emits = {
  'start-lesson': [lesson: Lesson];
};

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const handleLessonClick = () => {
  if (!props.isLocked) {
    emit('start-lesson', props.lesson);
  }
};

const getDifficultyClass = (difficulty: string) => {
  switch (difficulty) {
    case 'easy':
      return 'bg-green-900/50 text-green-400';
    case 'medium':
      return 'bg-yellow-900/50 text-yellow-400';
    case 'hard':
      return 'bg-red-900/50 text-red-400';
    default:
      return 'bg-gray-900/50 text-gray-400';
  }
};
</script>

<template>
  <div class="relative flex items-center gap-4">
    <!-- Lesson Icon/Node -->
    <div class="relative z-10">
      <button
        @click="handleLessonClick"
        :disabled="isLocked"
        class="w-24 h-24 rounded-full flex items-center justify-center transition-all duration-300 shadow-lg group"
        :class="{
          'bg-gradient-to-br from-green-500 to-green-600 hover:scale-110 cursor-pointer': isCompleted,
          'bg-gradient-to-br from-indigo-500 to-purple-600 hover:scale-110 cursor-pointer animate-pulse': !isCompleted && !isLocked,
          'bg-gray-700 cursor-not-allowed opacity-50': isLocked
        }"
      >
        <Check v-if="isCompleted" :size="40" class="text-white" />
        <Lock v-else-if="isLocked" :size="32" class="text-gray-400" />
        <BookOpen v-else :size="36" class="text-white group-hover:scale-110 transition-transform" />
      </button>
      
      <!-- Stars for completed -->
      <div v-if="isCompleted" class="absolute -top-2 -right-2">
        <div class="bg-yellow-400 rounded-full p-1.5 shadow-lg">
          <Star :size="16" class="text-yellow-900 fill-yellow-900" />
        </div>
      </div>
    </div>

    <!-- Lesson Card -->
    <div 
      @click="handleLessonClick"
      class="flex-1 bg-gray-800 rounded-2xl p-5 border-2 transition-all duration-300"
      :class="{
        'border-green-500 hover:border-green-400 cursor-pointer hover:shadow-lg hover:shadow-green-500/20': isCompleted,
        'border-indigo-500 hover:border-indigo-400 cursor-pointer hover:shadow-lg hover:shadow-indigo-500/20': !isCompleted && !isLocked,
        'border-gray-700 opacity-60': isLocked
      }"
    >
      <div class="flex items-start justify-between mb-2">
        <h3 class="text-lg font-bold text-white">{{ lesson.title }}</h3>
        <span class="px-2 py-1 bg-gray-700 rounded-full text-xs font-medium text-gray-300">
          Lesson {{ index + 1 }}
        </span>
      </div>
      <p class="text-sm text-gray-400 line-clamp-2 mb-3">{{ lesson.content }}</p>
      <div class="flex items-center justify-between">
        <span class="text-xs font-medium px-2 py-1 rounded-full"
          :class="getDifficultyClass(lesson.difficulty)">
          {{ lesson.difficulty }}
        </span>
        <span v-if="isCompleted" class="text-green-400 text-sm font-medium flex items-center gap-1">
          <Check :size="16" />
          Completed
        </span>
        <span v-else-if="isLocked" class="text-gray-500 text-sm font-medium flex items-center gap-1">
          <Lock :size="16" />
          Locked
        </span>
        <span v-else class="text-indigo-400 text-sm font-medium">
          Start →
        </span>
      </div>
    </div>
  </div>
</template>