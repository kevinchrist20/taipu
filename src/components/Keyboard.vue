<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, shallowRef, ref } from 'vue'
import { createKeyType } from '../types'

const { complete, next } = defineProps<{ complete: boolean, next?: string }>()
const emit = defineEmits<(e: 'key-pressed', key: string) => void>()

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
  <div class="flex flex-col space-y-2 w-full">
    <!-- Keyboard Rows -->
    <div
      v-for="(row, rowIndex) in keys"
      :key="rowIndex"
      class="key-row flex space-x-2"
    >
      <!-- Individual Keys -->
      <div
        v-for="(key, keyIndex) in row"
        :key="keyIndex"
        class="h-14 flex-grow min-w-0 px-2 py-2 text-xs sm:text-sm md:text-base lg:text-lg font-medium rounded-md text-center flex items-center justify-center transition-transform duration-150"
        :class="{
          'invisible': key.name === '',
          'grow flex-basis-25%': key.name === 'Space',
          'bg-key text-key-foreground': key.type !== 'Special' && key.name !== nextKey,
          'bg-primary text-primary-foreground shadow-md': key.name === nextKey || (key.name === 'Caps Lock' && isCapsLockActive),
          'bg-surface text-muted-foreground': key.type === 'Special',
          'hover:opacity-80': key.type !== 'Special',
          'shadow-inner transform scale-95': key.name.toLowerCase() === activeKey,
        }"
      >
        {{ key.name }}
      </div>
    </div>
  </div>
</template>
