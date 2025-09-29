<script setup lang="ts">
import { onMounted, computed } from 'vue';
import BackButton from '../components/BackButton.vue';
import CategoryCard from '../components/CategoryCard.vue';
import router from '../router';
import { SessionStore } from '../storage';
import { CategoryWithLessons, User } from '../types/bindings';
import useCategories from '../composables/useCategories';
import useCompletedLessons from '../composables/useCompletedLessons';
import useCategoryProgress from '../composables/useCategoryProgress';

const { 
  categories, 
  loading: categoriesLoading, 
  fetchCategories, 
  formatCategoryName 
} = useCategories();

const { 
  completedLessons, 
  loading: completedLessonsLoading, 
  fetchCompletedLessons 
} = useCompletedLessons();

const { 
  getCategoryProgress, 
  getCategoryTestStatus 
} = useCategoryProgress(completedLessons);

const user = computed<User | null>(() => SessionStore.user);
const loading = computed(() => categoriesLoading.value || completedLessonsLoading.value);

async function fetchData() {
  if (!user.value) return;
  
  try {
    await Promise.all([
      fetchCategories(),
      fetchCompletedLessons()
    ]);
  } catch (error) {
    console.error('Error fetching data:', error);
  }
}

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