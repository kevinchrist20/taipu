<script setup lang="ts">
import type { Lesson } from '../types/bindings'
import CategoryTest from './CategoryTest.vue'
import LessonCard from './LessonCard.vue'

interface Props {
  lessons: Lesson[]
  tests: Lesson[]
  completedLessons: number[]
  allLessonsCompleted: boolean
}

interface Emits {
  'start-lesson': [lesson: Lesson]
  'start-test': [test: Lesson]
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

function isLessonComplete(lessonId: number): boolean {
  return props.completedLessons.includes(lessonId)
}

function isLessonLocked(index: number): boolean {
  if (index === 0)
    return false
  const previousLesson = props.lessons[index - 1]
  return !isLessonComplete(previousLesson.id)
}

function isTestCompleted(testId: number): boolean {
  return props.completedLessons.includes(testId)
}

function handleStartLesson(lesson: Lesson) {
  emit('start-lesson', lesson)
}

function handleStartTest(test: Lesson) {
  emit('start-test', test)
}
</script>

<template>
  <div class="space-y-3">
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
</template>
