<script setup lang="ts">
import { onMounted, ref, computed } from 'vue';
import { Lesson, User } from '../types/bindings';
import LessonService from '../services/lesson.service';
import { SessionStore, useLessonStore } from '../storage';
import router from '../router';
import { routes } from '../constants';

const lessons = ref<Lesson[]>([])
const completedLessons = ref<number[]>([])
const user = ref<User | null>(null)

const lessonStore = useLessonStore()

async function getLessons() {
    try {
        user.value = SessionStore.user as User;
        lessons.value = await LessonService.getLessons(user.value.lessonDifficulty || '');
        if (user.value) {
            completedLessons.value = await LessonService.getCompletedLessons(user.value.id);
        }
    } catch (error) {
        console.error(error);
    }
}

function startLesson(lesson: Lesson) {
    lessonStore.setLesson(lesson)
    router.push({ path: routes.lessonArea });
}

// Check if lesson is available or locked
function isLessonAvailable(lessonId: number, index: number): boolean {
    // First lesson is always available
    if (index === 0) return true;
    
    // If previous lesson is completed, this lesson is available
    const previousLessonId = lessons.value[index - 1]?.id;
    return completedLessons.value.includes(previousLessonId);
}

onMounted(async () => await getLessons())
</script>

<template>
    <div class="flex flex-col items-center min-h-screen bg-gray-800 text-white font-mono px-6 pt-12">
        <!-- Back Button -->
        <button @click="router.back()"
            class="absolute top-4 left-4 p-2 rounded-full bg-gray-700 hover:bg-gray-600 text-white transition-transform transform hover:scale-110">
            <!-- Back Icon -->
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
            </svg>
        </button>

        <!-- Header -->
        <h1 class="text-3xl font-bold mb-8 capitalize">{{ user?.lessonDifficulty }} Lessons</h1>

        <!-- Scrollable Lessons Container -->
        <div class="w-full max-w-4xl overflow-y-auto mb-10">
            <!-- Lessons Grid -->
            <div v-if="lessons.length" class="grid grid-cols-1 sm:grid-cols-2 gap-6 p-4">
                <div v-for="(lesson, index) in lessons" :key="lesson.id"
                    :class="`bg-gray-900 rounded-lg shadow-md p-6 transition-transform ${isLessonAvailable(lesson.id, index) ? 'hover:bg-gray-700 hover:scale-105' : 'opacity-60 cursor-not-allowed'}`">
                    <div class="flex justify-between items-start mb-2">
                        <h2 class="text-xl font-semibold">{{ lesson.title }}</h2>
                        <div v-if="!isLessonAvailable(lesson.id, index)" class="text-yellow-500">
                            <!-- Lock Icon -->
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clip-rule="evenodd" />
                            </svg>
                        </div>
                        <div v-else-if="completedLessons.includes(lesson.id)" class="text-green-500">
                            <!-- Completed Icon -->
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                            </svg>
                        </div>
                    </div>
                    <p class="text-sm text-gray-400 mb-4">{{ lesson.content }}</p>
                    <div class="flex justify-between items-center">
                        <span class="text-sm text-gray-400">Difficulty: {{ lesson.difficulty }}</span>
                        <button
                            class="px-4 py-2 bg-indigo-600 hover:bg-indigo-500 text-white font-semibold rounded-md transition-transform hover:scale-105"
                            @click="startLesson(lesson)"
                            :disabled="!isLessonAvailable(lesson.id, index)"
                            :class="{ 'opacity-50 cursor-not-allowed': !isLessonAvailable(lesson.id, index) }">
                            {{ isLessonAvailable(lesson.id, index) ? 'Start Lesson' : 'Locked' }}
                        </button>
                    </div>
                </div>
            </div>

            <!-- Error Message -->
            <p v-else class="text-red-500 text-center">No lessons available for this difficulty.</p>
        </div>
    </div>
</template>

<style scoped></style>