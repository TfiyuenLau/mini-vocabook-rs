<script setup lang="ts">
import {mdiFileWord} from "@mdi/js";
import {onMounted, ref} from "vue";
import {getMemoryTestsWords} from "../api/word.ts";
import {useAuthStore} from "../store/auth.ts";
import {MemoryTestsWord} from "../model/word.ts";
import {UploadRecord} from "../model/learning_record.ts";
import {insertOrUpdateRecord} from "../api/learning_record.ts";
import {useToast} from "vue-toastification";

const toast = useToast();
const authStore = useAuthStore();

const tab = ref(0);
const testsWordList = ref<Array<MemoryTestsWord>>();

// 单词释义选择正确
const handleRight = (word: MemoryTestsWord, wordIndex: number, optionIndex: number) => {
  // 修改单词属性
  testsWordList.value![wordIndex].selected = [optionIndex];
  testsWordList.value![wordIndex].isCorrect = true;

  // 更新熟练度
  const record = {
    userId: authStore.user!.userId,
    wordId: word.wordId,
    flag: true,
  } as UploadRecord;
  insertOrUpdateRecord(record);
};

// 单词释义选择错误
const handleWorry = (word: MemoryTestsWord, wordIndex: number, optionIndex: number, correctIndex: number) => {
  // 修改单词属性
  testsWordList.value![wordIndex].selected = [optionIndex, correctIndex];
  testsWordList.value![wordIndex].isCorrect = false;

  // 更新熟练度
  const record = {
    userId: authStore.user!.userId,
    wordId: word.wordId,
    flag: false,
  } as UploadRecord;
  insertOrUpdateRecord(record);

  toast.warning(`${word.word}：${word.definition}`);
};

onMounted(() => {
  getMemoryTestsWords(authStore.user!.userId, 30).then(res => {
    if (res.status == 200) {
      testsWordList.value = res.data;
    }
  })
})

const getOptionCardColor = (word: MemoryTestsWord, index: number) => {
  if (word.selected && word.selected.indexOf(index) !== -1) {
    if (word.isCorrect || word.selected.indexOf(index) !== 0) {
      return "success";
    } else {
      return "warning";
    }
  }
  return "indigo";
}

const getOptionTitleClass = (word: MemoryTestsWord, index: number) => {
  if (word.selected && word.selected.indexOf(index) !== -1) {
    return {
      "text-success": word.isCorrect || word.selected.indexOf(index) !== 0,
      "text-warning": !word.isCorrect && word.selected.indexOf(index) === 0,
      "text-center": true,
      "ml-2": true,
      "mr-2": true,
    };
  }
  return ["text-indigo", "text-center", "ml-2", "mr-2"];
}
</script>

<template>
  <v-container>
    <v-window v-model="tab" show-arrows>
      <v-card image="/image/material.jpg" style="max-width: 688px;min-height: 493px">
        <v-tabs v-model="tab" color="indigo" align-tabs="start">
          <v-tab v-for="word in testsWordList" :value="word.wordId" :key="word.wordId" class="text-white">
            {{ word.word }}
          </v-tab>
        </v-tabs>
        <v-window-item v-for="(word, wordIndex) in testsWordList" :key="word.wordId" :value="word.wordId">
          <!-- 题目 -->
          <v-row justify="center" class="mt-2">
            <v-col :cols="8">
              <v-card variant="elevated" color="indigo">
                <v-card-title class="text-h3 text-center d-flex justify-center align-center">
                  <v-icon :icon="mdiFileWord" :size="48"/>
                  {{ word.word }}
                </v-card-title>
                <v-card-item class="align-center justify-center font-weight-bold">
                  <span>{{ word.phonogram }}</span>
                </v-card-item>
              </v-card>
            </v-col>
          </v-row>
          <!-- 选项 -->
          <v-row justify="center" v-for="optionIndex in 4">
            <v-col :cols="7">
              <!-- 正确答案 -->
              <v-card
                  v-if="(optionIndex + word.wordId) % 4 == 0"
                  @click="!word.selected ? handleRight(word, wordIndex, optionIndex) : null"
                  :color="getOptionCardColor(word, optionIndex)"
                  variant="outlined"
                  class="bg-white"
                  hover
              >
                <v-card-title :class="getOptionTitleClass(word, optionIndex)">
                  {{ word.definition }}
                </v-card-title>
              </v-card>
              <!-- 错误答案 -->
              <v-card v-else :color="getOptionCardColor(word, optionIndex)"
                      @click="!word.selected ? handleWorry(word, wordIndex, optionIndex, 4 - (word.wordId % 4)) : null"
                      variant="outlined" class="bg-white" hover>
                <v-card-title :class="getOptionTitleClass(word, optionIndex)">
                  {{ word.option[(optionIndex + word.wordId) % 4 - 1] }}
                </v-card-title>
              </v-card>
            </v-col>
          </v-row>
          <br>
        </v-window-item>
      </v-card>
    </v-window>
  </v-container>
</template>

<style scoped>

</style>