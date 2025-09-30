<script setup lang="ts">
import { computed, onMounted, onUnmounted, reactive, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router';
import Rate from '../components/Rate.vue';
import { SessionStore } from '../storage';
import Keyboard from '../components/Keyboard.vue';
import LessonService from '../services/lesson.service';
import { routes } from '../constants';
import { difficultyRequirements } from '../types';
import useLesson from '../composables/useLesson';
import useAlert from '../composables/useAlert';
import { LogOut, Pause, Play } from 'lucide-vue-next';
import CompletionModal from '../components/modals/CompletionModal.vue';
import PauseModal from '../components/modals/PauseModal.vue';
import ExitConfirmModal from '../components/modals/ExitConfirmModal.vue';

const route = useRoute();
const router = useRouter();
const lessonId = parseInt(route.params.id as string);
const user = SessionStore.user;

const { currentLesson: activeLesson, fetchLesson, loading: lessonLoading } = useLesson();

const currentPosition = ref(0);
const typedText = ref('');
const nextKey = computed(() => activeLesson.value?.content[currentPosition.value])
const secondsElapsed = ref(0);
const timerRunning = ref(false);
const lessonCompleted = ref(false);
const showStatsModal = ref(false);
const grade = ref('');
const userLessonRequirements = ref({ accuracy: 0, wpm: 0 });
const showPauseModal = ref(false);
const showExitModal = ref(false);

let timerInterval: number | undefined

const rateInfo = reactive({
    timer: ref('00:00'),
    percentComplete: computed(() =>
        Math.round((typedText.value.length / (activeLesson.value?.content.length || 0)) * 100)
    ),
    accuracy: computed(() => {
        const correctChars = typedText.value.split('').filter((char, index) =>
            char === activeLesson.value?.content[index]
        ).length
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
    showPauseModal.value = true;
    stopTimer();
}

function resumeLesson() {
    showPauseModal.value = false;
    startTimer();
}

function handleKeyPress(e: KeyboardEvent) {
    if (e.key === 'Escape') {
        if (showPauseModal.value) {
            resumeLesson();
        } else if (!showStatsModal.value && !showExitModal.value && currentPosition.value > 0) {
            togglePause();
        }
    }
}

function onKeyboardKeyPress(key: string) {
    if (lessonCompleted.value || showStatsModal.value || showPauseModal.value) return;

    if (!timerRunning.value)
        startTimer()

    if (key.length === 1) {
        typedText.value += key
        currentPosition.value++
    } else if (key === 'Backspace' && currentPosition.value > 0) {
        typedText.value = typedText.value.slice(0, -1)
        currentPosition.value--
    } else if (key === 'Space') {
        typedText.value += ' '
        currentPosition.value++
    }

    if (currentPosition.value >= (activeLesson.value?.content.length || 0)) {
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
        await LessonService.completeLesson(user.id, activeLesson.value!.id);
        lessonCompleted.value = true;
        showStatsModal.value = false;
        returnToLessons();
    } catch (error) {
        console.error("Error completing lesson:", error);
        useAlert().setAlert({ message: 'Error completing lesson. Please try again.', type: 'danger' });
    }
}

function returnToLessons() {
    router.back();
}

function retryLesson() {
    showStatsModal.value = false;
    typedText.value = '';
    currentPosition.value = 0;
    secondsElapsed.value = 0;
    rateInfo.timer = '00:00';
}

function confirmExit() {
    showExitModal.value = true;
    if (showPauseModal.value) {
        showPauseModal.value = false;
    }
}

function handleExitConfirm() {
    showExitModal.value = false;
    returnToLessons();
}

function getDifficultyRequirements() {
    if (!user?.lessonDifficulty) return { accuracy: 0, wpm: 0 };
    userLessonRequirements.value = difficultyRequirements[user.lessonDifficulty.toLowerCase()] || {
        accuracy: 0, wpm: 0
    };
}

function calculateGrade() {
    const requirements = userLessonRequirements.value;

    if (rateInfo.accuracy >= requirements.accuracy + 10 && rateInfo.wpm >= requirements.wpm + 10) return 'S';
    if (rateInfo.accuracy >= requirements.accuracy + 5 && rateInfo.wpm >= requirements.wpm + 5) return 'A';
    if (rateInfo.accuracy >= requirements.accuracy && rateInfo.wpm >= requirements.wpm) return 'B';
    if (rateInfo.accuracy >= requirements.accuracy - 10 && rateInfo.wpm >= requirements.wpm - 5) return 'C';
    return 'D';
}

onMounted(async () => {
    if (!lessonId || isNaN(lessonId)) {
        router.push({ path: routes.lessons });
        return;
    }

    const lesson = await fetchLesson(lessonId);
    if (!lesson) {
        router.push({ path: routes.lessons });
        return;
    }

    getDifficultyRequirements();

    window.addEventListener('keydown', handleKeyPress);
});

onUnmounted(() => {
    window.removeEventListener('keydown', handleKeyPress);
    if (timerInterval) {
        clearInterval(timerInterval);
    }
});
</script>

<template>
    <!-- Loading State -->
    <div v-if="lessonLoading || !activeLesson" class="flex justify-center items-center h-screen bg-gray-800 text-white">
        <div class="text-center">
            <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-indigo-500 mx-auto mb-4"></div>
            <p class="text-gray-400">Loading lesson...</p>
        </div>
    </div>

    <!-- Lesson Content -->
    <div v-else class="flex flex-col h-screen bg-gray-800 text-white relative">
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
                            :disabled="currentPosition === 0 || lessonCompleted || showStatsModal"
                            class="p-3 rounded-xl transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed"
                            :class="{
                                'bg-blue-600 hover:bg-blue-700 hover:scale-110': !showPauseModal,
                                'bg-green-600 hover:bg-green-700 hover:scale-110': showPauseModal
                            }">
                            <Pause v-if="!showPauseModal" :size="24" />
                            <Play v-else :size="24" />
                        </button>

                        <!-- Exit Button -->
                        <button @click="confirmExit"
                            class="p-3 rounded-xl bg-red-600 hover:bg-red-700 transition-all duration-300 hover:scale-110">
                            <LogOut :size="24" />
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

                <!-- Modals -->
                <CompletionModal :show="showStatsModal" :grade="grade" :accuracy="rateInfo.accuracy" :wpm="rateInfo.wpm"
                    :timer="rateInfo.timer" :current-position="currentPosition"
                    :total-characters="activeLesson?.content.length || 0"
                    :required-accuracy="userLessonRequirements.accuracy" :required-wpm="userLessonRequirements.wpm"
                    :passed="passedLesson" @retry="retryLesson" @continue="completeLesson"
                    @close="showStatsModal = false" />

                <PauseModal :show="showPauseModal" @resume="resumeLesson" @exit="confirmExit" @close="resumeLesson" />

                <ExitConfirmModal :show="showExitModal" :has-progress="currentPosition > 0 && !lessonCompleted"
                    @confirm="handleExitConfirm" @cancel="showExitModal = false" @close="showExitModal = false" />

                <!-- Keyboard Area -->
                <footer class="bg-gray-900 p-4 fixed bottom-0 left-0 w-full">
                    <Keyboard :next="nextKey" :complete="rateInfo.percentComplete === 100"
                        @key-pressed="onKeyboardKeyPress" />
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
