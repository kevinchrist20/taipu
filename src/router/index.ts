import { createRouter, createWebHistory } from "vue-router";

import HomeView from "../views/HomeView.vue";
import LessonAreaView from "../views/LessonAreaView.vue";

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: "/",
            name: "Welcome Page",
            component: HomeView
        },
        {
            path: '/lesson-area',
            name: 'Lesson Area',
            component: LessonAreaView
        }
    ]
});

export default router;