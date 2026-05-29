<script setup lang="ts">
import { onMounted, computed, ref } from 'vue';
import { Zap, Target, TrendingUp, Clock } from 'lucide-vue-next';
import CategoryCard from '../components/CategoryCard.vue';
import LoadingState from '../components/LoadingState.vue';
import ErrorState from '../components/ErrorState.vue';
import EmptyState from '../components/EmptyState.vue';
import router from '../router';
import { SessionStore } from '../storage';
import { CategoryWithLessons, User, UserStatistics } from '../types/bindings';
import useCategories from '../composables/useCategories';
import useCompletedLessons from '../composables/useCompletedLessons';
import useCategoryProgress from '../composables/useCategoryProgress';
import StatsService from '../services/stats.service';
import { extractErrorMessage } from '../utils/error-utils';
import { formatDurationShort } from '../utils/ui-formatters';

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
const categoriesError = ref('');

const statsLoading = ref(false);
const statsError = ref('');
const dashboardStats = ref<UserStatistics | null>(null);

function formatTimeSpent(totalSeconds: number): string {
  return formatDurationShort(totalSeconds);
}

const stats = computed(() => {
  const base = dashboardStats.value;

  return [
    { label: 'Avg WPM', value: base ? Math.round(base.avgWpm).toString() : '0', icon: Zap, color: 'text-green-500' },
    { label: 'Accuracy', value: base ? `${Math.round(base.avgAccuracy)}%` : '0%', icon: Target, color: 'text-blue-500' },
    { label: 'Best WPM', value: base ? Math.round(base.bestWpm).toString() : '0', icon: TrendingUp, color: 'text-yellow-500' },
    { label: 'Time Spent', value: base ? formatTimeSpent(base.totalTimeSeconds) : '0m', icon: Clock, color: 'text-red-500' },
  ];
});

async function loadDashboardStats() {
  if (!user.value) return;

  statsLoading.value = true;
  statsError.value = '';

  try {
    dashboardStats.value = await StatsService.getUserStatistics(user.value.id, user.value.lessonDifficulty || 'BEGINNER');
  } catch (error) {
    console.error('Error loading dashboard stats:', error);
    statsError.value = 'Unable to load latest stats.';
    dashboardStats.value = null;
  } finally {
    statsLoading.value = false;
  }
}

function getCategoryDifficulty(category: CategoryWithLessons): string {
  return category.lessons[0]?.difficulty ?? 'beginner';
}

async function fetchData() {
  if (!user.value) return;

  categoriesError.value = '';
  const [categoriesResult, completedResult, statsResult] = await Promise.allSettled([
    fetchCategories(),
    fetchCompletedLessons(),
    loadDashboardStats()
  ]);

  if (categoriesResult.status === 'rejected' || completedResult.status === 'rejected') {
    const reason = categoriesResult.status === 'rejected'
      ? categoriesResult.reason
      : (completedResult as PromiseRejectedResult).reason;
    categoriesError.value = extractErrorMessage(reason, 'Unable to load lesson categories right now.');
    console.error('Error loading categories/completions:', reason);
  }

  if (statsResult.status === 'rejected') {
    console.error('Error loading dashboard stats:', statsResult.reason);
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
        <div v-for="stat in stats" :key="stat.label" class="bg-card border border-border rounded-2xl p-5">
          <div class="flex items-center gap-2 text-muted-foreground text-sm mb-3">
            <component :is="stat.icon" :size="15" :class="stat.color" />
            {{ stat.label }}
          </div>
          <p class="text-2xl font-bold text-foreground font-mono">{{ stat.value }}</p>
        </div>
      </div>

      <p v-if="statsLoading" class="text-xs text-muted-foreground mb-6">Refreshing dashboard stats...</p>
      <p v-else-if="statsError" class="text-xs text-destructive mb-6">{{ statsError }}</p>

      <!-- Section Heading -->
      <h2 class="text-xl font-bold font-mono text-foreground mb-5">Lesson Categories</h2>

      <LoadingState v-if="loading" message="Loading lesson categories..." min-height-class="h-48" />

      <ErrorState
        v-else-if="categoriesError"
        :message="categoriesError"
        retry-label="Retry"
        @retry="fetchData"
      />

      <!-- Category Grid -->
      <div v-else-if="categories.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
        <CategoryCard v-for="category in categories" :key="category.category"
          :category-name="formatCategoryName(category.category)" :difficulty="getCategoryDifficulty(category)"
          :completed="getCategoryProgress(category).completed" :total="getCategoryProgress(category).total"
          :is-available="category.isAvailable" :description="getCategoryDescription(category.category)"
          @click="selectCategory(category)" />
      </div>

      <EmptyState v-else title="No lessons available" description="No categories are currently available for this profile." />
    </div>
  </div>
</template>
