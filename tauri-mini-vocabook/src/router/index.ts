import {createRouter, createWebHashHistory} from 'vue-router'
import HomeView from "../views/HomeView.vue";
import QuizView from "../views/QuizView.vue";
import StudyView from "../views/LearningView.vue";
import StatisticsView from "../views/StatisticsView.vue";
import UserView from "../views/UserView.vue";

const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            path: '/',
            name: 'home',
            meta: {
                chinese: "主 页"
            },
            component: () => HomeView,
        },
        {
            path: '/quiz',
            name: 'quiz',
            meta: {
                chinese: "单词测试",
            },
            component: () => QuizView,
        },
        {
            path: '/learning',
            name: 'learning',
            meta: {
                chinese: "学习打卡",
            },
            component: () => StudyView,
        },
        {
            path: '/statistics',
            name: 'statistics',
            meta: {
                chinese: "学习统计",
            },
            component: () => StatisticsView,
        },
        {
            path: '/user',
            name: 'user',
            meta: {
                chinese: "个人用户",
            },
            component: () => UserView,
        },
    ]
});

export default router;