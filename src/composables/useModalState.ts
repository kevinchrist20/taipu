import { ref, computed } from 'vue';

export default function useModalState() {
  const showStatsModal = ref(false);
  const showPauseModal = ref(false);
  const showExitModal = ref(false);

  const hasActiveModal = computed(() => 
    showStatsModal.value || showPauseModal.value || showExitModal.value
  );

  const openStatsModal = () => {
    showStatsModal.value = true;
  };

  const closeStatsModal = () => {
    showStatsModal.value = false;
  };

  const openPauseModal = () => {
    showPauseModal.value = true;
  };

  const closePauseModal = () => {
    showPauseModal.value = false;
  };

  const openExitModal = () => {
    showExitModal.value = true;
    if (showPauseModal.value) {
      showPauseModal.value = false;
    }
  };

  const closeExitModal = () => {
    showExitModal.value = false;
  };

  const closeAllModals = () => {
    showStatsModal.value = false;
    showPauseModal.value = false;
    showExitModal.value = false;
  };

  return {
    showStatsModal: computed(() => showStatsModal.value),
    showPauseModal: computed(() => showPauseModal.value),
    showExitModal: computed(() => showExitModal.value),
    hasActiveModal,
    openStatsModal,
    closeStatsModal,
    openPauseModal,
    closePauseModal,
    openExitModal,
    closeExitModal,
    closeAllModals
  };
}