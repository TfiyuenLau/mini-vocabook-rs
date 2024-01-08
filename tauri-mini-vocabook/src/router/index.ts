import {createRouter, createWebHashHistory} from 'vue-router'
import HomeView from "../views/HomeView.vue";
import QuizView from "../views/QuizView.vue";
import StudyView from "../views/LearningView.vue";

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
    ]
});

export default router;