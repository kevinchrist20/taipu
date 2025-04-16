<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import Rate from '../components/Rate.vue';
import { SessionStore, useLessonStore } from '../storage';
import Keyboard from '../components/Keyboard.vue';
import LessonService from '../services/lesson.service';
import router from '../router';
import { routes } from '../constants';
import { difficultyRequirements } from '../types';

const lessonStore = useLessonStore()
const activeLesson = lessonStore.currentLesson;
const user = SessionStore.user;

const currentPosition = ref(0);
const typedText = ref('');
const nextKey = computed(() => activeLesson?.content[currentPosition.value])
const secondsElapsed = ref(0);
const timerRunning = ref(false);
const lessonCompleted = ref(false);
const showStatsModal = ref(false);
const isPaused = ref(false);
const grade = ref('');
const userLessonRequirements = ref({ accuracy: 0, wpm: 0 });

let timerInterval: number | undefined

const rateInfo = reactive({
    timer: ref('00:00'),
    percentComplete: computed(() => Math.round((typedText.value.length / (activeLesson?.content.length || 0)) * 100)),
    accuracy: computed(() => {
        const correctChars = typedText.value.split('').filter((char, index) => char === activeLesson?.content[index]).length
        return Math.round((correctChars / (currentPosition.value > 0 ? currentPosition.value : 1)) * 100) || 100
    }),
    wpm: computed(() => {
        // Standard WPM calculation assumes 5 characters (including spaces) = 1 word
        const charCount = typedText.value.length;
        const wordCount = charCount / 5;
        const minutes = secondsElapsed.value / 60;
        return minutes > 0 ? Math.round(wordCount / minutes) : 0
    })
})

const passedLesson = computed(() => {
    if (!user?.lessonDifficulty || !activeLesson) return false;

    const requirements = difficultyRequirements[user.lessonDifficulty.toLowerCase()];
    if (!requirements) return false;

    return rateInfo.accuracy >= requirements.accuracy && rateInfo.wpm >= requirements.wpm;
})

