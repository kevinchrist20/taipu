<script setup lang="ts">
import { Check, Lock, Play, Redo } from 'lucide-vue-next';
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
</script>

<template>
  <div
    class="flex items-center gap-4 bg-card border border-border rounded-2xl px-5 py-4 transition-all shadow-md"
    :class="isLocked ? 'opacity-50' : 'hover:border-primary cursor-pointer'"
    @click="handleLessonClick"
  >
    <!-- Number + Title -->
    <div class="flex-1 min-w-0">
      <p
        class="font-mono font-semibold text-base leading-snug"
        :class="isLocked ? 'text-muted-foreground' : 'text-foreground'"
      >
        {{ index + 1 }}. {{ lesson.title }}
        <Lock v-if="isLocked" :size="13" class="inline ml-1.5 mb-0.5 text-muted-foreground" />
        <Check v-else-if="isCompleted" :size="13" class="inline ml-1.5 mb-0.5 text-success" />
      </p>
    </div>

    <!-- Start button -->
    <button
      :disabled="isLocked"
      class="flex items-center gap-1.5 px-4 py-1.5 rounded-xl text-sm font-medium transition-all shrink-0"
      :class="isLocked
        ? 'bg-surface text-muted-foreground cursor-not-allowed border border-border'
        : isCompleted
          ? 'bg-success/10 border text-primary-foreground border-success/10 hover:bg-success/10'
          : 'bg-primary text-primary-foreground hover:opacity-90 active:scale-95'"
      @click.stop="handleLessonClick"
    >
      <component :is="isCompleted ? Redo : Play" :size="13" />
      {{ isCompleted ? 'Redo' : 'Start' }}
    </button>
  </div>
</template>
