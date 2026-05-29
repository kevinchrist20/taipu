<script setup lang="ts">

interface Props {
  content: string;
  currentPosition: number;
  typedText: string;
}

defineProps<Props>();

const getCharacterClass = (index: number, char: string, currentPosition: number, typedText: string) => {
  if (index < currentPosition && typedText[index] === char) {
    return 'text-success';
  }
  if (index < currentPosition && typedText[index] !== char) {
    return 'text-destructive underline';
  }
  if (index === currentPosition) {
    return 'bg-primary text-primary-foreground rounded';
  }
  return 'text-muted-foreground';
};
</script>

<template>
  <div class="bg-surface rounded-bl-xl rounded-br-xl p-6 text-4xl leading-relaxed border border-border font-mono">
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