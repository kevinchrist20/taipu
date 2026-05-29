<script setup lang="ts">
import { onMounted, onUnmounted, computed, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ArrowLeft } from 'lucide-vue-next';
import LessonPath from '../components/LessonPath.vue';
import { routes } from '../constants';
import { SessionStore } from '../storage';
import { Lesson, User } from '../types/bindings';
import useCategories from '../composables/useCategories';
import useCompletedLessons from '../composables/useCompletedLessons';
import AppButton from '../components/AppButton.vue';
import LoadingState from '../components/LoadingState.vue';
import ErrorState from '../components/ErrorState.vue';
import EmptyState from '../components/EmptyState.vue';
import { extractErrorMessage } from '../utils/error-utils';

const route = useRoute();
const router = useRouter();
const categoryName = route.params.category as string;
let disposed = false;
const loadError = ref('');
const categoryMissing = ref(false);

const {
    getCategoryByName,
    formatCategoryName,
    getCategoryDescription,
    fetchCategories,
    loading: categoriesLoading
} = useCategories();

const {
    completedLessons,
    fetchCompletedLessons,
    loading: completedLessonsLoading
} = useCompletedLessons();

const user = computed<User | null>(() => SessionStore.user);
const category = computed(() => getCategoryByName(categoryName));
const loading = computed(() => categoriesLoading.value || completedLessonsLoading.value);

async function fetchCategoryData() {
    loadError.value = '';
    categoryMissing.value = false;

    if (!user.value) {
        await router.push({ path: routes.home });
        return;
    }

    const results = await Promise.allSettled([
        fetchCategories(),
        fetchCompletedLessons()
    ]);

    if (disposed) {
        return;
    }

    const failed = results.find(result => result.status === 'rejected');
    if (failed && failed.status === 'rejected') {
        console.error('Error fetching category data:', failed.reason);
        loadError.value = extractErrorMessage(failed.reason, 'Unable to load category details right now.');
        return;
    }

    if (!category.value) {
        categoryMissing.value = true;
    }
}

async function goBackToLessons() {
    try {
        await router.push({ path: routes.lessons });
    } catch (error) {
        console.error('Navigation error:', error);
    }
}

function startLesson(lesson: Lesson) {
    if (!category.value?.isAvailable) return;
    router.push({ path: `/lesson/${lesson.id}` });
}

function startTest(test: Lesson) {
    if (!category.value?.isAvailable || !allLessonsCompleted.value) return;
    router.push({ path: `/lesson/${test.id}` });
}

const allLessonsCompleted = computed(() => {
    return category.value?.lessons.every(lesson => completedLessons.value.includes(lesson.id)) || false;
});

function difficultyStyle(d: string) {
    const level = d.toLowerCase();
    if (level === 'beginner') return { color: 'var(--success)', backgroundColor: 'color-mix(in oklch, var(--success) 12%, transparent)', borderColor: 'color-mix(in oklch, var(--success) 40%, transparent)' };
    if (level === 'intermediate') return { color: 'var(--warning)', backgroundColor: 'color-mix(in oklch, var(--warning) 12%, transparent)', borderColor: 'color-mix(in oklch, var(--warning) 40%, transparent)' };
    return { color: 'var(--destructive)', backgroundColor: 'color-mix(in oklch, var(--destructive) 12%, transparent)', borderColor: 'color-mix(in oklch, var(--destructive) 40%, transparent)' };
}

onMounted(async () => {
    await fetchCategoryData();
});

onUnmounted(() => {
    disposed = true;
});
</script>

<template>
    <div class="min-h-screen bg-background px-6 py-6">
        <div class="max-w-2xl mx-auto">
            <!-- Loading -->
            <LoadingState v-if="loading" message="Loading category..." min-height-class="h-64" />

            <ErrorState v-else-if="loadError" :message="loadError" retry-label="Retry" @retry="fetchCategoryData" />

            <!-- Not found -->
            <EmptyState
                v-else-if="categoryMissing"
                title="Category Not Found"
                description="The requested category could not be found for your current track."
            >
                <AppButton variant="secondary" size="sm" @click="goBackToLessons">Back to lessons</AppButton>
            </EmptyState>

            <template v-else-if="category">
                <!-- Back link -->
                <AppButton variant="ghost" size="sm" @click="router.back()" class="mb-6">
                    <ArrowLeft class="w-3.5 h-3.5" />
                    Back to selection
                </AppButton>

                <!-- Category meta -->
                <div class="mb-8">
                    <span
                        class="inline-block text-xs font-semibold uppercase tracking-wider px-2.5 py-1 rounded-full border mb-3"
                        :style="difficultyStyle(category.lessons[0]?.difficulty ?? 'beginner')">
                        {{ category.lessons[0]?.difficulty ?? 'BEGINNER' }}
                    </span>
                    <h1 class="text-3xl font-bold font-mono text-foreground mb-2">
                        {{ formatCategoryName(category.category) }}
                    </h1>
                    <p class="text-muted-foreground text-sm">
                        {{ getCategoryDescription(category.category) }}
                    </p>
                </div>

                <!-- Lesson list -->
                <LessonPath :lessons="category.lessons" :tests="category.tests" :completed-lessons="completedLessons"
                    :all-lessons-completed="allLessonsCompleted" @start-lesson="startLesson" @start-test="startTest" />
            </template>
        </div>
    </div>
</template>