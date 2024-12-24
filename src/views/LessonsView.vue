<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { Lesson } from '../types/bindings';
import LessonService from '../services/lesson.service';

const lessons = ref<Lesson[]>([])

async function getLessons() {
    try {
        lessons.value = await LessonService.getLessons();
    } catch (error) {
        console.error(error);
    }
}

onMounted(async () => await getLessons())
</script>

<template>
    <div>

    </div>
</template>

<style scoped></style>