<script setup lang="ts">
import { onMounted, ref } from 'vue';
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
        user.value = SessionStore.user;
        if (!user.value) return;

        lessons.value = await LessonService.getLessons(user.value.lessonDifficulty || '');
        completedLessons.value = await LessonService.getCompletedLessons(user.value.id);
    } catch (error) {
        console.error(error);
    }
}

function startLesson(lesson: Lesson) {
    lessonStore.setLesson(lesson)
    router.push({ path: routes.lessonArea });
}

function isLessonAvailable(index: number): boolean {
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
                    :class="`bg-gray-900 rounded-lg shadow-md p-6 transition-transform ${isLessonAvailable(index) ? 'hover:bg-gray-700 hover:scale-105' : 'opacity-60 cursor-not-allowed'}`">
                    <div class="flex justify-between items-start mb-2">
                        <h2 class="text-xl font-semibold">{{ lesson.title }}</h2>
                        <v-icon v-if="!isLessonAvailable(index)" name="fc-lock" />
                        <v-icon v-else-if="completedLessons.includes(lesson.id)" name="fc-ok" />
                    </div>
                    <p class="text-sm text-gray-400 mb-4">{{ lesson.content }}</p>
                    <div class="flex justify-between items-center">
                        <span class="text-sm text-gray-400">Difficulty: {{ lesson.difficulty }}</span>
                        <button
                            class="px-4 py-2 bg-indigo-600 hover:bg-indigo-500 text-white font-semibold rounded-md transition-transform hover:scale-105"
                            @click="startLesson(lesson)" :disabled="!isLessonAvailable(index)"
                            :class="{ 'opacity-50 cursor-not-allowed': !isLessonAvailable(index) }">
                            {{ completedLessons.includes(lesson.id) ? 'Retake' : 'Start Lesson' }}
                        </button>
                    </div>
                </div>
            </div>

            <!-- Error Message -->
            <p v-else class="text-red-500 text-center">No lessons available for this difficulty.</p>
        </div>
    </div>
</template>
