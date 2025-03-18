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

// Handle key press
function onKeyPress(key: string) {
    if (lessonCompleted.value || showStatsModal.value) return;

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
    router.push({ path: routes.lessons });
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
    const reqs = userLessonRequirements.value;

    if (rateInfo.accuracy >= reqs.accuracy + 10 && rateInfo.wpm >= reqs.wpm + 10) return 'S';
    if (rateInfo.accuracy >= reqs.accuracy + 5 && rateInfo.wpm >= reqs.wpm + 5) return 'A';
    if (rateInfo.accuracy >= reqs.accuracy && rateInfo.wpm >= reqs.wpm) return 'B';
    if (rateInfo.accuracy >= reqs.accuracy - 10 && rateInfo.wpm >= reqs.wpm - 5) return 'C';
    return 'D';
}

onMounted(() => {
    getDifficultyRequirements();
});
</script>

<template>
    <div class="flex flex-col h-screen bg-gray-800 text-white">
        <div class="grid grid-cols-1 gap-5 px-10 2xl:px-20">
            <div class="flex-grow overflow-auto p-4 h-[40rem]">
                <div class="flex justify-between items-center mb-6">
                    <h2 class="text-2xl font-bold text-gray-300">
                        {{ activeLesson?.title }}
                    </h2>
                    <!-- <button v-if="lessonCompleted" @click="returnToLessons"
                        class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white font-semibold rounded-md transition-transform hover:scale-105">
                        Return to Lessons
                    </button> -->
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

                <!-- Keyboard Area -->
                <footer class="bg-gray-900 p-4 fixed bottom-0 left-0 w-full">
                    <Keyboard :next="nextKey" :complete="rateInfo.percentComplete === 100" @key-pressed="onKeyPress" />
                </footer>
            </div>
        </div>
    </div>
</template>

<style>
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
