<script setup lang="ts">
import { onMounted, computed } from 'vue';
import { Zap, Target, TrendingUp, Clock } from 'lucide-vue-next';
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
  formatCategoryName,
  getCategoryDescription,
} = useCategories();

const {
  completedLessons,
  loading: completedLessonsLoading,
  fetchCompletedLessons,
} = useCompletedLessons();

const { getCategoryProgress } = useCategoryProgress(completedLessons);

const user = computed<User | null>(() => SessionStore.user);
const loading = computed(() => categoriesLoading.value || completedLessonsLoading.value);

const stats = [
  { label: 'Avg WPM',    value: '0',  icon: Zap },
  { label: 'Accuracy',   value: '0%', icon: Target },
  { label: 'Best WPM',   value: '0',  icon: TrendingUp },
  { label: 'Time Spent', value: '0m', icon: Clock },
];

function getCategoryDifficulty(category: CategoryWithLessons): string {
  return category.lessons[0]?.difficulty ?? 'beginner';
}

async function fetchData() {
  if (!user.value) return;
  try {
    await Promise.all([fetchCategories(), fetchCompletedLessons()]);
  } catch (error) {
    console.error('Error fetching data:', error);
  }
}

function selectCategory(category: CategoryWithLessons) {
  if (!category.isAvailable) return;
  router.push({ path: `/lessons/${category.category}` });
}

onMounted(fetchData);
</script>

<template>
  <div class="min-h-full bg-background px-6 py-8">
    <div class="max-w-7xl mx-auto">
      <!-- Page Title -->
      <h1 class="text-3xl font-bold font-display text-foreground mb-6">Dashboard</h1>

      <!-- Stats Row -->
      <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-10">
        <div
          v-for="stat in stats"
          :key="stat.label"
          class="bg-card border border-border rounded-2xl p-5"
        >
          <div class="flex items-center gap-2 text-muted-foreground text-sm mb-3">
            <component :is="stat.icon" :size="15" />
            {{ stat.label }}
          </div>
          <p class="text-2xl font-bold text-foreground">{{ stat.value }}</p>
        </div>
      </div>

      <!-- Section Heading -->
      <h2 class="text-xl font-bold font-mono text-foreground mb-5">Lesson Categories</h2>

      <!-- Loading -->
      <div v-if="loading" class="flex justify-center items-center h-48">
        <div class="w-8 h-8 rounded-full border-2 border-primary border-t-transparent animate-spin" />
      </div>

      <!-- Category Grid -->
      <div v-else-if="categories.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
        <CategoryCard
          v-for="category in categories"
          :key="category.category"
          :category-name="formatCategoryName(category.category)"
          :difficulty="getCategoryDifficulty(category)"
          :completed="getCategoryProgress(category).completed"
          :total="getCategoryProgress(category).total"
          :is-available="category.isAvailable"
          :description="getCategoryDescription(category.category)"
          @click="selectCategory(category)"
        />
      </div>

      <!-- Empty -->
      <div v-else class="text-center py-16">
        <p class="text-muted-foreground">No lessons available.</p>
      </div>
    </div>
  </div>
</template>
