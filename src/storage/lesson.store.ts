import { defineStore } from "pinia";
import { Lesson } from "../types/bindings";

export const useLessonStore = defineStore('lesson', {
    state: () => ({
        currentLesson: null as Lesson | null,
    }),
    actions: {
        setLesson(lesson: Lesson) {
            this.currentLesson = lesson;
        },
        clearLesson() {
            this.currentLesson = null;
        },
    },
});