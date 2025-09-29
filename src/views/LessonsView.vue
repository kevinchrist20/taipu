<script setup lang="ts">
import { onMounted, ref } from 'vue';
import BackButton from '../components/BackButton.vue';
import CategoryCard from '../components/CategoryCard.vue';
import router from '../router';
import LessonService from '../services/lesson.service';
import { SessionStore } from '../storage';
import { CategoryWithLessons, User } from '../types/bindings';
import useAlert from '../utils/useAlert';

const categories = ref<CategoryWithLessons[]>([]);
const completedLessons = ref<number[]>([]);
const user = ref<User | null>(null);
const loading = ref(false);

// Compute category statistics
const getCategoryProgress = (category: CategoryWithLessons) => {
  const totalLessons = category.lessons.length;
  const completedCount = category.lessons.filter(lesson =>
    completedLessons.value.includes(lesson.id)
  ).length;
  const percentage = totalLessons > 0 ? Math.round((completedCount / totalLessons) * 100) : 0;

  return {
    completed: completedCount,
    total: totalLessons,
    percentage
  };
};

// Check if category test is completed
const isCategoryTestCompleted = (category: CategoryWithLessons) => {
  return category.tests.some(test => completedLessons.value.includes(test.id));
};

// Get test status for category
const getCategoryTestStatus = (category: CategoryWithLessons): 'Passed' | 'Ready' | 'Locked' => {
  if (!category.isAvailable) return 'Locked';

  const allLessonsCompleted = category.lessons.every(lesson =>
    completedLessons.value.includes(lesson.id)
  );

  if (!allLessonsCompleted) return 'Locked';
  if (isCategoryTestCompleted(category)) return 'Passed';
  return 'Ready';
};

// Format category name for display
const formatCategoryName = (categoryName: string) => {
  return categoryName
    .split('-')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
};

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
    useAlert().setAlert({ message: 'Error fetching lessons. Please try again later.', type: 'danger' });
    console.error('Error fetching data:', error);
  } finally {
    loading.value = false;
  }
}

// Navigate to category lessons view
function selectCategory(category: CategoryWithLessons) {
  if (!category.isAvailable) return;
  router.push({ path: `/lessons/${category.category}` });
}

onMounted(async () => {
  await fetchData();
});
</script>

<template>
  <div class="flex flex-col items-center min-h-screen bg-gray-800 text-white font-mono px-6 pt-12">
    <BackButton />

    <!-- Loading State -->
    <div v-if="loading" class="flex justify-center items-center h-64">
      <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-indigo-500"></div>
    </div>

    <!-- Category Grid View -->
    <div v-else class="w-full max-w-7xl">
      <!-- Header -->
      <div class="text-center mb-8">
        <h1 class="text-4xl font-bold mb-4 capitalize">{{ user?.lessonDifficulty }}</h1>
      </div>

      <!-- Category Cards Grid -->
      <div v-if="categories.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
        <CategoryCard
          v-for="category in categories"
          :key="category.category"
          :category-name="formatCategoryName(category.category)"
          :completed="getCategoryProgress(category).completed"
          :total="getCategoryProgress(category).total"
          :percentage="getCategoryProgress(category).percentage"
          :test-status="getCategoryTestStatus(category)"
          :is-available="category.isAvailable"
          :is-completed="getCategoryProgress(category).percentage === 100"
          @click="selectCategory(category)"
        />
      </div>

      <!-- No categories message -->
      <div v-else class="text-center py-16">
        <div class="text-6xl mb-4">📚</div>
        <h3 class="text-xl font-bold text-gray-400 mb-2">No lessons available</h3>
        <p class="text-gray-500">No lessons found for this difficulty level.</p>
      </div>
    </div>
  </div>
</template>