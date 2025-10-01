<script setup lang="ts">
import { CirclePause, LogOut, Play } from 'lucide-vue-next';
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
                    <CirclePause class="h-10 w-10 text-white" />
                </div>
                <h3 class="text-3xl font-bold text-white mb-2">Lesson Paused</h3>
                <p class="text-gray-300">Take your time. Resume when you're ready!</p>
            </div>

            <div class="flex gap-3">
                <button @click="emit('resume')"
                    class="w-full px-6 py-3 bg-green-600 hover:bg-green-700 text-white font-bold rounded-md transition transform hover:scale-105 flex items-center justify-center">
                    <Play class="mr-2" />
                    Resume
                </button>

                <button @click="emit('exit')"
                    class="w-full px-6 py-2 bg-gray-700 hover:bg-gray-600 text-white font-medium rounded-md transition transform hover:scale-105 flex items-center justify-center">
                    <LogOut class="mr-2" />
                    Exit
                </button>
            </div>

            <p class="text-xs text-gray-400 mt-4">Press ESC to resume</p>
        </div>
    </Modal>
</template>