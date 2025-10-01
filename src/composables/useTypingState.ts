import { ref, computed } from 'vue';

export default function useTypingState() {
  const currentPosition = ref(0);
  const typedText = ref('');

  const handleKeyInput = (key: string, lessonContent: string) => {
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

    return currentPosition.value >= lessonContent.length;
  };

  const resetTyping = () => {
    currentPosition.value = 0;
    typedText.value = '';
  };

  const nextKey = computed(() => (lessonContent: string) => {
    return lessonContent[currentPosition.value];
  });

  const hasProgress = computed(() => currentPosition.value > 0);

  return {
    currentPosition: computed(() => currentPosition.value),
    typedText: computed(() => typedText.value),
    nextKey,
    hasProgress,
    handleKeyInput,
    resetTyping
  };
}