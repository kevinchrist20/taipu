<script setup lang="ts">
import Keyboard from "./components/Keyboard.vue";
import { ref } from "vue";

const pressedKey = ref("");

function onKeyPress(key: string) {
  pressedKey.value = key;
}
</script>

<template>
  <div class="flex flex-col h-screen">
    <div class="flex-grow overflow-auto p-4">
      <h1 class="text-2xl font-bold mb-4">Simple Keyboard Layout</h1>
      <textarea class="w-full h-32 p-2 border border-gray-400 rounded mb-4"></textarea>
      <p>You pressed: {{ pressedKey }}</p>
    </div>
    <div class="flex-shrink-0">
      <Keyboard @keyPressed="onKeyPress" class="p-4"/>
    </div>
  </div>
</template>

<style lang="scss" scoped></style>