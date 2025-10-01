import { ref, computed } from 'vue';

export default function useTimer() {
  const secondsElapsed = ref(0);
  const timerRunning = ref(false);
  let timerInterval: number | undefined;

  const formattedTime = computed(() => {
    const minutes = Math.floor(secondsElapsed.value / 60);
    const seconds = secondsElapsed.value % 60;
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  });

  const startTimer = () => {
    if (!timerRunning.value) {
      timerRunning.value = true;
      timerInterval = setInterval(() => {
        secondsElapsed.value++;
      }, 1000);
    }
  };

  const stopTimer = () => {
    if (timerRunning.value && timerInterval) {
      clearInterval(timerInterval);
      timerRunning.value = false;
    }
  };

  const resetTimer = () => {
    stopTimer();
    secondsElapsed.value = 0;
  };

  const cleanup = () => {
    if (timerInterval) {
      clearInterval(timerInterval);
    }
  };

  return {
    secondsElapsed: computed(() => secondsElapsed.value),
    formattedTime,
    timerRunning: computed(() => timerRunning.value),
    startTimer,
    stopTimer,
    resetTimer,
    cleanup
  };
}