<script setup lang="ts">
import {computed, nextTick, onMounted, ref, watch} from "vue";
import {getLearningWords, getReviewWords} from "../api/word.ts";
import {useToast} from "vue-toastification";
import {useAuthStore} from "../store/auth.ts";
import {LearningWord, Word} from "../model/word.ts";
import {ApiResult} from "../model/res.ts";
import {useLearningStore} from "../store/learning.ts";
import {
  mdiEmoticonConfusedOutline,
  mdiEmoticonCoolOutline,
  mdiEmoticonExcitedOutline,
  mdiFileWordBoxOutline,
  mdiGestureTap, mdiWaveform
} from "@mdi/js";
import {insertOrUpdateRecord} from "../api/learning_record.ts";
import {UploadRecord} from "../model/learning_record.ts";

const toast = useToast();
const authStore = useAuthStore();
const learningStore = useLearningStore();

const girlPictureList = ref(["休息.png", "办公室文员.png", "做作业.png", "困.png", "思考.png", "忙碌.png", "思考中.png", "打瞌睡.png", "正在加班.png", "认真工作.png", "遇到烦恼.png"]);
const fruitPictureList = ref(["pexels-masahiro-naruse-4335249.jpg", "pexels-masahiro-naruse-4335277.jpg", "pexels-masahiro-naruse-4335313.jpg", "pexels-masahiro-naruse-4425023.jpg", "pexels-masahiro-naruse-14898400.jpg"]);

const tab = ref();
const learningWords = ref<Array<LearningWord>>();
const audioRef = ref<HTMLAudioElement[]>();

const handlePlayAudio = () => {
  if (audioRef.value) {
    audioRef.value[0].src = audioUrl.value;
    audioRef.value[0].play();
  }
};

const handleKnowWord = (word: LearningWord, index: number) => {
  handlePlayAudio();
  const record = {
    userId: authStore.user!.userId,
    wordId: word.wordId,
    flag: true,
  } as UploadRecord;
  insertOrUpdateRecord(record);

  learningWords.value![index].showDefinition = true;
  learningStore.cursor = tab.value;
};

const handleNotKnowWord = (word: Word, index: number) => {
  handlePlayAudio();
  const record = {
    userId: authStore.user!.userId,
    wordId: word.wordId,
    flag: false,
  } as UploadRecord;
  insertOrUpdateRecord(record);

  learningWords.value![index].showDefinition = true;
  learningStore.cursor = tab.value;
  // toast.warning(`单词 ${word.word} 的意思是：${word.definition}`)
};

onMounted(() => {
  // 从store或api中获取当日的学习计划
  learningStore.checkAndUpdateWords();
  if (learningStore.words == null) {
    // 获取今日待复习的单词
    getReviewWords(authStore.user!.userId, authStore.user!.wordbookId, 60).then((reviewResult: ApiResult<Array<LearningWord>>) => {
      // 获取今日待学习的单词
      getLearningWords(authStore.user!.userId, authStore.user!.wordbookId, 30).then((learningResult: ApiResult<Array<LearningWord>>) => {
        if (learningResult.status == 200) {
          learningStore.words = [...reviewResult.data];
          learningStore.words.push(...learningResult.data);
          learningStore.date = new Date();
          learningStore.cursor = 0;
          learningWords.value = learningStore.words;

          nextTick(() =>{
            handlePlayAudio();
          })

          toast.info(`今日任务：待复习单词${reviewResult.data.length}个，另有新词${learningResult.data.length}个。`);
        }
      });
    });
  } else {
    learningWords.value = learningStore.words;
    tab.value = learningStore.cursor;
    nextTick(() =>{
      handlePlayAudio();
    })
  }

  // 播放当前单词音频
  handlePlayAudio();
});

// 当窗口改变时播放音频
watch(tab, () => {
  handlePlayAudio();
});

// 使用有道API获取当前单词的音频地址
const audioUrl = computed(() => {
  if (learningWords.value) {
    return `http://dict.youdao.com/dictvoice?audio=${learningWords.value![tab.value].word}`;
  } else {
    return "http://dict.youdao.com/dictvoice?audio=error";
  }
});
</script>

