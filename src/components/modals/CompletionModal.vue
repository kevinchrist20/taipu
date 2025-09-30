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
            return 'text-green-400';
        case 'B':
            return 'text-blue-400';
        case 'C':
            return 'text-yellow-400';
        case 'D':
            return 'text-red-400';
        default:
            return 'text-gray-400';
    }
}

function getBorderColor(grade: string) {
    switch (grade) {
        case 'S':
        case 'A':
            return 'border-green-500';
        case 'B':
            return 'border-blue-500';
        case 'C':
            return 'border-yellow-500';
        case 'D':
            return 'border-red-500';
        default:
            return 'border-gray-500';
    }
}

function getStatColor(value: number, required: number) {
    return value >= required ? 'text-green-400' : 'text-red-400';
}
</script>

<template>
    <Modal :show="show" size="md" :close-on-escape="false">
        <div class="border-2 rounded-lg p-6 -m-6" :class="getBorderColor(grade)">
            <div class="text-center">
                <!-- Header -->
                <div class="mb-6">
                    <div class="text-5xl mb-3">
                        {{ passed ? '🎉' : '💪' }}
                    </div>
                    <h2 class="text-3xl font-bold text-white mb-2">
                        {{ passed ? 'Lesson Completed!' : 'Keep Practicing!' }}
                    </h2>
                    <p class="text-gray-300" v-if="!passed">
                        You're making progress. Try again to meet the requirements!
                    </p>
                </div>

                <!-- Grade Display -->
                <div class="mb-8">
                    <div class="inline-block bg-gray-700 rounded-lg px-8 py-4">
                        <p class="text-gray-400 text-sm mb-1">Your Grade</p>
                        <div class="text-6xl font-bold" :class="getGradeColor(grade)">
                            {{ grade }}
                        </div>
                    </div>
                </div>

                <!-- Stats Grid -->
                <div class="grid grid-cols-2 gap-4 mb-8">
                    <div class="bg-gray-700 p-4 rounded-lg">
                        <h3 class="text-gray-400 text-sm mb-1">Accuracy</h3>
                        <div class="text-2xl font-bold" :class="getStatColor(accuracy, requiredAccuracy)">
                            {{ accuracy }}%
                        </div>
                        <div class="text-xs text-gray-400 mt-1">
                            Required: {{ requiredAccuracy }}%
                        </div>
                    </div>

                    <div class="bg-gray-700 p-4 rounded-lg">
                        <h3 class="text-gray-400 text-sm mb-1">WPM</h3>
                        <div class="text-2xl font-bold" :class="getStatColor(wpm, requiredWpm)">
                            {{ wpm }}
                        </div>
                        <div class="text-xs text-gray-400 mt-1">
                            Required: {{ requiredWpm }}
                        </div>
                    </div>

                    <div class="bg-gray-700 p-4 rounded-lg">
                        <h3 class="text-gray-400 text-sm mb-1">Time</h3>
                        <div class="text-2xl font-bold text-blue-400">
                            {{ timer }}
                        </div>
                    </div>

                    <div class="bg-gray-700 p-4 rounded-lg">
                        <h3 class="text-gray-400 text-sm mb-1">Characters</h3>
                        <div class="text-2xl font-bold text-blue-400">
                            {{ currentPosition }} / {{ totalCharacters }}
                        </div>
                    </div>
                </div>

                <!-- Action Buttons -->
                <div class="flex gap-3">
                    <button @click="emit('retry')"
                        class="flex-1 px-6 py-3 bg-yellow-600 hover:bg-yellow-700 text-white font-bold rounded-md transition transform hover:scale-105">
                        Retry
                    </button>

                    <button @click="passed ? emit('continue') : null" :disabled="!passed" :class="[
                        'flex-1 px-6 py-3 font-bold rounded-md transition',
                        passed
                            ? 'bg-green-600 hover:bg-green-700 text-white transform hover:scale-105 cursor-pointer'
                            : 'bg-gray-600 text-gray-400 cursor-not-allowed'
                    ]">
                        {{ passed ? 'Continue' : 'Locked' }}
                    </button>
                </div>

                <!-- Help Text -->
                <div v-if="!passed" class="mt-4 text-sm text-gray-400 bg-gray-700 rounded-lg p-3">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 inline mr-1" viewBox="0 0 20 20"
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