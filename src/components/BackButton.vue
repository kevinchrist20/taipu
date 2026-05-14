<script setup lang="ts">
import { useRouter } from 'vue-router';

const props = defineProps({
  customAction: {
    type: Function,
    default: null
  },
  position: {
    type: String,
    default: 'absolute top-4 left-4' // Default position
  }
});

const router = useRouter();

function handleClick() {
  if (props.customAction) {
    props.customAction();
  } else {
    router.back();
  }
}
</script>

<template>
  <button @click="handleClick"
    :class="`p-2 rounded-full bg-surface-elevated hover:bg-card border border-border text-foreground transition-all hover:scale-105 ${position}`">
    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
      stroke="currentColor" stroke-width="2">
      <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
    </svg>
  </button>
</template>
