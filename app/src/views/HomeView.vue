<script setup lang="ts">
import {computed, onMounted, ref, watchEffect} from 'vue'
import {mdiArrowRightBoldCircleOutline, mdiBookOutline, mdiClose, mdiListStatus} from "@mdi/js";
import VueDatePicker from '@vuepic/vue-datepicker'
import '@vuepic/vue-datepicker/dist/main.css'
import {useToast} from 'vue-toastification';
import {useAuthStore} from "../store/auth.ts";
import {Word} from "../model/word.ts";
import {getWordById} from "../api/word.ts";
import {Wordbook} from "../model/wordbook.ts";
import {ApiResult} from "../model/res.ts";
import {getWordbookById, getWordbookProgress, getWordbookWordCount, getWordsByWordbookId} from "../api/wordbook.ts";
import LoginRegisterFrom from "../components/LoginRegisterFrom.vue";

const toast = useToast();
const authStore = useAuthStore();

const loginDialog = ref(false);
const timeDialog = ref(false);
const selectedDate = ref();
const wordlistDialog = ref(false);

const recommendWord = ref<Word>();
const wordbook = ref<Wordbook>();
const wordbookCount = ref()
const wordbookProgress = ref()
const wordlist = ref<Array<Word>>();
const listHeaders = ref([
  {title: '单词', key: 'word'},
  {title: '音标', key: 'phonogram'},
  {title: '释义', key: 'definition'},
]);

// 计算剩余天数
const daysDiff = computed(() => {
  let daysDiff = Math.abs(new Date(selectedDate.value).getTime() - new Date().getTime()) / (1000 * 60 * 60 * 24);
  return Math.floor(daysDiff);
});

const handleWordlist = () => {
  getWordsByWordbookId(authStore.user!.wordbookId, 0).then((res: ApiResult<Array<Word>>) => {
    if (res.status == 200) {
      wordlist.value = [...res.data];
      wordlistDialog.value = true;
    }
  });
}

// 通过监听store控制登录对话框显示
watchEffect(() => {
  loginDialog.value = !authStore.isLoggedIn;
});

onMounted(() => {
  // 获取推荐单词
  getWordById(Math.floor(Math.random() * 1000)).then(res => {
    recommendWord.value = res.data;
  })

  // 获取登录用户的目标单词本
  if (authStore.isLoggedIn && authStore.user) {
    getWordbookById(authStore.user.wordbookId).then((res: ApiResult<Wordbook>) => {
      if (res.status == 200) {
        wordbook.value = {...res.data};

        // 获取单词本学习进度信息
        getWordbookWordCount(wordbook.value?.wordbookId as number).then(res => {
          wordbookCount.value = res.data;
        })
        getWordbookProgress(authStore.user?.userId as number, wordbook.value?.wordbookId as number).then(res => {
          wordbookProgress.value = res.data;
        })
      }
    });
  }

  // 初始化目标日期
  selectedDate.value = authStore.targetDate;
});

// 存储目标日期
const setTargetDate = async (date: Date) => {
  authStore.targetDate = date;
  toast.success(`目标日期修改成功, 距离今日还剩${daysDiff.value}天。`);
  timeDialog.value = false;
};
</script>

