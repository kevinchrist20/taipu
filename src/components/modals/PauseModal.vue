<script setup lang="ts">
import Modal from './Modal.vue';

interface Props {
    show: boolean;
}

defineProps<Props>();

const emit = defineEmits<{
    resume: [];
    exit: [];
}>();
</script>

<template>
    <Modal :show="show" size="md" :close-on-escape="false">
        <div class="text-center">
            <div class="mb-6">
                <div class="w-20 h-20 bg-blue-600 rounded-full flex items-center justify-center mx-auto mb-4">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10 text-white" viewBox="0 0 20 20"
                        fill="currentColor">
                        <path fill-rule="evenodd"
                            d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zM7 8a1 1 0 012 0v4a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v4a1 1 0 102 0V8a1 1 0 00-1-1z"
                            clip-rule="evenodd" />
                    </svg>
                </div>
                <h3 class="text-3xl font-bold text-white mb-2">Lesson Paused</h3>
                <p class="text-gray-300">Take your time. Resume when you're ready!</p>
            </div>

            <div class="space-y-3">
                <button @click="emit('resume')"
                    class="w-full px-6 py-3 bg-green-600 hover:bg-green-700 text-white font-bold rounded-md transition transform hover:scale-105 flex items-center justify-center">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20"
                        fill="currentColor">
                        <path fill-rule="evenodd"
                            d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z"
                            clip-rule="evenodd" />
                    </svg>
                    Resume Lesson
                </button>

                <button @click="emit('exit')"
                    class="w-full px-6 py-2 bg-gray-700 hover:bg-gray-600 text-white font-medium rounded-md transition transform hover:scale-105 flex items-center justify-center">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20"
                        fill="currentColor">
                        <path fill-rule="evenodd"
                            d="M3 3a1 1 0 00-1 1v12a1 1 0 102 0V4a1 1 0 00-1-1zm10.293 9.293a1 1 0 001.414 1.414l3-3a1 1 0 000-1.414l-3-3a1 1 0 10-1.414 1.414L14.586 9H7a1 1 0 100 2h7.586l-1.293 1.293z"
                            clip-rule="evenodd" />
                    </svg>
                    Exit Lesson
                </button>
            </div>

            <p class="text-xs text-gray-400 mt-4">Press ESC to resume</p>
        </div>
    </Modal>
</template>