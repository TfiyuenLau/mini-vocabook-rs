<script setup lang="ts">
import {computed, nextTick, onMounted, ref, watch} from "vue";
import {SpellingQuiz, SpellingTestsWord} from "../model/word.ts";
import {getSpellingTestsWords} from "../api/word.ts";
import {useToast} from "vue-toastification";
import {useAuthStore} from "../store/auth.ts";
import {
  mdiCheck,
  mdiCloseThick,
  mdiLightbulbOnOutline,
  mdiMicrosoftWord, mdiWaveform,
} from "@mdi/js";
import {UploadRecord} from "../model/learning_record.ts";
import {insertOrUpdateRecord} from "../api/learning_record.ts";

const toast = useToast();
const authStore = useAuthStore();

const tab = ref(0);
const spellingTestsWords = ref<Array<SpellingTestsWord>>();
const audioRef = ref<HTMLAudioElement[]>();

const handlePlayAudio = () => {
  if (audioRef.value) {
    audioRef.value[0].src = audioUrl.value;
    audioRef.value[0].play();
  }
};

const handleSubmit = (word: SpellingTestsWord) => {
  if (word.input === word.quiz.middle) {
    // 拼写正确
    word.status = "right";
    const record = {
      userId: authStore.user!.userId,
      wordId: word.wordId,
      flag: true,
    } as UploadRecord;
    insertOrUpdateRecord(record);
    tab.value++;
  } else {
    // 拼写错误
    word.status = "error";
    word.input = "";
    toast.warning("拼写错误！复习后再尝试一下吧~");
  }
}

onMounted(() => {
  getSpellingTestsWords(authStore.user!.userId, 30).then(res => {
    if (res.status === 200) {
      spellingTestsWords.value = res.data;
      nextTick(() =>{
        handlePlayAudio();
      })
      for (let word of spellingTestsWords.value) {
        word.quiz = generateSpellingQuiz(word.word);
        word.input = "";
        word.status = "undefined";
      }

      toast.success(`当前共有${spellingTestsWords.value?.length}个单词待完成`);
    }
  });
});

// 当窗口改变时播放音频
watch(tab, () => {
  handlePlayAudio();
});

// 使用有道API获取当前单词的音频地址
const audioUrl = computed(() => {
  if (spellingTestsWords.value) {
    return `http://dict.youdao.com/dictvoice?audio=${spellingTestsWords.value[tab.value].word}`;
  } else {
    return "http://dict.youdao.com/dictvoice?audio=error";
  }
});

// 生成拼写测验的单词
const generateSpellingQuiz = (word: string): SpellingQuiz => {
  // 获取单词长度
  const wordLength = word.length;
  if (wordLength < 2) {
    throw new Error("单词长度不能小于2。");
  }

  // 随机生成中部截取的长度，保证最小为2，最大为单词长度
  const middleLength = Math.floor(Math.random() * (wordLength - 1)) + 2;
  const middleStart = Math.floor(Math.random() * (wordLength - middleLength + 1));

  // 截取首、中、尾部分
  const head = word.slice(0, middleStart);
  const middle = word.slice(middleStart, middleStart + middleLength);
  const tail = word.slice(middleStart + middleLength);

  return {head, middle, tail};
}

// 计算用户的测验题数据
const rightCount = computed(() => {
  let count = 0;
  if (spellingTestsWords.value) {
    for (let word of spellingTestsWords.value) {
      if (word.status === "right") {
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
          <v-tab class="text-white" v-for="word in spellingTestsWords" :key="word.wordId">
            ID-{{ word.wordId }}
          </v-tab>
        </v-tabs>
        <v-window-item v-for="word in spellingTestsWords" :key="word.wordId">
          <v-row justify="center" class="mt-8">
            <v-col :cols="10" class="d-flex justify-center align-center">
              <v-card variant="elevated" color="indigo">
                <v-card-title class="text-h5 text-center">
                  <v-icon :icon="mdiMicrosoftWord" :size="48"/>
                  {{ word.definition }}
                  <v-btn color="white" class="text-primary"
                         @click="toast.warning('tips:___' + word.quiz.middle + '___')">
                    <v-icon :icon="mdiLightbulbOnOutline" :size="24"/>
                    <v-tooltip activator="parent" location="end">拼写提示</v-tooltip>
                  </v-btn>
                </v-card-title>
                <v-card-item>
                  <v-row justify="center">
                    <v-col :cols="9">
                      <v-alert class="text-primary text-overline text-center">
                        <v-btn :icon="mdiWaveform" variant="flat" color="indigo" size="x-small" @click="handlePlayAudio"></v-btn>
                        {{ word.phonogram }}
                        <audio ref="audioRef" :hidden="true"></audio>
                      </v-alert>
                    </v-col>
                  </v-row>
                  <v-row justify="center" class="d-flex justify-center align-center ml-2 mr-2">
                    <div class="text-h4">
                      {{ word.quiz.head }}
                    </div>
                    <v-otp-input v-if="word.status === 'error'" v-model="word.input" :length="word.quiz.middle.length"
                                 type="text" variant="outlined" error></v-otp-input>
                    <v-otp-input v-else v-model="word.input" :length="word.quiz.middle.length"
                                 type="text" variant="outlined"></v-otp-input>
                    <div style="font-size: xxx-large;font-weight: bold;letter-spacing: .4rem;">
                      {{ word.quiz.tail }}
                    </div>
                  </v-row>
                </v-card-item>
                <v-card-actions>
                  <v-row justify="center" class="mt-2 mb-2">
                    <v-col :cols="6">
                      <v-btn @click="word.input = '';word.status = 'undefined';toast.info('清除成功')"
                             :prepend-icon="mdiCloseThick" variant="outlined" class="bg-white" style="width: 100%">
                        clear
                      </v-btn>
                    </v-col>
                    <v-col :cols="6">
                      <v-btn :prepend-icon="mdiCheck" @click="handleSubmit(word)" variant="outlined" class="bg-white"
                             style="width: 100%">
                        submit
                      </v-btn>
                    </v-col>
                  </v-row>
                </v-card-actions>
              </v-card>
            </v-col>
          </v-row>
        </v-window-item>
        <v-window-item v-if="spellingTestsWords">
          <v-row justify="center">
            <v-card class="mt-12" max-width="512">
              <v-img
                  class="align-end text-white"
                  height="192"
                  src="https://cdn.vuetifyjs.com/images/cards/docks.jpg"
                  cover
              >
                <v-card-title>单词拼写测验完成！</v-card-title>
              </v-img>
              <v-card-subtitle class="pt-4">
                Art is long, but life is short.
              </v-card-subtitle>
              <v-card-text>
                <div>
                  本轮拼写测验共有 <strong>{{ spellingTestsWords.length }}</strong> 个单词；
                </div>
                <div>
                  其中 <strong>{{ rightCount }}</strong> 个单词拼写对了，
                  正确率为 <strong>{{ rightCount / spellingTestsWords.length * 100 }}% </strong>！
                </div>
              </v-card-text>
              <v-card-actions>
                <v-btn color="orange" @click="$router.go(0)">
                  more tests!
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