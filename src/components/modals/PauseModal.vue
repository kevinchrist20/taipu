<script setup lang="ts">
import { CirclePause, LogOut, Play } from 'lucide-vue-next';
import Modal from './Modal.vue';
import AppButton from '../AppButton.vue';

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
                <div
                    class="w-16 h-16 border border-primary/25 rounded-2xl flex items-center justify-center mx-auto mb-4">
                    <CirclePause class="h-8 w-8 text-primary" />
                </div>
                <h3 class="text-2xl font-bold text-foreground mb-2 font-display">Lesson Paused</h3>
                <p class="text-muted-foreground text-sm">Take your time. Resume when you're ready!</p>
            </div>

            <div class="flex gap-3">
                <AppButton @click="emit('resume')" size="md" variant="primary"
                    class="w-full px-5 py-2.5 flex items-center justify-center gap-2">
                    <Play :size="16" />
                    Resume
                </AppButton>

                <AppButton @click="emit('exit')" size="md" variant="danger"
                    class="w-full px-5 py-2.5 flex items-center justify-center gap-2">
                    <LogOut :size="16" />
                    Exit
                </AppButton>
            </div>

            <p class="text-xs text-muted-foreground mt-4">Press ESC to resume</p>
        </div>
    </Modal>
</template>