<script setup lang="ts">
import {RouterView, useRouter} from 'vue-router';
import DrawerNavigation from "./components/DrawerNavigation.vue";
import {mdiClose} from "@mdi/js";

const router = useRouter();

//屏蔽右键菜单
document.oncontextmenu = function (event: any) {
  if (window.event) {
    event = window.event
  }
  try {
    var the = event.srcElement
    if (
        !(
            (the.tagName == 'INPUT' && the.type.toLowerCase() == 'text') ||
            the.tagName == 'TEXTAREA'
        )
    ) {
      return false
    }
    return true
  } catch (e) {
    return false
  }
}
</script>

<template>
  <v-layout class="rounded rounded-md">
    <drawer-navigation></drawer-navigation>
    <v-app-bar scroll-behavior="elevate">
      <v-app-bar-title>{{ router.currentRoute.value.meta.title }}</v-app-bar-title>
      <template v-slot:append>
        <v-btn v-show="router.currentRoute.value.meta.backward" :icon="mdiClose" @click="router.back()"></v-btn>
      </template>
    </v-app-bar>
    <router-view class="ml-16 mt-16"></router-view>
  </v-layout>
</template>

<style scoped>

</style>
