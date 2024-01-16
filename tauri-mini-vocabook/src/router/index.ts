import {createRouter, createWebHashHistory} from 'vue-router'
import HomeView from "../views/HomeView.vue";
import QuizView from "../views/QuizView.vue";
import StudyView from "../views/LearningView.vue";
import StatisticsView from "../views/StatisticsView.vue";
import UserView from "../views/UserView.vue";
import MemoryTestsView from "../views/MemoryTestsView.vue";
import SpellingTestsView from "../views/SpellingTestsView.vue";

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
            component: () => HomeView,
        },
        {
            path: '/quiz',
            name: 'quiz',
            meta: {
                title: "单词测试",
                backward: false,
            },
            component: () => QuizView,
        },
        {
            path: '/memory_tests',
            name: 'memory_tests',
            meta: {
                title: "单词记忆练习",
                backward: true,
            },
            component: () => MemoryTestsView,
        },
        {
            path: '/spelling_tests',
            name: 'spelling_tests',
            meta: {
                title: "单词拼写练习",
                backward: true,
            },
            component: () => SpellingTestsView,
        },
        {
            path: '/learning',
            name: 'learning',
            meta: {
                title: "学习打卡",
                backward: true,
            },
            component: () => StudyView,
        },
        {
            path: '/statistics',
            name: 'statistics',
            meta: {
                title: "学习统计",
                backward: false,
            },
            component: () => StatisticsView,
        },
        {
            path: '/user',
            name: 'user',
            meta: {
                title: "个人用户",
                backward: false,
            },
            component: () => UserView,
        },
    ]
});

export default router;