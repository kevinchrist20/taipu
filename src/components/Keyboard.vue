<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, shallowRef, ref } from 'vue'
import { createKeyType } from '../types'

const { complete, next } = defineProps<{ complete: boolean, next?: string }>()
const emit = defineEmits<{ (e: 'key-pressed', key: string): void }>()

const keyboard = [
  ['`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', 'Backspace'],
  ['Tab', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\\'],
  ['Caps Lock', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', 'Enter'],
  ['Shift', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', 'Shift'],
  ['Ctrl', '', 'Alt', 'Space', 'Alt', '', 'Ctrl'],
]

const keys = keyboard.map(row => row.map(key => createKeyType(key)))
const nextKey = computed(() => next === ' ' ? 'Space' : next)
const activeKey = shallowRef()
const isCapsLockActive = ref(false)

const specialKeys = {
  'Backspace': 'Backspace',
  'Tab': 'Tab',
  'CapsLock': 'Caps Lock',
  'Enter': 'Enter',
  'Shift': 'Shift',
  'Control': 'Ctrl',
  'Alt': 'Alt',
  ' ': 'Space',
} as const


function handlePhysicalKeyPress(event: KeyboardEvent) {
  const key: string = (event.key) in specialKeys ? specialKeys[event.key as keyof typeof specialKeys] : event.key
  if (!complete)
    emit('key-pressed', key)
  activeKey.value = key
}

function releaseButton() {
  activeKey.value = ''
}

onMounted(() => {
  window.addEventListener('keydown', handlePhysicalKeyPress)
  window.addEventListener('keyup', releaseButton)
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handlePhysicalKeyPress)
  window.removeEventListener('keyup', releaseButton)
})
</script>

<template>
  <div class="flex flex-col space-y-1 w-full">
    <div v-for="(row, rowIndex) in keys" :key="rowIndex" class="key-row flex space-x-1">
      <div
        v-for="(key, keyIndex) in row" :key="keyIndex"
        class="h-55px flex-grow-1 min-w-0 px-1 py-1 text-xs sm:text-sm md:text-base lg:text-lg bg-zinc-1 dark:bg-zinc-7 dark:border-0 border border-zinc-2 font-medium rounded hover:bg-zinc-400 transition-colors capitalize text-center flex items-center justify-center"
        :class="{
          'invisible': key.name === '',
          'grow flex-basis-25%': key.name === 'Space',
          'grow-[2]': key.type === 'Special',
          'bg-amber-500 text-white': key.name === nextKey || (key.name === 'Caps Lock' && isCapsLockActive),
          'shadow-inner outline-none shadow-zinc-600 transition transform scale-95 duration-150 ease-in-out' : key.name.toLowerCase() === activeKey,
        }"
      >
        {{ key.name }}
      </div>
    </div>
  </div>
</template>
