<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { Lesson, User } from '../types/bindings';
import LessonService from '../services/lesson.service';
import { SessionStore } from '../storage';
import router from '../router';
import { routes } from '../constants';

const lessons = ref<Lesson[]>([])
const user = ref<User | null>(null)

async function getLessons() {
    try {
        user.value = SessionStore.user as User;
        lessons.value = await LessonService.getLessons(user.value.lessonDifficulty || '');
    } catch (error) {
        console.error(error);
    }
}

function startLesson(lesson: Lesson) {
    router.push({ path: routes.lessonArea, query: { lesson: JSON.stringify(lesson) } });
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
                <div v-for="lesson in lessons" :key="lesson.id"
                    class="bg-gray-900 rounded-lg shadow-md p-6 hover:bg-gray-700 transition-transform transform hover:scale-105">
                    <h2 class="text-xl font-semibold mb-2">{{ lesson.title }}</h2>
                    <p class="text-sm text-gray-400 mb-4">{{ lesson.content }}</p>
                    <div class="flex justify-between items-center">
                        <span class="text-sm text-gray-400">Difficulty: {{ lesson.difficulty }}</span>
                        <button
                            class="px-4 py-2 bg-indigo-600 hover:bg-indigo-500 text-white font-semibold rounded-md transition-transform transform hover:scale-105"
                            @click="startLesson(lesson)">
                            Start Lesson
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