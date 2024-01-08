<script setup lang="ts">

import {mdiMessageText} from "@mdi/js";
</script>

<template>
  <v-container>
    <v-card image="/image/material.jpg">
      <v-row justify="center" class="mt-8 mb-8">
        <v-alert
            class="ml-12 mr-12"
            color="primary"
            :icon="mdiMessageText"
            prominent
        >
          欢迎使用单词测验！本应用程序提供记忆练习、拼写练习和综合测验三个功能，帮助您提高英语单词能力。您可以根据自己的需要选择相应的功能进行练习。
        </v-alert>
      </v-row>
      <v-row justify="center" class="mt-16 mb-16 ml-8 mr-8">
        <v-col cols="4">
          <v-card
              class="bg-indigo text-white"
              image="/image/蓝紫色星空.png"
              height="184"
              title="记忆练习"
              hover
          >
            <v-card-item>

            </v-card-item>
          </v-card>
        </v-col>
        <v-col cols="4">
          <v-card
              class="bg-green text-white"
              image="/image/绿色星空.png"
              height="184"
              title="拼写练习"
              hover
          >
            <v-card-item>

            </v-card-item>
          </v-card>
        </v-col>
        <v-col cols="4">
          <v-card
              class="bg-orange text-white"
              image="/image/山峰星空.png"
              height="184"
              title="综合测验"
              hover
          >
            <v-card-item>

            </v-card-item>
          </v-card>
        </v-col>
      </v-row>
    </v-card>
  </v-container>
</template>

<style scoped>

</style>