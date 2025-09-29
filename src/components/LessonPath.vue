<script setup lang="ts">
import LessonCard from './LessonCard.vue';
import CategoryTest from './CategoryTest.vue';
import { Lesson } from '../types/bindings';

interface Props {
  lessons: Lesson[];
  tests: Lesson[];
  completedLessons: number[];
  allLessonsCompleted: boolean;
}

type Emits = {
  'start-lesson': [lesson: Lesson];
  'start-test': [test: Lesson];
};

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const isLessonComplete = (lessonId: number): boolean => {
  return props.completedLessons.includes(lessonId);
};

const isLessonLocked = (index: number): boolean => {
  if (index === 0) return false;
  const previousLesson = props.lessons[index - 1];
  return !isLessonComplete(previousLesson.id);
};

const isTestCompleted = (testId: number): boolean => {
  return props.completedLessons.includes(testId);
};

const handleStartLesson = (lesson: Lesson) => {
  emit('start-lesson', lesson);
};

const handleStartTest = (test: Lesson) => {
  emit('start-test', test);
};
</script>

<template>
  <div class="relative">
    <!-- Vertical Path Line -->
    <div class="absolute left-12 top-0 bottom-0 w-1 bg-gray-700/50"></div>

    <div class="space-y-8">
      <!-- Lesson Cards -->
      <LessonCard
        v-for="(lesson, index) in lessons"
        :key="lesson.id"
        :lesson="lesson"
        :index="index"
        :is-completed="isLessonComplete(lesson.id)"
        :is-locked="isLessonLocked(index)"
        @start-lesson="handleStartLesson"
      />

      <!-- Category Test -->
      <CategoryTest
        :tests="tests"
        :all-lessons-completed="allLessonsCompleted"
        :is-test-completed="isTestCompleted"
        @start-test="handleStartTest"
      />
    </div>
  </div>
</template>