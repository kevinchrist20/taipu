<script setup lang="ts">
import Keyboard from "./components/Keyboard.vue";
import { ref } from "vue";

const testLesson = "a sad fad as ad da fad sad as asdf asdf asdf asdf a fad as sad dad ad as a fad asds adfd sadf asdf a sad dad fad as ad da a sad fad dad as a fad";

const currentPosition = ref(0);
const typedText = ref("");

function onKeyPress(key: string) {
  console.log('Key pressed:', key.toLowerCase());

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
}
</script>

<template>
  <div class="flex flex-col h-screen">
    <div class="flex-grow overflow-auto p-4">
      <h1 class="text-2xl font-bold mb-4">Typing Lesson</h1>

      <!-- Display test lesson -->
      <div class="lesson-area">
        <span v-for="(char, index) in testLesson" :key="index" :class="{
          'text-green-500': index < currentPosition && typedText[index] === char, // Correct text
          'text-red-500': index < currentPosition && typedText[index] !== char,  // Incorrect text
          'bg-yellow-200': index === currentPosition, // Highlight current character
        }">
          {{ char }}
        </span>
      </div>
    </div>

    <!-- Keyboard component -->
    <div class="flex-shrink-0">
      <Keyboard @key-pressed="onKeyPress" class="p-4" />
    </div>
  </div>
</template>

<style scoped>
.lesson-area {
  font-size: 1.5rem;
  line-height: 2rem;
  white-space: pre-wrap;
}

.bg-yellow-200 {
  background-color: #FEF3C7;
  /* Light yellow for the current character */
}

.text-green-500 {
  color: #22C55E;
  /* Green for correct characters */
}

.text-red-500 {
  color: #EF4444;
  /* Red for incorrect characters */
}
</style>