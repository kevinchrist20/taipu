<script setup lang="ts">
import Modal from './Modal.vue';

interface Props {
    show: boolean;
    grade: string;
    accuracy: number;
    wpm: number;
    timer: string;
    currentPosition: number;
    totalCharacters: number;
    requiredAccuracy: number;
    requiredWpm: number;
    passed: boolean;
}

defineProps<Props>();

const emit = defineEmits<{
    retry: [];
    continue: [];
}>();

function getGradeColor(grade: string) {
    switch (grade) {
        case 'S':
        case 'A':
            return 'text-success';
        case 'B':
            return 'text-accent-foreground';
        case 'C':
            return 'text-warning';
        case 'D':
            return 'text-destructive';
        default:
            return 'text-muted-foreground';
    }
}

function getBorderColor(grade: string) {
    switch (grade) {
        case 'S':
        case 'A':
            return 'border-success';
        case 'B':
            return 'border-ring';
        case 'C':
            return 'border-warning';
        case 'D':
            return 'border-destructive';
        default:
            return 'border-border';
    }
}

function getStatColor(value: number, required: number) {
    return value >= required ? 'text-success' : 'text-destructive';
}
</script>

<template>
    <Modal :show="show" size="md" :close-on-escape="false">
        <div class="border-2 rounded-2xl p-6 -m-6" :class="getBorderColor(grade)">
            <div class="text-center">
                <!-- Header -->
                <div class="mb-6">
                    <div class="text-4xl mb-3">
                        {{ passed ? '🎉' : '💪' }}
                    </div>
                    <h2 class="text-2xl font-bold text-foreground font-display mb-2">
                        {{ passed ? 'Lesson Completed!' : 'Keep Practicing!' }}
                    </h2>
                    <p class="text-muted-foreground text-sm" v-if="!passed">
                        You're making progress. Try again to meet the requirements!
                    </p>
                </div>

                <!-- Grade Display -->
                <div class="mb-6">
                    <div class="inline-block bg-surface rounded-xl px-8 py-4 border border-border">
                        <p class="text-muted-foreground text-xs mb-1">Your Grade</p>
                        <div class="text-5xl font-bold" :class="getGradeColor(grade)">
                            {{ grade }}
                        </div>
                    </div>
                </div>

                <!-- Stats Grid -->
                <div class="grid grid-cols-2 gap-3 mb-6">
                    <div class="bg-surface p-4 rounded-xl border border-border">
                        <h3 class="text-muted-foreground text-xs mb-1">Accuracy</h3>
                        <div class="text-2xl font-bold" :class="getStatColor(accuracy, requiredAccuracy)">
                            {{ accuracy }}%
                        </div>
                        <div class="text-xs text-muted-foreground mt-1">
                            Required: {{ requiredAccuracy }}%
                        </div>
                    </div>

                    <div class="bg-surface p-4 rounded-xl border border-border">
                        <h3 class="text-muted-foreground text-xs mb-1">WPM</h3>
                        <div class="text-2xl font-bold" :class="getStatColor(wpm, requiredWpm)">
                            {{ wpm }}
                        </div>
                        <div class="text-xs text-muted-foreground mt-1">
                            Required: {{ requiredWpm }}
                        </div>
                    </div>

                    <div class="bg-surface p-4 rounded-xl border border-border">
                        <h3 class="text-muted-foreground text-xs mb-1">Time</h3>
                        <div class="text-2xl font-bold text-primary">
                            {{ timer }}
                        </div>
                    </div>

                    <div class="bg-surface p-4 rounded-xl border border-border">
                        <h3 class="text-muted-foreground text-xs mb-1">Characters</h3>
                        <div class="text-2xl font-bold text-primary">
                            {{ currentPosition }} / {{ totalCharacters }}
                        </div>
                    </div>
                </div>

                <!-- Action Buttons -->
                <div class="flex gap-3">
                    <button @click="emit('retry')"
                        class="flex-1 px-5 py-2.5 bg-warning text-warning-foreground font-semibold rounded-xl hover:opacity-90 transition-all">
                        Retry
                    </button>

                    <button @click="passed ? emit('continue') : null" :disabled="!passed" :class="[
                        'flex-1 px-5 py-2.5 font-semibold rounded-xl transition-all',
                        passed
                            ? 'bg-primary text-primary-foreground hover:opacity-90 cursor-pointer'
                            : 'bg-surface text-muted-foreground border border-border cursor-not-allowed'
                    ]">
                        {{ passed ? 'Continue' : 'Locked' }}
                    </button>
                </div>

                <!-- Help Text -->
                <div v-if="!passed" class="mt-4 text-xs text-muted-foreground bg-surface rounded-xl p-3 border border-border text-left flex items-start gap-2">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5 mt-0.5 shrink-0 text-primary" viewBox="0 0 20 20"
                        fill="currentColor">
                        <path fill-rule="evenodd"
                            d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
                            clip-rule="evenodd" />
                    </svg>
                    Meet both accuracy and WPM requirements to unlock the next lesson
                </div>
            </div>
        </div>
    </Modal>
</template>