<template>
  <v-container>
    <v-window v-if="learningWords" v-model="tab" show-arrows>
      <v-card image="/image/background/material-design-colors-8k-2q.jpg" style="max-width: 688px;min-height: 493px">
        <v-tabs v-model="tab" color="indigo" align-tabs="start">
          <v-tab v-for="word in learningWords" :key="word.wordId" class="text-white">
            {{ word.word }}
          </v-tab>
        </v-tabs>
        <v-window-item v-for="(word, index) in learningWords" :key="word.wordId">
          <v-row justify="center" class="d-flex align-center justify-center mt-1 mb-1"
                 style="font-family: 'Times New Roman',宋体,serif">
            <v-col :cols="9">
              <v-card variant="elevated" color="primary" elevation="16">
                <v-card-title class="text-h3 text-center">
                  <v-icon :icon="mdiFileWordBoxOutline" :size="42"></v-icon>
                  {{ word.word }}
                </v-card-title>
                <v-card-item class="text-center font-weight-bold mb-1">
                  <audio ref="audioRef" :hidden="true"></audio>
                  <span>{{ word.phonogram }}</span>
                  <v-btn :icon="mdiWaveform" variant="flat" size="small" @click="handlePlayAudio"></v-btn>
                </v-card-item>
                <v-card-actions>
                  <v-row v-if="word.showDefinition" justify="center" class="mb-1">
                    <v-btn variant="outlined" :prepend-icon="mdiEmoticonExcitedOutline" class="bg-white mr-4"
                           size="large" rounded="xm" @click="tab += 1">下一个单词
                    </v-btn>
                  </v-row>
                  <v-row justify="center" class="mb-1" v-else>
                    <v-btn variant="outlined" :prepend-icon="mdiEmoticonConfusedOutline" class="bg-white mr-4"
                           size="large" rounded="xm" @click="handleNotKnowWord(word, index)">不认识
                    </v-btn>
                    <v-btn variant="outlined" :append-icon="mdiEmoticonCoolOutline" class="bg-white mr-4" size="large"
                           rounded="xm" @click="handleKnowWord(word, index)">认识
                    </v-btn>
                  </v-row>
                </v-card-actions>
              </v-card>
            </v-col>
          </v-row>
          <v-row justify="center" class="mt-1 mb-1">
            <v-col :cols="9">
              <v-card v-show="!word.showDefinition"
                      :image="'/image/' + fruitPictureList[word.wordId % fruitPictureList.length]"
                      height="220" variant="flat" color="orange" elevation="16">
                <v-card-title class="d-flex align-center justify-center mt-2">
                  <div class="text-white text-overline" style="font-size: x-large!important;">word definition</div>
                </v-card-title>
              </v-card>
              <v-scroll-x-transition>
                <v-card v-show="word.showDefinition" variant="flat" color="orange" elevation="16">
                  <v-row justify="start" class="mt-1 ml-1 mb-1 mr-1">
                    <v-col :cols="7">
                      <v-card-title class="text-h4 text-start text-yellow">{{ word.word }}</v-card-title>
                      <v-card-subtitle class="font-weight-bold text-white">{{ word.phonogram }}</v-card-subtitle>
                      <v-card-item class="text-h6 text-start text-white">{{ word.definition }}</v-card-item>
                      <v-card-actions>
                        <v-dialog width="500">
                          <template v-slot:activator="{ props }">
                            <v-btn v-bind="props" variant="outlined" color="orange" :prepend-icon="mdiGestureTap"
                                   class="bg-white mr-4" size="large" rounded="xm">查看例句
                            </v-btn>
                          </template>
                          <template v-slot:default="{ isActive }">
                            <v-card>
                              <v-card-title class="text-center text-h4 mt-4">{{ word.word + ' 例句' }}</v-card-title>
                              <v-card-text>
                                <v-list lines="one">
                                  <v-list-item
                                      v-for="[index, sentence] of word.exampleSentence.split('\n').entries()"
                                      :key="index"
                                  >
                                    <div v-if="index % 2 === 0">
                                      <v-list-item-title class="text-h6">
                                        Example {{ index / 2 + 1 }}
                                      </v-list-item-title>
                                      <div>{{ sentence }}</div>
                                      <div>{{ word.exampleSentence.split('\n')[index + 1] }}</div>
                                    </div>
                                    <v-divider v-else></v-divider>
                                  </v-list-item>
                                </v-list>
                              </v-card-text>
                              <v-card-actions>
                                <v-spacer></v-spacer>
                                <v-btn text="关闭" @click="isActive.value = false"></v-btn>
                              </v-card-actions>
                            </v-card>
                          </template>
                        </v-dialog>
                      </v-card-actions>
                    </v-col>
                    <v-col :cols="5" class="d-flex justify-center align-center">
                      <v-avatar
                          class="ma-3"
                          size="150"
                          rounded="0"
                      >
                        <v-img :src="'/image/' + girlPictureList[word.wordId % girlPictureList.length]"></v-img>
                      </v-avatar>
                    </v-col>
                  </v-row>
                </v-card>
              </v-scroll-x-transition>
            </v-col>
          </v-row>
        </v-window-item>
        <v-window-item v-if="learningWords">
          <v-row justify="center">
            <v-card class="mt-12" max-width="512">
              <v-img
                  class="align-end text-white"
                  height="192"
                  src="https://cdn.vuetifyjs.com/images/cards/sunshine.jpg"
                  cover
              >
                <v-card-title>今日打卡完成！</v-card-title>
              </v-img>
              <v-card-subtitle class="pt-4">
                Take time while time is,for time will away.
              </v-card-subtitle>
              <v-card-text>
                <div>
                  本轮共打卡 <strong>{{ learningWords.length }}</strong> 个单词。
                </div>
              </v-card-text>
              <v-card-actions>
                <v-btn color="orange" @click="$router.go(-1)">
                  backward
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