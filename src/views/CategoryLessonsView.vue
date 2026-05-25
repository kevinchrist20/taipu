<script setup lang="ts">
import { onMounted, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ArrowLeft } from 'lucide-vue-next';
import LessonPath from '../components/LessonPath.vue';
import { routes } from '../constants';
import { SessionStore } from '../storage';
import { Lesson, User } from '../types/bindings';
import useCategories from '../composables/useCategories';
import useCompletedLessons from '../composables/useCompletedLessons';
import useCategoryProgress from '../composables/useCategoryProgress';
import AppButton from '../components/AppButton.vue';

const route = useRoute();
const router = useRouter();
const categoryName = route.params.category as string;

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

const {
    getCategoryProgress: _getCategoryProgress,
} = useCategoryProgress(completedLessons);

const user = computed<User | null>(() => SessionStore.user);
const category = computed(() => getCategoryByName(categoryName));
const loading = computed(() => categoriesLoading.value || completedLessonsLoading.value);

async function fetchCategoryData() {
    if (!user.value) {
        router.push({ path: routes.home });
        return;
    }

    try {
        await Promise.all([
            fetchCategories(),
            fetchCompletedLessons()
        ]);

        if (!category.value) {
            router.push({ path: routes.lessons });
            return;
        }
    } catch (error) {
        console.error('Error fetching category data:', error);
        router.push({ path: routes.lessons });
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
</script>

<template>
    <div class="min-h-screen bg-background px-6 py-6">
        <div class="max-w-2xl mx-auto">
            <!-- Loading -->
            <div v-if="loading" class="flex justify-center items-center h-64">
                <div class="w-8 h-8 rounded-full border-2 border-primary border-t-transparent animate-spin" />
            </div>

            <!-- Not found -->
            <div v-else-if="!category" class="text-center py-16">
                <div class="text-6xl mb-4">❓</div>
                <h3 class="text-xl font-bold text-muted-foreground mb-2">Category Not Found</h3>
                <p class="text-muted-foreground text-sm">The requested category could not be found.</p>
            </div>

            <template v-else>
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