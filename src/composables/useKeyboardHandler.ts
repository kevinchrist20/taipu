import { onMounted, onUnmounted, ComputedRef } from 'vue';

interface KeyboardHandlerParams {
  onTogglePause: () => void;
  onResume: () => void;
  showPauseModal: ComputedRef<boolean>;
  showStatsModal: ComputedRef<boolean>;
  showExitModal: ComputedRef<boolean>;
  currentPosition: ComputedRef<number>;
}

export default function useKeyboardHandler({
  onTogglePause,
  onResume,
  showPauseModal,
  showStatsModal,
  showExitModal,
  currentPosition
}: KeyboardHandlerParams) {

  const handleGlobalKeyPress = (e: KeyboardEvent) => {
    if (e.key === 'Escape') {
      if (showPauseModal.value) {
        onResume();
      } else if (!showStatsModal.value && !showExitModal.value && currentPosition.value > 0) {
        onTogglePause();
      }
    }
  };

  onMounted(() => {
    window.addEventListener('keydown', handleGlobalKeyPress);
  });

  onUnmounted(() => {
    window.removeEventListener('keydown', handleGlobalKeyPress);
  });

  return {
    // No return values needed as this composable only handles events
  };
}