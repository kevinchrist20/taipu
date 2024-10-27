<script setup lang="ts">
import Keyboard from "./components/Keyboard.vue";
import { ref, computed } from "vue";

const testLesson = "a sad fad as ad da fad sad as asdf asdf asdf asdf a fad as sad dad ad as a fad asds adfd sadf asdf a sad dad fad as ad da a sad fad dad as a fad";

const currentPosition = ref(0);
const typedText = ref("");
const accuracy = computed(()=>{
  const correctChars = typedText.value.split('').filter((char, index) => char === testLesson[index]).length;
  return Math.round((correctChars / (currentPosition.value + 1)) * 100) || 100;
});
const timer = ref("00:00");
const percentComplete = computed(()=> Math.round((typedText.value.length / testLesson.length) * 100));
const wpm = computed(()=>{
  const words = typedText.value.split(' ').length;
  const minutes = secondsElapsed.value / 60;
  return minutes > 0 ? Math.round(words / minutes) : words;
});

const secondsElapsed = ref(0);
const timerRunning = ref(false);
let timerInterval: number | undefined = undefined;

// Function to start the timer
function startTimer() {
  if (!timerRunning.value) {
    timerRunning.value = true;
    timerInterval = setInterval(() => {
      secondsElapsed.value++;
      const minutes = Math.floor(secondsElapsed.value / 60);
      const seconds = secondsElapsed.value % 60;
      timer.value = `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    }, 1000);
  }
}

// Function to stop the timer
function stopTimer() {
  if (timerRunning.value && timerInterval) {
    clearInterval(timerInterval);
    timerRunning.value = false;
  }
}

// Handle key press
function onKeyPress(key: string) {
  if (!timerRunning.value) {
    startTimer();
  }

  if (key.length === 1) {
    typedText.value += key;
    currentPosition.value++;
  } else if (key === 'Backspace' && currentPosition.value > 0) {
    typedText.value = typedText.value.slice(0, -1);
    currentPosition.value--;
  } else if (key === 'Space') {
    typedText.value += ' ';
    currentPosition.value++;
  }

  if (currentPosition.value >= testLesson.length) {
    stopTimer();
  }
}
</script>

<template>
  <div class="flex flex-col h-screen">
    <div class="flex-grow overflow-auto p-4">
      <h1 class="text-2xl font-bold mb-4">Typing Lesson</h1>

      <!-- Accuracy, Timer, Percent Complete, WPM -->
      <div class="flex justify-between mb-4">
        <div>
          <span class="text-gray-400">Timer: {{ timer }}</span> &nbsp;
          <span class="text-gray-400">Accuracy: {{ accuracy }}%</span>
        </div>
        <div>
          <span class="text-gray-400">Percent Complete: {{ percentComplete }}%</span> &nbsp;
          <span class="text-gray-400">WPM: {{ wpm }}</span>
        </div>
      </div>

      <!-- Lesson area -->
      <div class="lesson-area">
        <span v-for="(char, index) in testLesson" class="text-gray-400" :key="index" :class="{
          'text-green-500': index < currentPosition && typedText[index] === char,
          'text-red-500': index < currentPosition && typedText[index] !== char,
          'current-text-color': index === currentPosition,
        }">
          {{ char }}
        </span>
      </div>
    </div>

    <!-- Keyboard component -->
    <div class="flex-shrink-0">
      <Keyboard @key-pressed="onKeyPress" class="p-4" :complete="percentComplete === 100" />
    </div>
  </div>
</template>

<style scoped>
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
