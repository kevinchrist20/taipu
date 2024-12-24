import { createRouter, createWebHistory } from "vue-router";

import HomeView from "../views/HomeView.vue";
import LessonAreaView from "../views/LessonAreaView.vue";
import CreateAccountView from "../views/CreateAccountView.vue";
import LessonsView from "../views/LessonsView.vue";

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: "/",
            name: "Welcome Page",
            component: HomeView
        },
        {
            path: '/create-account',
            name: 'Create Account',
            component: CreateAccountView
        },
        {
            path: '/lessons',
            name: 'Lessons',
            component: LessonsView
        },
        {
            path: '/lesson-area',
            name: 'Lesson Area',
            component: LessonAreaView
        },
    ]
});

export default router;