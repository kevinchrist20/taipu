<script setup lang="ts">
import { defineEmits, ref, onMounted, onBeforeUnmount } from 'vue';
import { createKeyType } from '../types';

const emit = defineEmits(['key-pressed']);

const keys = ref([
  ['`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', 'Backspace'],
  ['Tab', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\\'],
  ['Caps Lock', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', 'Enter'],
  ['Shift', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', 'Shift'],
  ['Ctrl', '', 'Alt', 'Space', 'Alt', '', 'Ctrl'],
]).value.map(row => row.map(key => createKeyType(key)));

const specialKeys: { [key: string]: string } = {
  'Backspace': 'Backspace',
  'Tab': 'Tab',
  'CapsLock': 'Caps Lock',
  'Enter': 'Enter',
  'Shift': 'Shift',
  'Control': 'Ctrl',
  'Alt': 'Alt',
  ' ': 'Space',
}

const activeKey = ref('');

function handlePhysicalKeyPress(event: KeyboardEvent) {
  const key: string = event.key in specialKeys ? specialKeys[event.key] : event.key;
  activeKey.value = key;

  emit('key-pressed', key);
}

onMounted(() => {
  window.addEventListener('keydown', handlePhysicalKeyPress);
});

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handlePhysicalKeyPress);
});
</script>

<template>
  <div class="flex flex-col space-y-1 w-screen">
    <div v-for="(row, rowIndex) in keys" :key="rowIndex" class="key-row flex space-x-1">
      <div v-for="(key, keyIndex) in row" :key="keyIndex"
        class="keyboard-key px-1 py-1 text-xs sm:text-sm md:text-base lg:text-lg bg-gray-300 border border-gray-400 rounded hover:bg-gray-400 transition-colors capitalize"
        :class="{
          'invisible': key.name === '',
          'grow flex-basis-quarter': key.name === 'Space',
          'grow-[2]': key.type === 'Special',
          'bg-blue-400': key.name === activeKey,
        }">
        {{ key.name }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.keyboard-key {
  height: 55px;
  flex-grow: 1;
  min-width: 0;
}

.flex-basis-quarter {
  flex-basis: 25%;
}

.bg-blue-400 {
  background-color: #60a5fa;
}
</style>