<script setup lang="ts">

interface Props {
  content: string;
  currentPosition: number;
  typedText: string;
}

defineProps<Props>();

const getCharacterClass = (index: number, char: string, currentPosition: number, typedText: string) => {
  if (index < currentPosition && typedText[index] === char) {
    return 'text-green-400';
  }
  if (index < currentPosition && typedText[index] !== char) {
    return 'text-red-500';
  }
  if (index === currentPosition) {
    return 'bg-indigo-600 text-white';
  }
  return 'text-gray-500';
};
</script>

<template>
  <div class="bg-gray-700 rounded-lg p-6 text-4xl leading-relaxed shadow-md">
    <span 
      v-for="(char, index) in content" 
      :key="index"
      class="transition-colors duration-150"
      :class="getCharacterClass(index, char, currentPosition, typedText)"
    >
      {{ char }}
    </span>
  </div>
</template>

<style scoped>
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
</style>