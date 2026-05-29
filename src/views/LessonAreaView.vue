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
import useCompletedLessons from '../composables/useCompletedLessons';
import useCategories from '../composables/useCategories';
import { OctagonX, Pause, Play } from 'lucide-vue-next';
import CompletionModal from '../components/modals/CompletionModal.vue';
import PauseModal from '../components/modals/PauseModal.vue';
import ExitConfirmModal from '../components/modals/ExitConfirmModal.vue';
import TypingArea from '../components/TypingArea.vue';
import AppButton from '../components/AppButton.vue';

const route = useRoute();
const router = useRouter();
const lessonId = Number.parseInt(route.params.id as string);
const user = SessionStore.user;

// Composables
const { currentLesson: activeLesson, fetchLesson, loading: lessonLoading } = useLesson();
const { secondsElapsed, formattedTime, timerRunning, startTimer, stopTimer, resetTimer, cleanup } = useTimer();
const { currentPosition, typedText, nextKey, handleKeyInput, resetTyping } = useTypingState();

const lessonContent = computed(() => activeLesson.value?.content || '');
const rateInfo = useRateInfo({ typedText, currentPosition, secondsElapsed, lessonContent });
const { refreshCompletedLessons } = useCompletedLessons();
const { fetchCategories } = useCategories();

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
        await LessonService.completeLesson(
            user.id,
            activeLesson.value!.id,
            rateInfo.wpm.value,
            rateInfo.accuracy.value,
            grade.value,
            secondsElapsed.value
        );
        markCompleted();

        const refreshResults = await Promise.allSettled([
            refreshCompletedLessons(),
            fetchCategories(true)
        ]);

        const [completedLessonsRefresh, categoriesRefresh] = refreshResults;
        if (completedLessonsRefresh.status === 'rejected' || categoriesRefresh.status === 'rejected') {
            console.warn('Lesson completion succeeded, but some caches failed to refresh.', {
                completedLessonsRefresh,
                categoriesRefresh
            });
        }

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
    if (!lessonId || Number.isNaN(lessonId)) {
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
    <div v-if="lessonLoading || !activeLesson"
        class="flex flex-1 min-h-0 justify-center items-center bg-background text-foreground overflow-hidden">
        <div class="text-center">
            <div class="w-10 h-10 rounded-full border-2 border-primary border-t-transparent animate-spin mx-auto mb-4">
            </div>
            <p class="text-muted-foreground">Loading lesson...</p>
        </div>
    </div>

    <!-- Lesson Content -->
    <div v-else class="flex flex-1 min-h-0 flex-col bg-background text-foreground overflow-hidden">
        <div class="grid grid-cols-1 gap-5 px-10 2xl:px-20 flex-1 min-h-0">
            <div class="flex min-h-0 flex-col p-4">
                <div class="flex justify-between items-center mb-6">
                    <div class="flex items-center">
                        <h2 class="text-2xl font-bold text-foreground font-mono">
                            {{ activeLesson?.title }}
                        </h2>
                    </div>
                    <div class="flex space-x-3">
                        <!-- Pause/Resume Button -->
                        <AppButton @click="togglePause"
                            :disabled="currentPosition === 0 || lessonCompleted || showStatsModal"
                            :variant="showPauseModal ? 'secondary' : 'ghost'" class="p-2.5 transition-all duration-200">
                            <Pause v-if="!showPauseModal" :size="24" />
                            <Play v-else :size="24" />
                        </AppButton>

                        <!-- Exit Button -->
                        <AppButton @click="confirmExit"
                            :disabled="currentPosition === 0 || lessonCompleted || showStatsModal" variant="danger"
                            class="p-2.5 transition-all duration-200">
                            <OctagonX :size="24" />
                        </AppButton>
                    </div>
                </div>

                <Rate :rate-info="{
                    timer: formattedTime,
                    accuracy: rateInfo.accuracy.value,
                    percentComplete: rateInfo.percentComplete.value,
                    wpm: rateInfo.wpm.value
                }" />

                <!-- Main Content Area -->
                <main class="flex-grow min-h-0 overflow-auto pb-4">
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
                    @confirm="handleExitConfirm" @cancel="closeExitModal" />

                <!-- Keyboard Area -->
                <footer class="border-t border-border p-4 shrink-0">
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

.lesson-content-enter-active {
    animation: bounce-in 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

@keyframes bounce-in {
    0% {
        transform: scale(0.97);
        opacity: 0;
    }

    100% {
        transform: scale(1);
        opacity: 1;
    }
}
</style>
