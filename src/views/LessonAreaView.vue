<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router';
import Rate from '../components/Rate.vue';
import { SessionStore } from '../storage';
import Keyboard from '../components/Keyboard.vue';
import LessonService from '../services/lesson.service';
import { routes } from '../constants';
import useLesson from '../composables/useLesson';
import useAlert from '../composables/useAlert';
import useTimer from '../composables/useTimer';
import useTypingState from '../composables/useTypingState';
import useRateInfo from '../composables/useRateInfo';
import useLessonProgress from '../composables/useLessonProgress';
import useModalState from '../composables/useModalState';
import useKeyboardHandler from '../composables/useKeyboardHandler';
import { LogOut, Pause, Play } from 'lucide-vue-next';
import CompletionModal from '../components/modals/CompletionModal.vue';
import PauseModal from '../components/modals/PauseModal.vue';
import ExitConfirmModal from '../components/modals/ExitConfirmModal.vue';
import TypingArea from '../components/TypingArea.vue';

const route = useRoute();
const router = useRouter();
const lessonId = parseInt(route.params.id as string);
const user = SessionStore.user;

// Composables
const { currentLesson: activeLesson, fetchLesson, loading: lessonLoading } = useLesson();
const { secondsElapsed, formattedTime, timerRunning, startTimer, stopTimer, resetTimer, cleanup } = useTimer();
const { currentPosition, typedText, nextKey, handleKeyInput, resetTyping } = useTypingState();

const lessonContent = computed(() => activeLesson.value?.content || '');
const rateInfo = useRateInfo({ typedText, currentPosition, secondsElapsed, lessonContent });

const {
    lessonCompleted,
    grade,
    userRequirements,
    passedLesson,
    setGrade,
    markCompleted,
    reset: resetProgress
} = useLessonProgress({
    user,
    accuracy: rateInfo.accuracy,
    wpm: rateInfo.wpm
});

const {
    showStatsModal,
    showPauseModal,
    showExitModal,
    openStatsModal,
    closeStatsModal,
    openPauseModal,
    closePauseModal,
    openExitModal,
    closeExitModal
} = useModalState();

function togglePause() {
    if (currentPosition.value === 0 || lessonCompleted.value || showStatsModal.value) return;
    openPauseModal();
    stopTimer();
}

function resumeLesson() {
    closePauseModal();
    startTimer();
}

function onKeyboardKeyPress(key: string) {
    if (lessonCompleted.value || showStatsModal.value || showPauseModal.value || showExitModal.value) return;

    if (!timerRunning.value) {
        startTimer();
    }

    const isCompleted = handleKeyInput(key, lessonContent.value);

    if (isCompleted) {
        stopTimer();
        showCompletionStats();
    }
}

function showCompletionStats() {
    setGrade();
    openStatsModal();
}

async function completeLesson() {
    if (!passedLesson.value || !activeLesson || !user) return;

    try {
        await LessonService.completeLesson(user.id, activeLesson.value!.id);
        markCompleted();
        closeStatsModal();
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
    closeStatsModal();
    resetTyping();
    resetTimer();
    resetProgress();
}

function confirmExit() {
    openExitModal();
}

function handleExitConfirm() {
    closeExitModal();
    returnToLessons();
}

// Setup keyboard handling
useKeyboardHandler({
    onTogglePause: togglePause,
    onResume: resumeLesson,
    showPauseModal,
    showStatsModal,
    showExitModal,
    currentPosition
});

onMounted(async () => {
    if (!lessonId || isNaN(lessonId)) {
        router.push({ path: routes.lessons });
        return;
    }

    const lesson = await fetchLesson(lessonId);
    if (!lesson) {
        router.push({ path: routes.lessons });
    }
});

onUnmounted(() => {
    cleanup();
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

                <Rate :rate-info="{
                    timer: formattedTime,
                    accuracy: rateInfo.accuracy.value,
                    percentComplete: rateInfo.percentComplete.value,
                    wpm: rateInfo.wpm.value
                }" />

                <!-- Main Content Area -->
                <main class="flex-grow overflow-auto">
                    <TypingArea :content="lessonContent" :current-position="currentPosition" :typed-text="typedText" />
                </main>

                <!-- Modals -->
                <CompletionModal :show="showStatsModal" :grade="grade" :accuracy="rateInfo.accuracy.value"
                    :wpm="rateInfo.wpm.value" :timer="formattedTime" :current-position="currentPosition"
                    :total-characters="activeLesson?.content.length || 0" :required-accuracy="userRequirements.accuracy"
                    :required-wpm="userRequirements.wpm" :passed="passedLesson" @retry="retryLesson"
                    @continue="completeLesson" @close="closeStatsModal" />

                <PauseModal :show="showPauseModal" @resume="resumeLesson" @exit="confirmExit" @close="resumeLesson" />

                <ExitConfirmModal :show="showExitModal" :has-progress="currentPosition > 0 && !lessonCompleted"
                    @confirm="handleExitConfirm" @close="closeExitModal" />

                <!-- Keyboard Area -->
                <footer class="bg-gray-900 p-4 fixed bottom-0 left-0 w-full">
                    <Keyboard :next="nextKey(lessonContent)" :complete="rateInfo.percentComplete.value === 100"
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
