<script setup lang="ts">
import {RouterView, useRouter} from 'vue-router';
import DrawerNavigation from "./components/DrawerNavigation.vue";

const router = useRouter();
</script>

<template>
  <v-layout class="rounded rounded-md">
    <drawer-navigation></drawer-navigation>
    <v-app-bar :title="router.currentRoute.value.meta.chinese as string"></v-app-bar>
    <router-view class="ml-16 mt-16"></router-view>
  </v-layout>
</template>

<style scoped>

</style>
