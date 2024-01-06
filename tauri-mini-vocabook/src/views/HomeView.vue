<script setup lang="ts">
import {computed, onMounted, reactive, ref, watchEffect} from 'vue'
import {
  mdiAccount,
  mdiArrowRightBoldCircleOutline,
  mdiClose,
  mdiEmail,
  mdiLock,
  mdiLogin,
  mdiRegisteredTrademark
} from "@mdi/js";
import VueDatePicker from '@vuepic/vue-datepicker'
import '@vuepic/vue-datepicker/dist/main.css'
import {Store} from "tauri-plugin-store-api";
import {UserLogin, UserRegister} from "../model/user.ts";
import {loginUser, registerUser} from "../api/user.ts";
import {useToast} from 'vue-toastification';
import {useAuthStore} from "../store/auth.ts";
import {useRouter} from "vue-router";

const toast = useToast();
const router = useRouter();
const authStore = useAuthStore();
const store = new Store(".settings.dat");

const loginDialog = ref(false);
const registerDialog = ref(false);
const timeDialog = ref(false);
const selectedDate = ref();

const loginFrom = reactive<UserLogin>({
  email: "", password: "",
});

const registerFrom = reactive<UserRegister>({
  email: "", password: "", username: "", wordbook_id: 1
});

// 计算剩余天数
const daysDiff = computed(() => {
  let daysDiff = Math.abs(new Date(selectedDate.value).getTime() - new Date().getTime()) / (1000 * 60 * 60 * 24);
  return Math.floor(daysDiff);
});

const handleLogin = () => {
  loginUser(loginFrom).then(res => {
    if (res.status == 200) {
      toast.success(`登陆成功，${res.data.username} 欢迎！`);

      authStore.login(res.data);
      router.go(0);
    } else {
      toast.error("登陆失败，账号或密码错误");
    }
  });
};

const handleRegister = () => {
  registerUser(registerFrom).then(res => {
    if (res.status == 200) {
      toast.success("账户注册成功");
      registerDialog.value = false;
    } else {
      toast.success("注册失败，请填写完整后重试");
    }
  });
};

// 通过监听store控制登录对话框显示
watchEffect(() => {
  loginDialog.value = !authStore.isLoggedIn;
});

onMounted(() => {
  // 获取初始化的目标日期
  store.get("targetDate").then((res: any) => {
    if (res == null) {
      setTargetDate(new Date())
    } else {
      selectedDate.value = res;
    }
  });
});

// 存储目标日期
const setTargetDate = async (date: Date) => {
  await store.set("targetDate", date);
  await store.save();
  toast.success(`目标日期修改成功, 距离今日还剩${daysDiff.value}天。`);
  timeDialog.value = false;
};
</script>

