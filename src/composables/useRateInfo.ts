import { computed, ComputedRef } from 'vue';

interface RateInfoParams {
  typedText: ComputedRef<string>;
  currentPosition: ComputedRef<number>;
  secondsElapsed: ComputedRef<number>;
  lessonContent: ComputedRef<string>;
}

export default function useRateInfo({ 
  typedText, 
  currentPosition, 
  secondsElapsed, 
  lessonContent 
}: RateInfoParams) {
  
  const percentComplete = computed(() => 
    Math.round((typedText.value.length / (lessonContent.value.length || 0)) * 100)
  );

  const accuracy = computed(() => {
    const correctChars = typedText.value.split('').filter((char, index) =>
      char === lessonContent.value[index]
    ).length;
    return Math.round((correctChars / (currentPosition.value > 0 ? currentPosition.value : 1)) * 100) || 100;
  });

  const wpm = computed(() => {
    // Standard WPM calculation assumes 5 characters (including spaces) = 1 word
    const charCount = typedText.value.length;
    const wordCount = charCount / 5;
    const minutes = secondsElapsed.value / 60;
    return minutes > 0 ? Math.round(wordCount / minutes) : 0;
  });

  const correctChars = computed(() =>
    typedText.value.split('').filter((char, index) =>
      char === lessonContent.value[index]
    ).length
  );

  const incorrectChars = computed(() =>
    currentPosition.value - correctChars.value
  );

  return {
    percentComplete,
    accuracy,
    wpm,
    correctChars,
    incorrectChars
  };
}