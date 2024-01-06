import {createRouter, createWebHashHistory} from 'vue-router'
import HomeView from "../views/HomeView.vue";
import QuizView from "../views/QuizView.vue";

const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            path: '/',
            name: 'home',
            meta: {
                chinese: "主页"
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
    ]
});

export default router;