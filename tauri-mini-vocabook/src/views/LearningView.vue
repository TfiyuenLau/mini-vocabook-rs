<script setup lang="ts">
import {onMounted, ref, watch} from "vue";
import {getLearningWords} from "../api/word.ts";
import {useToast} from "vue-toastification";
import {useAuthStore} from "../store/auth.ts";
import {Word} from "../model/word.ts";
import {ApiResult} from "../model/res.ts";
import {useLearningStore} from "../store/learning.ts";
import {mdiEmoticonConfusedOutline, mdiEmoticonCoolOutline, mdiGestureTap} from "@mdi/js";
import {insertOrUpdateRecord} from "../api/learning_record.ts";
import {UploadRecord} from "../model/learning_record.ts";

const toast = useToast();
const authStore = useAuthStore();
const learningStore = useLearningStore();

const girlPictureList = ref(["休息.png", "办公室文员.png", "做作业.png", "困.png", "思考.png", "忙碌.png", "思考中.png", "打瞌睡.png", "正在加班.png", "认真工作.png", "遇到烦恼.png"]);
const fruitPictureList = ref(["pexels-masahiro-naruse-4335249.jpg", "pexels-masahiro-naruse-4335277.jpg", "pexels-masahiro-naruse-4335313.jpg", "pexels-masahiro-naruse-4425023.jpg", "pexels-masahiro-naruse-14898400.jpg"]);

const window = ref();
const showDefinition = ref(false);

// 当 window 属性变化时，重新显示幕布
watch(window, () => { // (newValue, oldValue)
  showDefinition.value = false;
});

const learningWords = ref<Array<Word>>();

const handleKnowWord = (wordId: number) => {
  const record = {
    userId: authStore.user?.userId,
    wordId: wordId,
    flag: true,
  } as UploadRecord;
  insertOrUpdateRecord(record);
  showDefinition.value = true;
  learningStore.cursor = window.value;
};

const handleNotKnowWord = (word: Word) => {
  const record = {
    userId: authStore.user?.userId,
    wordId: word.wordId,
    flag: false,
  } as UploadRecord;
  insertOrUpdateRecord(record);
  showDefinition.value = true;
  learningStore.cursor = window.value;

  // toast.warning(`单词 ${word.word} 的意思是：${word.definition}`)
};

onMounted(() => {
  // 从store或api中获取当日的学习计划
  learningStore.checkAndUpdateWords();
  if (learningStore.words == null) {
    getLearningWords(authStore.user!.userId, authStore.user!.wordbookId, 30).then((res: ApiResult<Array<Word>>) => {
      if (res.status == 200) {
        toast.success("今日单词任务获取成功");

        learningStore.words = [...res.data];
        learningStore.date = new Date();
        learningStore.cursor = 0;
        learningWords.value = learningStore.words;
      }
    });
  } else {
    learningWords.value = learningStore.words;
    window.value = learningStore.cursor;
  }
});

</script>

<template>
  <v-container>
    <v-window v-model="window" show-arrows>
      <v-card image="/image/material-design-colors-8k-2q.jpg">
        <div v-for="word in learningWords" :key="word.wordId">
          <v-window-item>
            <v-row justify="start" class="mt-4 mb-4" style="font-family: 'Times New Roman',宋体,serif">
              <v-col :cols="8" :offset="2">
                <v-card variant="elevated" color="primary" elevation="16">
                  <v-card-title class="text-h2 text-center mt-2">{{ word.word }}</v-card-title>
                  <v-card-item class="align-center justify-center font-weight-bold mb-2">
                    <span>{{ word.phonogram }}</span>
                  </v-card-item>
                  <v-card-actions>
                    <v-row justify="center" class="mb-2">
                      <v-btn variant="outlined" :prepend-icon="mdiEmoticonConfusedOutline" class="bg-white mr-4"
                             size="large" rounded="xm" @click="handleNotKnowWord(word)">不认识
                      </v-btn>
                      <v-btn variant="outlined" :append-icon="mdiEmoticonCoolOutline" class="bg-white mr-4" size="large"
                             rounded="xm" @click="handleKnowWord(word.wordId)">认识
                      </v-btn>
                    </v-row>
                  </v-card-actions>
                </v-card>
              </v-col>
            </v-row>
            <v-row justify="start" class="mt-4 mb-4">
              <v-col :cols="8" :offset="2">
                <v-card v-show="!showDefinition" :image="'/image/' + fruitPictureList[word.wordId % fruitPictureList.length]"
                        height="225" variant="flat" color="orange" elevation="16">
                  <v-card-title class="d-flex align-center justify-center mt-2">
                    <div class="text-white text-overline" style="font-size: x-large!important;">word definition</div>
                  </v-card-title>
                </v-card>
                <v-scroll-x-transition>
                  <v-card v-show="showDefinition" variant="flat" color="orange" elevation="16">
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
        </div>
      </v-card>
    </v-window>
  </v-container>
</template>

<style scoped>

</style>