<template>
  <v-container>
    <!-- Home主页面 -->
    <v-card image="https://images.pexels.com/photos/45204/alm-friuli-snow-snowfall-45204.jpeg">
      <v-row justify="start" class="mt-4">
        <v-col :cols="5" offset="1">
          <v-card
              class="mx-auto"
              height="192"
              image="https://picsum.photos/500/300?image=232"
              title="今日单词"
              theme="dark"
          >
            <v-row justify="center" class="mt-2" style="font-family: 'Times New Roman',宋体,serif">
              <div class="text-h3 text-white">Language</div>
              <div class="text-center text-white ml-4 mr-4">
                n. 语言;言语;语言文字;说话;计算机语言;表达方式，交际方式;
              </div>
            </v-row>
          </v-card>
        </v-col>
        <v-col :cols="5">
          <v-card
              class="mx-auto"
              height="192"
              image="https://cdn.vuetifyjs.com/images/cards/road.jpg"
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
          <v-card class="mx-auto bg-deep-purple" :title="'四六级考试词汇本（精简）'">
            <v-card-item>
              <v-container>
                <v-row justify="center" class="mt-4">
                  <v-progress-linear :model-value="42">
                  </v-progress-linear>
                </v-row>
                <v-row justify="end" class="mt-4">

                </v-row>
              </v-container>
            </v-card-item>
            <v-card-actions>
              <v-row justify="end">
                <v-btn variant="outlined" class="bg-white mr-4">455 / 2254</v-btn>
              </v-row>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
      <v-row justify="center">
        <v-btn size="x-large">
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
      <v-container fill-height fluid>
        <v-row align="center" justify="center">
          <v-col cols="12" sm="8" md="4">
            <v-card image="https://images.pexels.com/photos/925743/pexels-photo-925743.jpeg">
              <v-card-title class="headline text-center mt-4">用户登录</v-card-title>

              <v-card-text>
                <v-form class="ml-4 mr-4">
                  <v-text-field
                      v-model="loginFrom.email"
                      label="Email"
                      :prepend-icon="mdiEmail"
                      outlined
                  >
                  </v-text-field>
                  <v-text-field
                      v-model="loginFrom.password"
                      label="Password"
                      type="password"
                      :prepend-icon="mdiLock"
                      outlined
                  >
                  </v-text-field>

                  <v-btn variant="outlined" class="mt-4" color="orange" :prepend-icon="mdiRegisteredTrademark"
                         @click="loginDialog = false;registerDialog = true;" block>
                    Register
                  </v-btn>
                  <v-btn class="mt-4" color="indigo" :prepend-icon="mdiLogin" @click="handleLogin" block>
                    Login
                  </v-btn>
                </v-form>
              </v-card-text>
            </v-card>
          </v-col>
        </v-row>
      </v-container>
    </v-dialog>

    <!-- 注册弹出框 -->
    <v-dialog
        v-model="registerDialog"
        width="90%"
        persistent
    >
      <v-container fill-height fluid>
        <v-row align="center" justify="center">
          <v-col cols="12" sm="8" md="4">
            <v-card image="">
              <v-card-title class="headline text-center mt-4">用户注册</v-card-title>

              <v-card-text>
                <v-form class="ml-4 mr-4">
                  <v-text-field
                      v-model="registerFrom.email"
                      label="Email"
                      :prepend-icon="mdiEmail"
                      outlined
                  >
                  </v-text-field>
                  <v-text-field
                      v-model="registerFrom.username"
                      label="Username"
                      type="Username"
                      :prepend-icon="mdiAccount"
                      outlined
                  >
                  </v-text-field>
                  <v-text-field
                      v-model="registerFrom.password"
                      label="Password"
                      type="password"
                      :prepend-icon="mdiLock"
                      outlined
                  >
                  </v-text-field>
                  <v-radio-group v-model="registerFrom.wordbook_id" inline>
                    <v-row justify="start">
                      <v-col :cols="4">
                        <div class="text-center text-subtitle-2">四级核心词汇本</div>
                        <v-card image="/image/满足.png" color="yellow" height="110" hover>
                          <v-radio label="" color="red" :value="1"></v-radio>
                        </v-card>
                      </v-col>
                      <v-col :cols="4">
                        <div class="text-center text-subtitle-2">六级核心词汇本</div>
                        <v-card image="/image/开心.png" color="orange" height="110" hover>
                          <v-radio label="" color="pink" :value="2"></v-radio>
                        </v-card>
                      </v-col>
                      <v-col :cols="4">
                        <div class="text-center text-subtitle-2">考研精选词汇本</div>
                        <v-card image="/image/欢快.png" color="green" height="110" hover>
                          <v-radio label="" color="yellow" :value="3"></v-radio>
                        </v-card>
                      </v-col>
                    </v-row>
                  </v-radio-group>

                  <v-row justify="end" class="mt-4 mb-4">
                    <v-btn variant="outlined" class="mr-2" color="indigo" :prepend-icon="mdiLogin"
                           @click="registerDialog = false;loginDialog = true;">
                      close
                    </v-btn>
                    <v-btn class="ml-2" color="orange" :prepend-icon="mdiRegisteredTrademark" @click="handleRegister">
                      Register
                    </v-btn>
                  </v-row>
                </v-form>
              </v-card-text>
            </v-card>
          </v-col>
        </v-row>
      </v-container>
    </v-dialog>
  </v-container>
</template>

<style scoped>
.days {
  font-size: xxx-large;
  font-family: "Times New Roman", serif;
}
</style>