<template>
  <v-container>
    <!-- Home主页面 -->
    <v-card image="/image/background/alm-friuli-snow-snowfall-45204.jpeg">
      <v-row justify="start" class="mt-4">
        <v-col :cols="5" offset="1">
          <v-card
              class="mx-auto"
              height="192"
              image="/image/pexels-ejov-igor-10456550.jpg"
              title="今日新词"
              theme="dark"
          >
            <div v-if="recommendWord">
              <v-row justify="center" class="mt-2" style="font-family: 'Times New Roman',宋体,serif">
                <div class="text-h3 text-white">{{ recommendWord.word }}</div>
              </v-row>
              <v-row justify="center" class="mt-2" style="font-family: 'Times New Roman',宋体,serif">
                <div class="text-center text-white ml-4 mr-4">{{ recommendWord.phonogram }}</div>
              </v-row>
              <v-row justify="center" class="mt-2" style="font-family: 'Times New Roman',宋体,serif">
                <div class="text-center text-white ml-4 mr-4">{{ recommendWord.definition }}</div>
              </v-row>
            </div>
          </v-card>
        </v-col>
        <v-col :cols="5">
          <v-card
              class="mx-auto"
              height="192"
              image="/image/pexels-leeloo-thefirst-5386829.jpg"
              title="Target"
              theme="dark"
          >
            <v-row class="mt-4" justify="center">
              <v-dialog
                  v-model="timeDialog"
                  :scrim="false"
                  transition="dialog-bottom-transition"
                  theme="dark"
                  fullscreen
              >
                <template v-slot:activator="{ props }">
                  <v-btn v-bind="props">距离下一目标还有</v-btn>
                </template>
                <v-card theme="light">
                  <v-toolbar color="primary" dark>
                    <v-btn @click="timeDialog = false" :icon="mdiClose" dark></v-btn>
                    <v-toolbar-title>目标日期选择</v-toolbar-title>
                    <v-spacer></v-spacer>
                    <v-toolbar-items>
                      <v-btn
                          variant="text"
                          @click="setTargetDate(selectedDate)"
                      >
                        Save
                      </v-btn>
                    </v-toolbar-items>
                  </v-toolbar>
                  <v-container>
                    <vue-date-picker v-model="selectedDate" :enable-time-picker="false"></vue-date-picker>
                    <v-divider></v-divider>
                    <v-list
                        lines="two"
                        subheader
                    >
                      <v-list-subheader>Tips</v-list-subheader>
                      <v-list-item title="全国大学英语四、六级考试"
                                   subtitle="大学四六级考试一般在每年的6月和12月举行，考试时长为140分钟和145分钟。 "></v-list-item>
                      <v-list-item title="全国硕士研究生统一招生考试"
                                   subtitle="研究生考试每年都在12月下旬举行；一般是12月的最后一个周六日进行初试考试。"></v-list-item>
                    </v-list>
                  </v-container>
                </v-card>
              </v-dialog>
            </v-row>
            <v-row justify="center">
              <p class="days">
                {{ daysDiff }}<span class="text-overline">days</span>
              </p>
            </v-row>
          </v-card>
        </v-col>
      </v-row>

      <v-row justify="start">
        <v-col :cols="10" :offset="1">
          <v-card class="mx-auto bg-deep-purple">
            <v-card-title>
              <v-row>
                <v-col :cols="10">{{ wordbook?.bookName }}</v-col>
                <v-col :cols="2">
                  <v-btn :prepend-icon="mdiListStatus" variant="outlined" rounded="lg" class="bg-white"
                         @click="handleWordlist">词表
                  </v-btn>
                </v-col>
              </v-row>
            </v-card-title>
            <v-card-item>
              <v-container>
                <v-row justify="center" class="mt-4">
                  <v-progress-linear :model-value="wordbookProgress / wordbookCount"></v-progress-linear>
                </v-row>
                <v-row justify="end" class="mt-4">

                </v-row>
              </v-container>
            </v-card-item>
            <v-card-actions>
              <v-row justify="end">
                <v-btn variant="outlined" class="bg-white mr-4">{{ wordbookProgress }} / {{ wordbookCount }}</v-btn>
              </v-row>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
      <v-row justify="center">
        <v-btn size="x-large" @click="$router.push({ path: '/learning' })">
          <template v-slot:prepend>
            <v-icon :icon="mdiArrowRightBoldCircleOutline"></v-icon>
          </template>
          开始打卡
        </v-btn>
      </v-row>
      <br>
    </v-card>

    <!-- 登录弹出框 -->
    <v-dialog
        v-model="loginDialog"
        width="100%"
        persistent
    >
      <login-register-from></login-register-from>
    </v-dialog>

    <!-- 词表弹出框 -->
    <v-dialog
        v-model="wordlistDialog"
        width="100%"
        scrollable
    >
      <v-card :prepend-icon="mdiBookOutline" :title="wordbook?.bookName" v-if="wordlist">
        <v-data-table :headers="listHeaders" :items="wordlist"></v-data-table>
      </v-card>
    </v-dialog>
  </v-container>
</template>

<style scoped>
.days {
  font-size: xxx-large;
  font-family: "Times New Roman", serif;
}
</style>