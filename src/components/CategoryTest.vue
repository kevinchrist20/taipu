<script setup lang="ts">
import { Check, Lock, Play } from 'lucide-vue-next';
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
  <div v-if="tests.length" class="flex items-center gap-4 bg-card border border-border rounded-2xl px-5 py-5 transition-all shadow-md"
    :class="allLessonsCompleted ? 'hover:border-warning cursor-pointer' : 'opacity-50'"
    @click="allLessonsCompleted ? handleTestClick(tests[0]) : null"
  >
    <!-- Title -->
    <div class="flex-1 min-w-0">
      <p class="font-mono font-semibold text-base text-foreground leading-snug flex items-center gap-2">
        {{ tests[0]?.title ?? 'Unit Test' }}
        <span class="text-xs font-bold uppercase tracking-wider px-2 py-0.5 rounded-sm bg-red-100 text-destructive">TEST</span>
        <Lock v-if="!allLessonsCompleted" :size="13" class="text-muted-foreground" />
      </p>
    </div>

    <!-- Action -->
    <button
      :disabled="!allLessonsCompleted"
      class="flex items-center gap-1.5 px-4 py-1.5 rounded-xl text-sm font-medium transition-all shrink-0"
      :class="!allLessonsCompleted
        ? 'bg-surface text-muted-foreground cursor-not-allowed border border-border'
        : tests.length && isTestCompleted(tests[0].id)
          ? 'bg-success/10 border text-primary-foreground border-success/10 hover:bg-success/10'
          : 'bg-warning text-warning-foreground hover:opacity-90 active:scale-95'"
      @click.stop="allLessonsCompleted ? handleTestClick(tests[0]) : null"
    >
      <component :is="tests.length && isTestCompleted(tests[0].id) ? Check : Play" :size="13" />
      {{ tests.length && isTestCompleted(tests[0].id) ? 'Passed' : 'Start' }}
    </button>
  </div>
</template>
