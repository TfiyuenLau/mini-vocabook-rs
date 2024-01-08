<script setup lang="ts">
import {useRouter} from 'vue-router';
import {
  mdiHomeCity,
  mdiAccount,
  mdiLogoutVariant,
  mdiChartBar, mdiAlphaABox
} from '@mdi/js'
import {useAuthStore} from "../store/auth";
import {User} from "../model/user.ts";
import {onMounted, UnwrapRef} from "vue";

const router = useRouter();
const authStore = useAuthStore();

const userInfo: UnwrapRef<User | null> = authStore.user;

// 打开视图
const openView = (value: string) => {
  router.push({path: `/${value}`});
};

// 用户登出
const handleLogout = () => {
  router.push({path: `/`});
  authStore.logout();
  router.go(0); // 跳转后刷新
}

onMounted(() => {

})

</script>

<template>
  <v-navigation-drawer
      class="bg-deep-purple"
      expand-on-hover
      permanent
      rail
  >
    <v-list v-if="userInfo">
      <v-list-item
          prepend-avatar="/image/o_8cc2c399.png"
          :title="userInfo.username"
          :subtitle="userInfo.email"
      >
        <template v-slot:append>
          <v-btn :icon="mdiLogoutVariant" class="bg-white" variant="text" size="small" @click="handleLogout"></v-btn>
        </template>
      </v-list-item>
    </v-list>
    <v-list v-else>
      <v-list-item prepend-avatar="/image/akalin.jpg"></v-list-item>
    </v-list>

    <v-divider></v-divider>

    <v-list density="compact" nav>
      <v-list-item :prepend-icon="mdiHomeCity" title="主页" value="home" @click="openView('')"></v-list-item>
      <v-list-item :prepend-icon="mdiAlphaABox" title="单词测试" value="quiz" @click="openView('quiz')"></v-list-item>
      <v-list-item :prepend-icon="mdiChartBar" title="学习统计" value="statistics"
                   @click="openView('statistics')"></v-list-item>
      <v-list-item :prepend-icon="mdiAccount" title="用户设置" value="user" @click="openView('user')"></v-list-item>
    </v-list>
  </v-navigation-drawer>

</template>

<style scoped>

</style>