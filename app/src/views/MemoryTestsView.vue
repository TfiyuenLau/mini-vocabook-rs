<script setup lang="ts">
import {mdiFileWord, mdiWaveform} from "@mdi/js";
import {computed, nextTick, onMounted, ref, watch} from "vue";
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
const audioRef = ref<HTMLAudioElement[]>();

// 控制播放音频
const handlePlayAudio = () => {
  if (audioRef.value) {
    audioRef.value[0].src = audioUrl.value;
    audioRef.value[0].play();
  }
};

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
      nextTick(() =>{
        handlePlayAudio();
      })
      toast.success(`当前测验共有${testsWordList.value!.length}个单词待解决`);
    }
  });
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

// 当窗口改变时播放音频
watch(tab, () => {
  handlePlayAudio();
});

// 使用有道API获取当前单词的音频地址
const audioUrl = computed(() => {
  if (testsWordList.value) {
    return `http://dict.youdao.com/dictvoice?audio=${testsWordList.value[tab.value].word}`;
  } else {
    return "http://dict.youdao.com/dictvoice?audio=error";
  }
});

// 计算用户的测验题数据
const rightCount = computed(() => {
  let count = 0;
  if (testsWordList.value) {
    for (let word of testsWordList.value) {
      if (word.isCorrect) {
        count++;
      }
    }
  }
  return count;
});

</script>

<template>
  <v-container>
    <v-window v-model="tab" show-arrows>
      <v-card image="/image/background/material.jpg" style="max-width: 688px;min-height: 493px">
        <v-tabs v-model="tab" color="indigo" align-tabs="start">
          <v-tab v-for="word in testsWordList" :key="word.wordId" class="text-white">
            {{ word.word }}
          </v-tab>
        </v-tabs>
        <v-window-item v-for="(word, wordIndex) in testsWordList" :key="word.wordId">
          <!-- 题目 -->
          <v-row justify="center" class="mt-1">
            <v-col :cols="8">
              <v-card variant="elevated" color="indigo">
                <v-card-title class="text-h3 text-center d-flex justify-center align-center">
                  <v-icon :icon="mdiFileWord" :size="48"/>
                  {{ word.word }}
                </v-card-title>
                <v-card-item class="d-flex align-center justify-center font-weight-bold">
                  <audio ref="audioRef" :hidden="true"></audio>
                  <span>{{ word.phonogram }}</span>
                  <v-btn :icon="mdiWaveform" variant="flat" color="indigo" size="x-small" @click="handlePlayAudio"></v-btn>
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
        <v-window-item v-if="testsWordList">
          <v-row justify="center">
            <v-card class="mt-12" max-width="512">
              <v-img
                  class="align-end text-white"
                  height="192"
                  src="https://cdn.vuetifyjs.com/images/cards/cooking.png"
                  cover
              >
                <v-card-title>单词记忆测验完成！</v-card-title>
              </v-img>
              <v-card-subtitle class="pt-4">
                Reading a book， by a gain in your wit.
              </v-card-subtitle>
              <v-card-text>
                <div>
                  本轮记忆测验共有 <strong>{{ testsWordList.length }}</strong> 个单词；
                </div>
                <div>
                  其中 <strong>{{ rightCount }}</strong> 个单词回答正确，
                  正确率为 <strong>{{ rightCount / testsWordList.length * 100 }}% </strong>！
                </div>
              </v-card-text>
              <v-card-actions>
                <v-btn color="orange" @click="$router.go(0)">
                  once again!
                </v-btn>
              </v-card-actions>
            </v-card>
          </v-row>
        </v-window-item>
      </v-card>
    </v-window>
  </v-container>
</template>

<style scoped>

</style>