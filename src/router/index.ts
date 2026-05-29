import { createRouter, createWebHistory } from "vue-router";

import HomeView from "../views/HomeView.vue";
import LessonAreaView from "../views/LessonAreaView.vue";
import CreateAccountView from "../views/CreateAccountView.vue";
import LessonsView from "../views/LessonsView.vue";
import CategoryLessonsView from "../views/CategoryLessonsView.vue";
import { routes } from "../constants";
import { SessionStore } from "../storage";

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
            path: routes.categoryLessons,
            name: 'Category Lessons',
            component: CategoryLessonsView
        },
        {
            path: routes.lessonArea,
            name: 'Lesson Area',
            component: LessonAreaView
        },
        {
            path: routes.stats,
            name: 'Stats',
            component: () => import("../views/StatsView.vue")
        },
        {
            path: routes.settings,
            name: 'Settings',
            component: () => import("../views/SettingsView.vue")
        }
    ]
});

router.beforeEach((to) => {
    const protectedPaths = [routes.lessons, routes.stats, routes.settings];
    const requiresAuth = protectedPaths.some(path => to.path === path || to.path.startsWith(`${path}/`))
        || to.path.startsWith('/lesson/');

    if (requiresAuth && !SessionStore.user) {
        return { path: routes.home };
    }

    return true;
});

export default router;