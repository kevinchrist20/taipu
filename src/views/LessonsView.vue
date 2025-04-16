<script setup lang="ts">
import { onMounted, ref, computed } from 'vue';
import { Lesson, User, LessonTest } from '../types/bindings';
import LessonService from '../services/lesson.service';
import { SessionStore, useLessonStore } from '../storage';
import router from '../router';
import { routes } from '../constants';
import BackButton from '../components/BackButton.vue';

interface GroupedLessons {
  title: string;
  lessons: Lesson[];
}

const lessons = ref<Lesson[]>([]);
const completedLessons = ref<number[]>([]);
const user = ref<User | null>(null);
const lessonTests = ref<Map<number, LessonTest[]>>(new Map());
const expandedTests = ref<Set<number>>(new Set());
const expandedContent = ref<Set<number>>(new Set());

const lessonStore = useLessonStore();

async function getLessons() {
  user.value = SessionStore.user;
  if (!user.value) return;
  lessons.value = await LessonService.getLessons(user.value.lessonDifficulty || '');
  completedLessons.value = await LessonService.getCompletedLessons(user.value.id);
  for (const lesson of lessons.value) {
    const tests = await LessonService.getTestForLesson(lesson.id);
    if (tests.length) lessonTests.value.set(lesson.id, tests);
  }
}

function toggleTestDetails(lessonId: number) {
  expandedTests.value.has(lessonId)
    ? expandedTests.value.delete(lessonId)
    : expandedTests.value.add(lessonId);
}

function toggleContent(lessonId: number) {
  expandedContent.value.has(lessonId)
    ? expandedContent.value.delete(lessonId)
    : expandedContent.value.add(lessonId);
}

function startLesson(lesson: Lesson) {
  lessonStore.setLesson(lesson);
  router.push({ path: routes.lessonArea });
}

function isLessonAvailable(index: number): boolean {
  if (index === 0) return true;
  const prev = lessons.value[index - 1]?.id;
  return completedLessons.value.includes(prev);
}

// derive progress
const progress = computed(() => {
  return lessons.value.length
    ? Math.round((completedLessons.value.length / lessons.value.length) * 100)
    : 0;
});

// group by row prefix in title
const groups = computed<GroupedLessons[]>(() => {
  const map = new Map<string, Lesson[]>();
  for (const l of lessons.value) {
    // assume titles start with e.g. "Home Row:" or "Top Row:"
    const key = l.title.split(':')[0] + ':';
    if (!map.has(key)) map.set(key, []);
    map.get(key)!.push(l);
  }
  return Array.from(map.entries()).map(([title, ls]) => ({ title, lessons: ls }));
});

onMounted(getLessons);
</script>

<template>
  <div class="flex flex-col items-center min-h-screen bg-gray-800 text-white font-mono px-6 pt-6">
    <BackButton />

    <h1 class="text-3xl font-bold mb-4 capitalize">{{ user?.lessonDifficulty }} Lessons</h1>

    <!-- Progress Bar -->
    <div class="w-full max-w-4xl mb-6">
      <div class="bg-gray-700 rounded-full h-2 overflow-hidden">
        <div
          class="bg-indigo-500 h-2"
          :style="{ width: `${progress}%` }"
        ></div>
      </div>
      <p class="text-sm text-gray-400 mt-2">
        {{ completedLessons.length }}/{{ lessons.length }} lessons completed
      </p>
    </div>

    <!-- Lessons Sections -->
    <div class="w-full max-w-4xl space-y-12 pb-12">
      <div v-for="group in groups" :key="group.title">
        <h2 class="text-2xl font-semibold mb-4">{{ group.title }}</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
          <div
            v-for="(lesson, idx) in group.lessons"
            :key="lesson.id"
            :class="[
              'bg-gray-900 rounded-lg shadow-md p-8 transition-transform',
              isLessonAvailable(idx) ? 'hover:bg-gray-700 hover:scale-105' : 'opacity-60 cursor-not-allowed'
            ]"
          >
            <div class="flex justify-between items-center mb-4">
              <h3 class="text-xl font-semibold">{{ lesson.title }}</h3>
              <div class="flex items-center space-x-2">
                <span
                  v-if="lessonTests.has(lesson.id)"
                  class="px-2 py-1 text-xs bg-indigo-600 rounded-full"
                >Test Available</span>
                <i
                  v-if="!isLessonAvailable(idx)"
                  class="fas fa-lock text-gray-500"
                ></i>
                <i
                  v-else-if="completedLessons.includes(lesson.id)"
                  class="fas fa-check-circle text-green-400"
                ></i>
              </div>
            </div>

            <p
              :class="[
                'text-sm text-gray-400 mb-4',
                expandedContent.has(lesson.id) ? '' : 'line-clamp-3'
              ]"
            >
              {{ lesson.content }}
            </p>
            <button
              v-if="lesson.content.split(' ').length > 20"
              @click="toggleContent(lesson.id)"
              class="text-xs text-indigo-300 mb-4"
            >
              {{ expandedContent.has(lesson.id) ? 'Show less' : 'Show more' }}
            </button>

            <!-- Test Details -->
            <div v-if="lessonTests.has(lesson.id)" class="mb-4">
              <button
                @click="toggleTestDetails(lesson.id)"
                class="text-sm text-indigo-400 flex items-center hover:text-indigo-300 mb-2"
              >
                <i
                  :class="expandedTests.has(lesson.id)
                    ? 'fas fa-chevron-down'
                    : 'fas fa-chevron-right'"
                  class="mr-1"
                ></i>
                Associated Test
              </button>
              <div
                v-if="expandedTests.has(lesson.id)"
                class="p-4 bg-gray-800 rounded-md space-y-3"
              >
                <div
                  v-for="test in lessonTests.get(lesson.id)"
                  :key="test.id"
                  class="text-sm"
                >
                  <h4 class="font-semibold">{{ test.title }}</h4>
                  <p class="text-xs text-gray-400 truncate">{{ test.content }}</p>
                  <div class="flex justify-between text-xs text-gray-400">
                    <span>WPM: {{ test.passingWpm }}</span>
                    <span>Accuracy: {{ test.accuracyThreshold }}%</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Footer -->
            <div class="flex justify-between items-center">
              <span class="text-sm text-gray-400">
                Difficulty: {{ lesson.difficulty }}
              </span>
              <button
                @click="startLesson(lesson)"
                :disabled="!isLessonAvailable(idx)"
                class="px-6 py-2 bg-indigo-600 hover:bg-indigo-500 text-white font-semibold rounded-md transition-all"
                :class="{ 'opacity-50 cursor-not-allowed': !isLessonAvailable(idx) }"
              >
                {{ completedLessons.includes(lesson.id) ? 'Retake' : 'Start Lesson' }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <p v-if="!lessons.length" class="text-red-500 text-center">
      No lessons available for this difficulty.
    </p>
  </div>
</template>

<style>
/* add the Tailwind line-clamp plugin in your project to use .line-clamp-3 */
</style>