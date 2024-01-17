import { defineStore } from 'pinia';
import {LearningWord} from "../model/word";

// 今日学习任务存储
export const useLearningStore = defineStore("learning", {
    state: () => ({
        words: null as Array<LearningWord> | null,
        date: null as Date | null,
        cursor: 0,
    }),
    actions: {
        // 检查日期并重置words
        checkAndUpdateWords() {
            const today = new Date();
            if (this.date && new Date(this.date).toDateString() !== today.toDateString()) {
                this.words = null;
                this.date = null;
                this.cursor = 0;
            }
        },
    },
    persist: true,
});
