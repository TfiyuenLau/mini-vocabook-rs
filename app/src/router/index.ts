import {createRouter, createWebHashHistory} from 'vue-router'

const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            path: '/',
            name: 'home',
            meta: {
                title: "主 页",
                backward: false,
            },
            component: () => import("../views/HomeView.vue"),
        },
        {
            path: '/quiz',
            name: 'quiz',
            meta: {
                title: "单词测试",
                backward: false,
            },
            component: () => import("../views/QuizView.vue"),
        },
        {
            path: '/memory_tests',
            name: 'memory_tests',
            meta: {
                title: "单词记忆练习",
                backward: true,
            },
            component: () => import("../views/MemoryTestsView.vue"),
        },
        {
            path: '/spelling_tests',
            name: 'spelling_tests',
            meta: {
                title: "单词拼写练习",
                backward: true,
            },
            component: () => import("../views/SpellingTestsView.vue"),
        },
        {
            path: '/learning',
            name: 'learning',
            meta: {
                title: "学习打卡",
                backward: true,
            },
            component: () => import("../views/LearningView.vue"),
        },
        {
            path: '/statistics',
            name: 'statistics',
            meta: {
                title: "学习统计",
                backward: false,
            },
            component: () => import("../views/StatisticsView.vue"),
        },
        {
            path: '/user',
            name: 'user',
            meta: {
                title: "个人用户",
                backward: false,
            },
            component: () => import("../views/UserView.vue"),
        },
    ]
});

export default router;