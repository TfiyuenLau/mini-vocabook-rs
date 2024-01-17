import { defineStore } from 'pinia';
import {User} from "../model/user.ts";

// 用户存储
export const useAuthStore = defineStore("auth", {
    state: () => ({
        user: null as User | null,
        isLoggedIn: false,
        targetDate: new Date(),
    }),
    actions: {
        login(userInfo: User) {
            this.user = {...userInfo};
            this.isLoggedIn = true;
        },
        logout() {
            this.user = null;
            this.isLoggedIn = false;
        },
    },
    persist: true,
});
