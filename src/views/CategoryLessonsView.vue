<script setup lang="ts">
import { onMounted, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import BackButton from '../components/BackButton.vue';
import CategoryHeader from '../components/CategoryHeader.vue';
import LessonPath from '../components/LessonPath.vue';
import { routes } from '../constants';
import { SessionStore } from '../storage';
import { Lesson, User } from '../types/bindings';
import useCategories from '../composables/useCategories';
import useCompletedLessons from '../composables/useCompletedLessons';
import useCategoryProgress from '../composables/useCategoryProgress';

const route = useRoute();
const router = useRouter();
const categoryName = route.params.category as string;

const {
    getCategoryByName,
    formatCategoryName,
    fetchCategories,
    loading: categoriesLoading
} = useCategories();

const {
    completedLessons,
    fetchCompletedLessons,
    loading: completedLessonsLoading
} = useCompletedLessons();

const {
    getCategoryProgress,
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

onMounted(async () => {
    await fetchCategoryData();
});
</script>

<template>
    <div
        class="flex flex-col items-center min-h-screen bg-gradient-to-b from-gray-900 via-gray-800 to-gray-900 text-white font-sans px-4 pt-8 pb-20">
        <BackButton />

        <div v-if="loading" class="flex justify-center items-center h-64">
            <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-indigo-500"></div>
        </div>

        <div v-else-if="!category" class="text-center py-16">
            <div class="text-6xl mb-4">❓</div>
            <h3 class="text-xl font-bold text-gray-400 mb-2">Category Not Found</h3>
            <p class="text-gray-500">The requested category could not be found.</p>
        </div>

        <!-- Category Lessons View -->
        <div v-else class="w-full max-w-2xl">
            <!-- Category Header with Progress -->
            <CategoryHeader :category-name="formatCategoryName(category.category)"
                :completed="getCategoryProgress(category).completed" :total="getCategoryProgress(category).total"
                :percentage="getCategoryProgress(category).percentage" />

            <!-- Lesson Path -->
            <LessonPath :lessons="category.lessons" :tests="category.tests" :completed-lessons="completedLessons"
                :all-lessons-completed="allLessonsCompleted" @start-lesson="startLesson" @start-test="startTest" />
        </div>
    </div>
</template>