<script setup lang="ts">
import { computed, reactive, ref } from 'vue'
import { sampleText } from './constants/misc';
import Header from './components/Header.vue'
import Keyboard from './components/Keyboard.vue'
import Rate from './components/Rate.vue';

const testLesson = sampleText

const currentPosition = ref(0)
const typedText = ref('')
const nextKey = computed(() => testLesson.content[currentPosition.value])
const secondsElapsed = ref(0)
const timerRunning = ref(false)
let timerInterval: number | undefined

const rateInfo = reactive({
  timer: ref('00:00'),
  percentComplete: computed(() => Math.round((typedText.value.length / testLesson.content.length) * 100)),
  accuracy: computed(() => {
  const correctChars = typedText.value.split('').filter((char, index) => char === testLesson.content[index]).length
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

  if (currentPosition.value >= testLesson.content.length)
    stopTimer()
}
</script>

<template>
  <div class="flex flex-col h-screen dark:bg-zinc-8 dark:text-white relative overflow-scroll">
    <Header />

    <div class="grid grid-cols-1 gap-5 px-10 2xl:px-20rem!">
      <div class="flex-grow overflow-auto p-4 h-40rem">
      <h2 class="text-lg font-medium leading-snug tracking-tight mb-4 text-zinc-6">
        {{ testLesson.title }}
      </h2>

      <Rate :rate-info />

      <div class="bg-zinc-50 dark:bg-zinc-8! rounded-b-lg p-5 text-5xl leading-relaxed">
        <span
          v-for="(char, index) in testLesson.content" :key="index" class="text-gray-400" :class="{
            'text-green-500': index < currentPosition && typedText[index] === char,
            'text-red-500': index < currentPosition && typedText[index] !== char,
            'current-text-color': index === currentPosition,
          }"
        >
          {{ char }}
        </span>
      </div>
      </div>

      <div class="w-full relative bottom-0! left-0">
        <Keyboard :next="nextKey" class="p-4" :complete="rateInfo.percentComplete === 100" @key-pressed="onKeyPress" />
      </div>
    </div>
  </div>
</template>

<style>
* {
  box-sizing: border-box;
}

.lesson-area {
  font-size: 1.5rem;
  line-height: 2rem;
  white-space: pre-wrap;
}

.current-text-color {
  background-color: #333;
  color: #fff;
}

.text-green-500 {
  color: #22C55E;
}

.text-red-500 {
  color: #EF4444;
  text-decoration: underline;
}
</style>
