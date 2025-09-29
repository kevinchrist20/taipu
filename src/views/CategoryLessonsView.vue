<script setup lang="ts">
import { onMounted, ref, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import BackButton from '../components/BackButton.vue';
import CircularProgressIndicator from '../components/CircularProgressIndicator.vue';
import { routes } from '../constants';
import LessonService from '../services/lesson.service';
import { SessionStore, useLessonStore } from '../storage';
import { CategoryWithLessons, Lesson, User } from '../types/bindings';
import useAlert from '../utils/useAlert';
import { Check, BookOpen, Trophy, Lock, Star } from 'lucide-vue-next';

const route = useRoute();
const router = useRouter();
const categoryName = route.params.category as string;

const category = ref<CategoryWithLessons | null>(null);
const completedLessons = ref<number[]>([]);
const user = ref<User | null>(null);
const lessonStore = useLessonStore();
const loading = ref(false);

// Compute category statistics
const getCategoryProgress = computed(() => {
    if (!category.value) return { completed: 0, total: 0, percentage: 0 };

    const totalLessons = category.value.lessons.length;
    const completedCount = category.value.lessons.filter(lesson =>
        completedLessons.value.includes(lesson.id)
    ).length;
    const percentage = totalLessons > 0 ? Math.round((completedCount / totalLessons) * 100) : 0;

    return {
        completed: completedCount,
        total: totalLessons,
        percentage
    };
});

// Format category name for display
const formatCategoryName = (categoryName: string) => {
    return categoryName
        .split('-')
        .map(word => word.charAt(0).toUpperCase() + word.slice(1))
        .join(' ');
};

async function fetchCategoryData() {
    try {
        loading.value = true;
        user.value = SessionStore.user;
        if (!user.value) {
            router.push({ path: routes.home });
            return;
        }

        const categories = await LessonService.getLessonsByCategories(
            user.value.lessonDifficulty || '',
            user.value.id
        );

        const foundCategory = categories.find(cat => cat.category === categoryName);
        if (!foundCategory) {
            useAlert().setAlert({ message: 'Category not found', type: 'danger' });
            router.push({ path: routes.lessons });
            return;
        }

        category.value = foundCategory;
        completedLessons.value = await LessonService.getCompletedLessons(user.value.id);
    } catch (error) {
        useAlert().setAlert({ message: 'Error fetching category data. Please try again later.', type: 'danger' });
        console.error('Error fetching category data:', error);
        router.push({ path: routes.lessons });
    } finally {
        loading.value = false;
    }
}

function startLesson(lesson: Lesson) {
    if (!category.value?.isAvailable) return;
    lessonStore.setLesson(lesson);
    router.push({ path: routes.lessonArea });
}

function startTest(test: Lesson) {
    if (!category.value?.isAvailable || !allLessonsCompleted.value) return;
    lessonStore.setLesson(test);
    router.push({ path: routes.lessonArea });
}

function isLessonComplete(lessonId: number): boolean {
    return completedLessons.value.includes(lessonId);
}

const allLessonsCompleted = computed(() => {
    return category.value?.lessons.every(l => isLessonComplete(l.id)) || false;
});

// Check if lesson is locked (previous not completed)
function isLessonLocked(index: number): boolean {
    if (!category.value?.isAvailable) return true;
    if (index === 0) return false;
    const previousLesson = category.value.lessons[index - 1];
    return !isLessonComplete(previousLesson.id);
}

onMounted(async () => {
    await fetchCategoryData();
});
</script>

<template>
    <div class="flex flex-col items-center min-h-screen bg-gradient-to-b from-gray-900 via-gray-800 to-gray-900 text-white font-sans px-4 pt-8 pb-20">
        <BackButton />

        <!-- Loading State -->
        <div v-if="loading" class="flex justify-center items-center h-64">
            <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-indigo-500"></div>
        </div>

        <!-- Category not found -->
        <div v-else-if="!category" class="text-center py-16">
            <div class="text-6xl mb-4">❓</div>
            <h3 class="text-xl font-bold text-gray-400 mb-2">Category Not Found</h3>
            <p class="text-gray-500">The requested category could not be found.</p>
        </div>

        <!-- Category Lessons View -->
        <div v-else class="w-full max-w-2xl">
            <!-- Category Header with Progress -->
            <div class="bg-gradient-to-r from-indigo-600 to-purple-600 rounded-2xl p-6 mb-8 shadow-xl">
                <div class="flex items-center justify-between">
                    <div>
                        <h1 class="text-3xl font-bold mb-2">{{ formatCategoryName(category.category) }}</h1>
                        <p class="text-indigo-100 text-sm">{{ getCategoryProgress.completed }} of {{ getCategoryProgress.total }} lessons</p>
                    </div>
                    <div class="flex items-center gap-4">
                        <CircularProgressIndicator 
                            :percentage="getCategoryProgress.percentage"
                            :diameter="80"
                            :stroke-width="8"
                        />
                    </div>
                </div>
            </div>

            <!-- Lesson Path (Duolingo Style) -->
            <div class="relative">
                <!-- Vertical Path Line -->
                <div class="absolute left-12 top-0 bottom-0 w-1 bg-gray-700/50"></div>

                <div class="space-y-8">
                    <!-- Lesson Nodes -->
                    <div v-for="(lesson, index) in category.lessons" :key="lesson.id"
                        class="relative flex items-center gap-4">
                        
                        <!-- Lesson Icon/Node -->
                        <div class="relative z-10">
                            <button
                                @click="startLesson(lesson)"
                                :disabled="isLessonLocked(index)"
                                class="w-24 h-24 rounded-full flex items-center justify-center transition-all duration-300 shadow-lg group"
                                :class="{
                                    'bg-gradient-to-br from-green-500 to-green-600 hover:scale-110 cursor-pointer': isLessonComplete(lesson.id),
                                    'bg-gradient-to-br from-indigo-500 to-purple-600 hover:scale-110 cursor-pointer animate-pulse': !isLessonComplete(lesson.id) && !isLessonLocked(index),
                                    'bg-gray-700 cursor-not-allowed opacity-50': isLessonLocked(index)
                                }"
                            >
                                <Check v-if="isLessonComplete(lesson.id)" :size="40" class="text-white" />
                                <Lock v-else-if="isLessonLocked(index)" :size="32" class="text-gray-400" />
                                <BookOpen v-else :size="36" class="text-white group-hover:scale-110 transition-transform" />
                            </button>
                            
                            <!-- Stars for completed -->
                            <div v-if="isLessonComplete(lesson.id)" class="absolute -top-2 -right-2">
                                <div class="bg-yellow-400 rounded-full p-1.5 shadow-lg">
                                    <Star :size="16" class="text-yellow-900 fill-yellow-900" />
                                </div>
                            </div>
                        </div>

                        <!-- Lesson Card -->
                        <div 
                            @click="!isLessonLocked(index) ? startLesson(lesson) : null"
                            class="flex-1 bg-gray-800 rounded-2xl p-5 border-2 transition-all duration-300"
                            :class="{
                                'border-green-500 hover:border-green-400 cursor-pointer hover:shadow-lg hover:shadow-green-500/20': isLessonComplete(lesson.id),
                                'border-indigo-500 hover:border-indigo-400 cursor-pointer hover:shadow-lg hover:shadow-indigo-500/20': !isLessonComplete(lesson.id) && !isLessonLocked(index),
                                'border-gray-700 opacity-60': isLessonLocked(index)
                            }"
                        >
                            <div class="flex items-start justify-between mb-2">
                                <h3 class="text-lg font-bold text-white">{{ lesson.title }}</h3>
                                <span class="px-2 py-1 bg-gray-700 rounded-full text-xs font-medium text-gray-300">
                                    Lesson {{ index + 1 }}
                                </span>
                            </div>
                            <p class="text-sm text-gray-400 line-clamp-2 mb-3">{{ lesson.content }}</p>
                            <div class="flex items-center justify-between">
                                <span class="text-xs font-medium px-2 py-1 rounded-full"
                                    :class="{
                                        'bg-green-900/50 text-green-400': lesson.difficulty === 'easy',
                                        'bg-yellow-900/50 text-yellow-400': lesson.difficulty === 'medium',
                                        'bg-red-900/50 text-red-400': lesson.difficulty === 'hard'
                                    }">
                                    {{ lesson.difficulty }}
                                </span>
                                <span v-if="isLessonComplete(lesson.id)" class="text-green-400 text-sm font-medium flex items-center gap-1">
                                    <Check :size="16" />
                                    Completed
                                </span>
                                <span v-else-if="isLessonLocked(index)" class="text-gray-500 text-sm font-medium flex items-center gap-1">
                                    <Lock :size="16" />
                                    Locked
                                </span>
                                <span v-else class="text-indigo-400 text-sm font-medium">
                                    Start →
                                </span>
                            </div>
                        </div>
                    </div>

                    <!-- Category Test -->
                    <div v-if="category.tests.length" class="relative flex items-center gap-4 mt-12">
                        <!-- Trophy Icon/Node -->
                        <div class="relative z-10">
                            <button
                                v-for="test in category.tests"
                                :key="test.id"
                                @click="startTest(test)"
                                :disabled="!allLessonsCompleted"
                                class="w-28 h-28 rounded-full flex items-center justify-center transition-all duration-300 shadow-2xl"
                                :class="{
                                    'bg-gradient-to-br from-yellow-500 via-yellow-600 to-orange-600 hover:scale-110 cursor-pointer': allLessonsCompleted && !isLessonComplete(test.id),
                                    'bg-gradient-to-br from-green-500 to-green-600 hover:scale-110 cursor-pointer': isLessonComplete(test.id),
                                    'bg-gray-700 cursor-not-allowed opacity-50': !allLessonsCompleted
                                }"
                            >
                                <Trophy v-if="isLessonComplete(test.id)" :size="48" class="text-white" />
                                <Trophy v-else-if="allLessonsCompleted" :size="48" class="text-white animate-bounce" />
                                <Lock v-else :size="36" class="text-gray-400" />
                            </button>
                        </div>

                        <!-- Test Card -->
                        <div 
                            class="flex-1 bg-gradient-to-br from-yellow-900/30 to-orange-900/30 rounded-2xl p-6 border-2 transition-all duration-300"
                            :class="{
                                'border-yellow-500 hover:border-yellow-400 cursor-pointer hover:shadow-lg hover:shadow-yellow-500/20': allLessonsCompleted,
                                'border-gray-700 opacity-60': !allLessonsCompleted
                            }"
                            @click="allLessonsCompleted && category.tests.length ? startTest(category.tests[0]) : null"
                        >
                            <div class="flex items-center gap-3 mb-3">
                                <div class="w-12 h-12 bg-yellow-500 rounded-xl flex items-center justify-center shadow-lg">
                                    <Trophy :size="24" class="text-yellow-900" />
                                </div>
                                <div>
                                    <h3 class="text-xl font-bold text-white">Unit Test</h3>
                                    <p class="text-sm text-gray-400">Final challenge</p>
                                </div>
                            </div>
                            
                            <p class="text-sm text-gray-300 mb-4">
                                {{ allLessonsCompleted 
                                    ? 'Complete this test to unlock the next category!' 
                                    : 'Complete all lessons above to unlock this test.' 
                                }}
                            </p>

                            <div v-if="isLessonComplete(category.tests[0].id)" 
                                class="flex items-center gap-2 text-green-400 font-medium bg-green-900/30 px-4 py-2 rounded-lg">
                                <Check :size="20" />
                                Test Passed! Next category unlocked.
                            </div>
                            <div v-else-if="allLessonsCompleted" 
                                class="flex items-center justify-between bg-yellow-500 text-yellow-900 font-bold px-4 py-3 rounded-lg hover:bg-yellow-400 transition-colors">
                                <span>Take the Test</span>
                                <Trophy :size="20" />
                            </div>
                            <div v-else 
                                class="flex items-center gap-2 text-gray-500 font-medium bg-gray-800 px-4 py-2 rounded-lg">
                                <Lock :size="16" />
                                Locked
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>