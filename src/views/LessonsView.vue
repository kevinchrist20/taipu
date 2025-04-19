<script setup lang="ts">
import { onMounted, ref, onUnmounted } from 'vue';
import BackButton from '../components/BackButton.vue';
import { routes } from '../constants';
import router from '../router';
import LessonService, { CategoryWithLessons } from '../services/lesson.service';
import { SessionStore, useLessonStore } from '../storage';
import { Lesson, User } from '../types/bindings';

const categories = ref<CategoryWithLessons[]>([]);
const completedLessons = ref<number[]>([]);
const user = ref<User | null>(null);
const lessonStore = useLessonStore();
const loading = ref(false);
const loadingMore = ref(false);
const visibleCategoryCount = ref(1); // Start with one visible category

// Intersection Observer for infinite scrolling
const observer = ref<IntersectionObserver | null>(null);
const bottomMarker = ref<HTMLElement | null>(null);

async function fetchData() {
  try {
    loading.value = true;
    user.value = SessionStore.user;
    if (!user.value) return;

    categories.value = await LessonService.getLessonsByCategories(
      user.value.lessonDifficulty || '',
      user.value.id
    );
    completedLessons.value = await LessonService.getCompletedLessons(user.value.id);
  } catch (error) {
    console.error('Error fetching data:', error);
  } finally {
    loading.value = false;
  }
}

function startLesson(lesson: Lesson) {
  lessonStore.setLesson(lesson);
  router.push({ path: routes.lessonArea });
}

function startTest(test: Lesson) {
  lessonStore.setLesson(test);
  router.push({ path: routes.lessonArea });
}

function isLessonComplete(lessonId: number): boolean {
  return completedLessons.value.includes(lessonId);
}

function isCategoryFullyCompleted(category: CategoryWithLessons): boolean {
  // All lessons must be completed
  const allLessonsCompleted = category.lessons.every(lesson => 
    completedLessons.value.includes(lesson.id)
  );
  
  // At least one test must be completed (if there are any tests)
  const anyTestCompleted = category.tests.length === 0 || 
    category.tests.some(test => completedLessons.value.includes(test.id));
  
  return allLessonsCompleted && anyTestCompleted;
}

// Function to load more categories when scrolling
function loadMoreCategories() {
  if (loadingMore.value) return;
  if (visibleCategoryCount.value >= categories.value.length) return;
  
  loadingMore.value = true;
  setTimeout(() => {
    visibleCategoryCount.value += 1;
    loadingMore.value = false;
  }, 300); // Small delay to prevent too rapid loading
}

// Setup intersection observer for infinite scrolling
function setupIntersectionObserver() {
  observer.value = new IntersectionObserver((entries) => {
    const entry = entries[0];
    if (entry.isIntersecting) {
      loadMoreCategories();
    }
  }, { threshold: 0.1 });
  
  if (bottomMarker.value) {
    observer.value.observe(bottomMarker.value);
  }
}

onMounted(async () => {
  await fetchData();
  setupIntersectionObserver();
});

onUnmounted(() => {
  if (observer.value && bottomMarker.value) {
    observer.value.unobserve(bottomMarker.value);
  }
});
</script>

<template>
  <div class="flex flex-col items-center min-h-screen bg-gray-800 text-white font-mono px-6 pt-12">
    <!-- Back Button -->
    <BackButton />

    <!-- Header -->
    <h1 class="text-3xl font-bold mb-8 capitalize">{{ user?.lessonDifficulty }} Lessons</h1>

    <!-- Loading State -->
    <div v-if="loading" class="flex justify-center items-center h-64">
      <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-indigo-500"></div>
    </div>

    <!-- Category-based Lessons -->
    <div v-else class="w-full max-w-4xl">
      <div v-if="categories.length" class="space-y-12">
        <!-- Display only the visible categories -->
        <div v-for="(category, idx) in categories.slice(0, visibleCategoryCount)" :key="category.category" class="border border-gray-700 rounded-lg p-6">
          <!-- Category Header -->
          <div class="flex justify-between items-center mb-6 pb-3 border-b border-gray-700">
            <h2 class="text-2xl font-bold">{{ category.category }}</h2>
            <div class="flex items-center">
              <span v-if="!category.is_available" class="mr-2 text-sm text-red-400">
                Complete previous category first
              </span>
              <v-icon v-if="!category.is_available" name="fc-lock" />
              <v-icon v-else-if="isCategoryFullyCompleted(category)" name="fc-ok" class="text-green-500" />
            </div>
          </div>

          <!-- Category Lessons -->
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-6 mb-6">
            <div v-for="lesson in category.lessons" :key="lesson.id"
              :class="`bg-gray-900 rounded-lg shadow-md p-6 transition-transform ${category.is_available ? 'hover:bg-gray-700 hover:scale-105' : 'opacity-60 cursor-not-allowed'}`">
              <div class="flex justify-between items-start mb-2">
                <h2 class="text-xl font-semibold">{{ lesson.title }}</h2>
                <v-icon v-if="!category.is_available" name="fc-lock" />
                <v-icon v-else-if="isLessonComplete(lesson.id)" name="fc-ok" />
              </div>
              <p class="text-sm text-gray-400 mb-4">{{ lesson.content }}</p>
              <div class="flex justify-between items-center">
                <span class="text-sm text-gray-400">Difficulty: {{ lesson.difficulty }}</span>
                <button
                  class="px-4 py-2 bg-indigo-600 hover:bg-indigo-500 text-white font-semibold rounded-md transition-transform hover:scale-105"
                  @click="startLesson(lesson)" :disabled="!category.is_available"
                  :class="{ 'opacity-50 cursor-not-allowed': !category.is_available }">
                  {{ isLessonComplete(lesson.id) ? 'Retake' : 'Start Lesson' }}
                </button>
              </div>
            </div>
          </div>

          <!-- Category Tests -->
          <div v-if="category.tests.length" class="mt-6 border-t border-gray-700 pt-4">
            <h3 class="text-lg font-semibold mb-3">Category Tests</h3>
            <p class="text-gray-400 text-sm mb-4">Complete at least one test to unlock the next category.</p>
            
            <div class="grid grid-cols-1 gap-4">
              <div v-for="test in category.tests" :key="test.id"
                class="bg-gray-800 border border-gray-700 rounded-lg p-4 flex justify-between items-center">
                <div>
                  <h4 class="font-medium">{{ test.title }}</h4>
                  <p class="text-sm text-gray-400">{{ test.content }}</p>
                </div>
                <button
                  class="px-4 py-2 bg-green-600 hover:bg-green-500 text-white font-semibold rounded-md"
                  @click="startTest(test)" 
                  :disabled="!category.is_available || !category.lessons.every(l => isLessonComplete(l.id))"
                  :class="{ 'opacity-50 cursor-not-allowed': !category.is_available || !category.lessons.every(l => isLessonComplete(l.id)) }">
                  {{ isLessonComplete(test.id) ? 'Retake Test' : 'Take Test' }}
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- Loading more indicator -->
        <div v-if="loadingMore" class="text-center py-4">
          <div class="animate-spin inline-block rounded-full h-8 w-8 border-t-2 border-b-2 border-indigo-500"></div>
          <p class="mt-2 text-gray-400">Loading more...</p>
        </div>

        <!-- Bottom marker for intersection observer -->
        <div ref="bottomMarker" class="h-4"></div>
      </div>

      <!-- No lessons message -->
      <p v-else class="text-red-500 text-center">No lessons available for this difficulty.</p>
    </div>
  </div>
</template>