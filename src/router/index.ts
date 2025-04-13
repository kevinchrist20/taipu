import { createRouter, createWebHistory } from "vue-router";

import HomeView from "../views/HomeView.vue";
import LessonAreaView from "../views/LessonAreaView.vue";
import CreateAccountView from "../views/CreateAccountView.vue";
import LessonsView from "../views/LessonsView.vue";
import { routes } from "../constants";
import { useLessonStore } from "../storage";

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: routes.home,
            name: "Welcome Page",
            component: HomeView
        },
        {
            path: routes.createAccount,
            name: 'Create Account',
            component: CreateAccountView
        },
        {
            path: routes.lessons,
            name: 'Lessons',
            component: LessonsView
        },
        {
            path: routes.lessonArea,
            name: 'Lesson Area',
            component: LessonAreaView,
            beforeEnter: (_, __, next) => {
                const lessonStore = useLessonStore();
                if (lessonStore.currentLesson) {
                    next();
                } else {
                    next({ path: routes.lessons });
                }
            }
        },
    ]
});

export default router;