<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';

interface Props {
  show: boolean;
  size?: 'sm' | 'md' | 'lg';
  closeOnEscape?: boolean;
  closeOnBackdrop?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  size: 'md',
  closeOnEscape: true,
  closeOnBackdrop: false
});

const emit = defineEmits<{
  close: []
}>();

const sizeClasses = {
  sm: 'max-w-sm',
  md: 'max-w-md',
  lg: 'max-w-lg'
};

function handleEscape(e: KeyboardEvent) {
  if (e.key === 'Escape' && props.closeOnEscape && props.show) {
    emit('close');
  }
}

function handleBackdropClick() {
  if (props.closeOnBackdrop) {
    emit('close');
  }
}

onMounted(() => {
  if (props.closeOnEscape) {
    window.addEventListener('keydown', handleEscape);
  }
});

onUnmounted(() => {
  if (props.closeOnEscape) {
    window.removeEventListener('keydown', handleEscape);
  }
});
</script>

<template>
  <Transition name="modal">
    <div v-if="show" 
         class="fixed inset-0 flex items-center justify-center bg-gray-900 bg-opacity-80 z-20"
         @click="handleBackdropClick">
      <div class="bg-gray-800 rounded-lg shadow-2xl p-8 w-full mx-4 transform transition-all"
           :class="sizeClasses[size]"
           @click.stop>
        <slot />
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .bg-gray-800,
.modal-leave-active .bg-gray-800 {
  transition: transform 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

.modal-enter-from .bg-gray-800 {
  transform: scale(0.8);
}

.modal-leave-to .bg-gray-800 {
  transform: scale(0.8);
}
</style>