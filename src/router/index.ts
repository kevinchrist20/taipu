import { createRouter, createWebHistory } from "vue-router";

import HomeView from "../views/HomeView.vue";
import LessonAreaView from "../views/LessonAreaView.vue";
import CreateAccountView from "../views/CreateAccountView.vue";

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
        },
        {
            path: '/create-account',
            name: 'Create Account',
            component: CreateAccountView
        }
    ]
});

export default router;