function startTimer() {
    if (!timerRunning.value) {
        timerRunning.value = true
        timerInterval = setInterval(() => {
            secondsElapsed.value++
            const minutes = Math.floor(secondsElapsed.value / 60)
            const seconds = secondsElapsed.value % 60
            rateInfo.timer = `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
        }, 1000)
    }
}

function stopTimer() {
    if (timerRunning.value && timerInterval) {
        clearInterval(timerInterval)
        timerRunning.value = false
    }
}

function togglePause() {
    if (currentPosition.value === 0 || lessonCompleted.value || showStatsModal.value) return;

    isPaused.value = !isPaused.value;

    if (isPaused.value) {
        stopTimer();
    } else {
        startTimer();
    }
}

// Handle key press
function onKeyPress(key: string) {
    if (lessonCompleted.value || showStatsModal.value || isPaused.value) return;

    if (!timerRunning.value)
        startTimer()

    if (key.length === 1) {
        typedText.value += key
        currentPosition.value++
    }
    else if (key === 'Backspace' && currentPosition.value > 0) {
        typedText.value = typedText.value.slice(0, -1)
        currentPosition.value--
    }
    else if (key === 'Space') {
        typedText.value += ' '
        currentPosition.value++
    }

    if (currentPosition.value >= (activeLesson?.content.length || 0)) {
        stopTimer()
        showCompletionStats()
    }
}

function showCompletionStats() {
    showStatsModal.value = true;
    grade.value = calculateGrade();
}

async function completeLesson() {
    if (!passedLesson.value || !activeLesson || !user) return;

    try {
        await LessonService.completeLesson(user.id, activeLesson.id);
        lessonCompleted.value = true;
        showStatsModal.value = false;
        returnToLessons();
    } catch (error) {
        console.error("Error completing lesson:", error);
    }
}

function returnToLessons() {
    router.replace({ path: routes.lessons });
}

function retryLesson() {
    // Reset stats and timer
    showStatsModal.value = false;
    typedText.value = '';
    currentPosition.value = 0;
    secondsElapsed.value = 0;
    rateInfo.timer = '00:00';
}

function getDifficultyRequirements() {
    if (!user?.lessonDifficulty) return { accuracy: 0, wpm: 0 };
    userLessonRequirements.value = difficultyRequirements[user.lessonDifficulty.toLowerCase()] || { accuracy: 0, wpm: 0 };
}

function calculateGrade() {
    const requirements = userLessonRequirements.value;

    if (rateInfo.accuracy >= requirements.accuracy + 10 && rateInfo.wpm >= requirements.wpm + 10) return 'S';
    if (rateInfo.accuracy >= requirements.accuracy + 5 && rateInfo.wpm >= requirements.wpm + 5) return 'A';
    if (rateInfo.accuracy >= requirements.accuracy && rateInfo.wpm >= requirements.wpm) return 'B';
    if (rateInfo.accuracy >= requirements.accuracy - 10 && rateInfo.wpm >= requirements.wpm - 5) return 'C';
    return 'D';
}

function confirmExit() {
    if (currentPosition.value > 0 && !lessonCompleted.value && !showStatsModal.value) {
        if (confirm('Are you sure you want to exit this lesson? Your progress will be lost.')) {
            returnToLessons();
        }
    } else {
        returnToLessons();
    }
}

onMounted(() => {
    getDifficultyRequirements();

    // Add keyboard shortcuts for pause and exit
    window.addEventListener('keydown', (e) => {
        if (e.key === 'Escape') {
            togglePause();
        }
    });
});
</script>

<template>
    <div class="flex flex-col h-screen bg-gray-800 text-white relative">
        <div class="grid grid-cols-1 gap-5 px-10 2xl:px-20">
            <div class="flex-grow overflow-auto p-4 h-[40rem]">
                <div class="flex justify-between items-center mb-6">
                    <div class="flex items-center">
                        <h2 class="text-2xl font-bold text-gray-300">
                            {{ activeLesson?.title }}
                        </h2>
                    </div>
                    <div class="flex space-x-3">
                        <!-- Pause/Resume Button -->
                        <button @click="togglePause"
                            class="p-2 rounded-full bg-blue-600 hover:bg-blue-700 text-white transition-transform hover:scale-110"
                            :class="{ 'bg-green-600 hover:bg-green-700': isPaused }">
                            <svg v-if="!isPaused" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none"
                                viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M10 9v6m4-6v6m7-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none"
                                viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                        </button>

                        <!-- Exit Button -->
                        <button @click="confirmExit"
                            class="p-2 rounded-full bg-red-600 hover:bg-red-700 text-white transition-transform hover:scale-110">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                                stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                            </svg>
                        </button>
                    </div>
                </div>

                <Rate :rate-info />

                <!-- Main Content Area -->
                <main class="flex-grow overflow-auto">
                    <div class="bg-gray-700 rounded-lg p-6 text-4xl leading-relaxed shadow-md">
                        <span v-for="(char, index) in activeLesson?.content" :key="index"
                            class="transition-colors duration-150" :class="{
                                'text-green-400': index < currentPosition && typedText[index] === char,
                                'text-red-500': index < currentPosition && typedText[index] !== char,
                                'bg-indigo-600 text-white': index === currentPosition,
                                'text-gray-500': index > currentPosition,
                            }">
                            {{ char }}
                        </span>
                    </div>
                </main>

                <!-- Completion Stats Modal -->
                <div v-if="showStatsModal"
                    class="fixed inset-0 flex items-center justify-center bg-gray-900 bg-opacity-80 z-20">
                    <div class="bg-gray-800 border-2 border-indigo-500 rounded-lg shadow-2xl p-8 max-w-md w-full mx-4 transform transition-all"
                        :class="{
                            'border-green-500': grade === 'S' || grade === 'A',
                            'border-blue-500': grade === 'B',
                            'border-yellow-500': grade === 'C',
                            'border-red-500': grade === 'D'
                        }">
                        <div class="text-center">
                            <h2 class="text-3xl font-bold mb-2 text-white">
                                {{ passedLesson ? '🎉 Lesson Completed!' : '😩 Almost There!' }}
                            </h2>

                            <div class="mt-6 mb-8 flex flex-col items-center">
                                <div class="text-5xl font-bold mb-2" :class="{
                                    'text-green-400': grade === 'S' || grade === 'A',
                                    'text-blue-400': grade === 'B',
                                    'text-yellow-400': grade === 'C',
                                    'text-red-400': grade === 'D'
                                }">
                                    Grade: {{ grade }}
                                </div>
                            </div>

                            <div class="grid grid-cols-2 gap-6 mb-8">
                                <div class="bg-gray-700 p-4 rounded-lg">
                                    <h3 class="text-gray-400 text-sm mb-1">Accuracy</h3>
                                    <div class="text-2xl font-bold" :class="{
                                        'text-green-400': rateInfo.accuracy >= userLessonRequirements.accuracy,
                                        'text-red-400': rateInfo.accuracy < userLessonRequirements.accuracy
                                    }">
                                        {{ rateInfo.accuracy }}%
                                    </div>
                                    <div class="text-xs text-gray-400 mt-1">
                                        Required: {{ userLessonRequirements.accuracy }}%
                                    </div>
                                </div>

                                <div class="bg-gray-700 p-4 rounded-lg">
                                    <h3 class="text-gray-400 text-sm mb-1">WPM</h3>
                                    <div class="text-2xl font-bold" :class="{
                                        'text-green-400': rateInfo.wpm >= userLessonRequirements.wpm,
                                        'text-red-400': rateInfo.wpm < userLessonRequirements.wpm
                                    }">
                                        {{ rateInfo.wpm }}
                                    </div>
                                    <div class="text-xs text-gray-400 mt-1">
                                        Required: {{ userLessonRequirements.wpm }}
                                    </div>
                                </div>

                                <div class="bg-gray-700 p-4 rounded-lg">
                                    <h3 class="text-gray-400 text-sm mb-1">Time</h3>
                                    <div class="text-2xl font-bold text-blue-400">
                                        {{ rateInfo.timer }}
                                    </div>
                                </div>

                                <div class="bg-gray-700 p-4 rounded-lg">
                                    <h3 class="text-gray-400 text-sm mb-1">Characters</h3>
                                    <div class="text-2xl font-bold text-blue-400">
                                        {{ currentPosition }} / {{ activeLesson?.content.length }}
                                    </div>
                                </div>
                            </div>

                            <div class="flex gap-4 justify-center">
                                <button @click="retryLesson"
                                    class="px-6 py-3 bg-yellow-600 hover:bg-yellow-700 text-white font-bold rounded-md transition transform hover:scale-105">
                                    Retry
                                </button>

                                <button @click="passedLesson ? completeLesson() : null" :class="{
                                    'px-6 py-3 bg-green-600 hover:bg-green-700 text-white font-bold rounded-md transition transform hover:scale-105': passedLesson,
                                    'px-6 py-3 bg-gray-600 text-gray-400 font-bold rounded-md cursor-not-allowed': !passedLesson
                                }">
                                    {{ passedLesson ? 'Continue' : 'Practice More' }}
                                </button>
                            </div>

                            <div v-if="!passedLesson" class="mt-4 text-sm text-gray-400">
                                Meet the required accuracy and WPM to continue to the next lesson.
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Pause Overlay -->
                <div v-if="isPaused"
                    class="absolute inset-0 flex items-center justify-center bg-gray-900 bg-opacity-80 rounded-lg">
                    <div class="text-center p-8">
                        <h3 class="text-3xl font-bold mb-4">Lesson Paused</h3>
                        <p class="text-gray-300 mb-6">Click the button below to resume your lesson.</p>
                        <button @click="togglePause"
                            class="px-6 py-3 bg-green-600 hover:bg-green-700 text-white font-bold rounded-md transition transform hover:scale-105 flex items-center justify-center mx-auto">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20"
                                fill="currentColor">
                                <path fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z"
                                    clip-rule="evenodd" />
                            </svg>
                            Resume
                        </button>
                        <button @click="confirmExit"
                            class="mt-4 px-6 py-2 bg-gray-700 hover:bg-gray-600 text-white font-bold rounded-md transition transform hover:scale-105 flex items-center justify-center mx-auto">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20"
                                fill="currentColor">
                                <path fill-rule="evenodd"
                                    d="M3 3a1 1 0 00-1 1v12a1 1 0 001 1h12a1 1 0 001-1V4a1 1 0 00-1-1H3zm11 4a1 1 0 10-2 0v4.586l-1.293-1.293a1 1 0 10-1.414 1.414l3 3a1 1 0 001.414 0l3-3a1 1 0 00-1.414-1.414L14 11.586V7z"
                                    clip-rule="evenodd" />
                            </svg>
                            Exit Lesson
                        </button>
                    </div>
                </div>

                <!-- Keyboard Area -->
                <footer class="bg-gray-900 p-4 fixed bottom-0 left-0 w-full">
                    <Keyboard :next="nextKey" :complete="rateInfo.percentComplete === 100" @key-pressed="onKeyPress" />
                </footer>
            </div>
        </div>
    </div>
</template>

<style scoped>
.lesson-area {
    font-size: 1.5rem;
    line-height: 2rem;
    white-space: pre-wrap;
}

.bg-indigo-600 {
    background-color: #4F46E5;
}

.text-green-400 {
    color: #4ADE80;
}

.text-red-500 {
    color: #EF4444;
    text-decoration: underline;
}

.text-gray-500 {
    color: #6B7280;
}

.text-gray-300 {
    color: #D1D5DB;
}

@keyframes bounce-in {
    0% {
        transform: scale(0.8);
        opacity: 0;
    }

    70% {
        transform: scale(1.05);
    }

    100% {
        transform: scale(1);
        opacity: 1;
    }
}

.bg-gray-800 {
    animation: bounce-in 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}
</style>
