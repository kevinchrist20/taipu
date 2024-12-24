<script setup lang="ts">
import { computed, reactive, ref } from 'vue'
import Rate from '../components/Rate.vue';
import { useLessonStore } from '../storage';
import Keyboard from '../components/Keyboard.vue';

const lessonStore = useLessonStore()
const testLesson = lessonStore.currentLesson;

const currentPosition = ref(0)
const typedText = ref('')
const nextKey = computed(() => testLesson?.content[currentPosition.value])
const secondsElapsed = ref(0)
const timerRunning = ref(false)

let timerInterval: number | undefined

const rateInfo = reactive({
    timer: ref('00:00'),
    percentComplete: computed(() => Math.round((typedText.value.length / (testLesson?.content.length || 0)) * 100)),
    accuracy: computed(() => {
        const correctChars = typedText.value.split('').filter((char, index) => char === testLesson?.content[index]).length
        return Math.round((correctChars / (currentPosition.value + 1)) * 100) || 100
    }),
    wpm: computed(() => {
        const words = typedText.value.split(' ').length
        const minutes = secondsElapsed.value / 60
        return minutes > 0 ? Math.round(words / minutes) : words
    })
})

// Function to start the timer
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

// Function to stop the timer
function stopTimer() {
    if (timerRunning.value && timerInterval) {
        clearInterval(timerInterval)
        timerRunning.value = false
    }
}

// Handle key press
function onKeyPress(key: string) {
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

    if (currentPosition.value >= (testLesson?.content.length || 0)) {
        stopTimer()
    }
}
</script>

<template>
    <div class="flex flex-col h-screen bg-gray-800 text-white">
        <div class="grid grid-cols-1 gap-5 px-10 2xl:px-20">
            <div class="flex-grow overflow-auto p-4 h-[40rem]">
                <h2 class="text-2xl font-bold mb-6 text-gray-300">
                    {{ testLesson?.title }}
                </h2>

                <Rate :rate-info />

                <!-- Main Content Area -->
                <main class="flex-grow overflow-auto">
                    <div class="bg-gray-700 rounded-lg p-6 text-4xl leading-relaxed shadow-md">
                        <span v-for="(char, index) in testLesson?.content" :key="index"
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
</style>
