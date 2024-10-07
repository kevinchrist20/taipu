<script setup lang="ts">
import { defineEmits, ref } from 'vue'

const emit = defineEmits(['key-pressed'])

const keys = ref([
  ['`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', 'Backspace'],
  ['Tab', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '[', ']', '\\'],
  ['Caps Lock', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ';', '\'', 'Enter'],
  ['Shift', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '/', 'Shift'],
  ['Ctrl', '', 'Alt', 'Space', 'Alt', '', 'Ctrl'],
])

const keyboardRef = ref<HTMLDivElement | null>(null)

function handleKeyPress(key: string) {
  emit('key-pressed', key)
}
</script>

<template>
  <div ref="keyboardRef" class="flex flex-col space-y-1 w-screen">
    <div v-for="(row, rowIndex) in keys" :key="rowIndex" class="key-row flex space-x-1">
      <button
        v-for="(key, keyIndex) in row" :key="keyIndex" class="keyboard-key px-1 py-1 text-xs sm:text-sm md:text-base lg:text-lg bg-gray-300 border border-gray-400 rounded hover:bg-gray-400 transition-colors"
        :class="{
          'invisible': key === '',
          'grow': key === 'Space',
          'grow-[2]': key === 'Shift' || key === 'Backspace' || key === 'Enter' || key === 'Caps Lock',
        }"
        @click="handleKeyPress(key)"
      >
        {{ key }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.keyboard-key {
    cursor: pointer;
    height: 55px;
    flex-grow: 1;
    min-width: 0;
}
</style>
