<script setup lang="ts">
import Modal from './Modal.vue';

interface Props {
    show: boolean;
    hasProgress: boolean;
}

defineProps<Props>();

const emit = defineEmits<{
    confirm: [];
    cancel: [];
}>();
</script>

<template>
    <Modal :show="show" size="md" :close-on-escape="true" @close="emit('cancel')">
        <div class="text-center">
            <div class="mb-6">
                <div class="w-20 h-20 bg-red-600 rounded-full flex items-center justify-center mx-auto mb-4">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10 text-white" viewBox="0 0 20 20"
                        fill="currentColor">
                        <path fill-rule="evenodd"
                            d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                            clip-rule="evenodd" />
                    </svg>
                </div>
                <h3 class="text-2xl font-bold text-white mb-2">Exit Lesson?</h3>
                <p class="text-gray-300" v-if="hasProgress">
                    Your progress will be lost if you exit now.
                </p>
                <p class="text-gray-300" v-else>
                    Are you sure you want to exit?
                </p>
            </div>

            <div class="flex gap-3">
                <button @click="emit('cancel')"
                    class="flex-1 px-6 py-3 bg-gray-700 hover:bg-gray-600 text-white font-bold rounded-md transition transform hover:scale-105">
                    Cancel
                </button>
                <button @click="emit('confirm')"
                    class="flex-1 px-6 py-3 bg-red-600 hover:bg-red-700 text-white font-bold rounded-md transition transform hover:scale-105">
                    Exit
                </button>
            </div>
        </div>
    </Modal>
</template>