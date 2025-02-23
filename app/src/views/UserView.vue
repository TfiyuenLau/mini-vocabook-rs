<script setup lang="ts">
import {useToast} from "vue-toastification";
import {useAuthStore} from "../store/auth";
import {
  mdiAccount,
  mdiAccountBox,
  mdiAirballoon,
  mdiBookVariant,
  mdiEmail,
  mdiLock, mdiLockCheck,
  mdiLogin,
  mdiStar, mdiUpdate
} from "@mdi/js";
import {onMounted, reactive, ref} from "vue";
import {User, UserRegister} from "../model/user.ts";
import {updateUser, updateUserPassword} from "../api/user.ts";
import {ApiResult} from "../model/res.ts";
import {getDateCheckInStatistic} from "../api/learning_record.ts";
import {useRouter} from "vue-router";

const toast = useToast();
const router = useRouter();
const authStore = useAuthStore();

const checkInDateList = ref<Array<any>>();
const userUpdateDialog = ref(false);
const passwordUpdateDialog = ref(false);
const passwordUpdateForm = reactive({
  email: authStore.user!.email,
  password: "",
  modifyPw: "",
});
const userUpdateForm = reactive<UserRegister>({
  email: authStore.user!.email,
  password: "",
  username: authStore.user!.username,
  wordbookId: authStore.user!.wordbookId,
});
const avatarList = ["像素小男孩.png", "o_8cc2c399.png", "像素插画学生.png", "像素绅士.png", "像素美女.png", "像素老师.png", "像素职场女性.png", "像素长发女孩.png", "项目儿童.png"]

// 更新用户密码
const handleUpdateUserPassword = () => {
  if (passwordUpdateForm.password === "") {
    toast.error("请在修改密码前输入旧密码以校验安全性");
  } else if (passwordUpdateForm.modifyPw === ""){
    toast.error("请输入新密码");
  } else {
    updateUserPassword(passwordUpdateForm.email, passwordUpdateForm.password, passwordUpdateForm.modifyPw).then((res: ApiResult<User>) => {
      if (res.status == 200) {
        passwordUpdateDialog.value = false;

        // 用户登出
        router.push({path: `/`}).then(_ => {
          authStore.logout();
          router.go(0);

          toast.success("账户密码更新成功");
        });
      } else {
        toast.error(`用户密码修改失败，请检查旧密码的准确性`);
      }
    });
  }
}

// 更新用户信息
const handleUpdateUser = () => {
  if (userUpdateForm.password === "") {
    toast.error("请在修改信息前输入密码以校验账号");
  } else {
    updateUser(userUpdateForm).then((res: ApiResult<User>) => {
      if (res.status == 200) {
        toast.success("基本信息更新成功");
        userUpdateDialog.value = false;

        authStore.user = res.data;
      } else {
        toast.error(`用户信息修改失败，请检查密码准确性`);
      }
    });
  }
}

onMounted(() => {
  getDateCheckInStatistic(authStore.user!.userId, 3).then(res => {
    if (res.status == 200) {
      checkInDateList.value = res.data;
    }
  })
});

</script>

<template>
  <v-container>
    <v-card image="/image/background/material-pattern-wallpaper.jpg">
      <v-row justify="center" class="mt-4 mb-2">
        <v-col :cols="10">
          <v-card variant="elevated" title="我的账号" :prepend-icon="mdiAccountBox">
            <v-row justify="start">
              <v-col :cols="7">
                <v-card-item>
                  <div class="text-h5 mb-1">
                    {{ authStore.user?.email }}
                  </div>
                  <div class="text-overline mb-1">
                    {{ authStore.user?.username }}
                  </div>
                  <div class="text-caption">Hello, Welcome to Mini Vocabook client.</div>
                </v-card-item>
                <v-card-actions class="d-flex justify-end align-center">
                  <v-btn variant="outlined" color="indigo" rounded="xm" @click="passwordUpdateDialog = true;">修改密码</v-btn>
                  <v-btn variant="elevated" color="indigo" rounded="xm" @click="userUpdateDialog = true;">编辑信息</v-btn>
                </v-card-actions>
              </v-col>
              <v-col :cols="4">
                <v-card class="d-flex justify-center align-center">
                  <v-avatar class="ma-3" size="128">
                    <v-img :src="'/image/avatar/' + avatarList[authStore.user!.userId % avatarList.length]"></v-img>
                  </v-avatar>
                </v-card>
              </v-col>
            </v-row>
            <v-divider></v-divider>
            <v-timeline v-if="checkInDateList && checkInDateList.length > 0" direction="horizontal" class="mt-4 mb-2">
              <v-timeline-item dot-color="indigo" :icon="mdiStar" v-if="checkInDateList.length >= 3">
                <template v-slot:opposite>
                  {{ checkInDateList[2].study_date }}
                </template>
                <v-alert :model-value="true" color="info">
                  <div class="text-caption">打卡了 {{ checkInDateList[2].study_count }} 个单词</div>
                </v-alert>
              </v-timeline-item>
              <v-timeline-item dot-color="indigo" :icon="mdiBookVariant" v-if="checkInDateList.length >= 2">
                <template v-slot:opposite>
                  {{ checkInDateList[1].study_date }}
                </template>
                <v-alert :model-value="true" color="info">
                  <div class="text-caption">打卡了 {{ checkInDateList[1].study_count }} 个单词</div>
                </v-alert>
              </v-timeline-item>
              <v-timeline-item dot-color="indigo" :icon="mdiAirballoon" v-if="checkInDateList.length >= 1">
                <template v-slot:opposite>
                  {{ checkInDateList[0].study_date }}
                </template>
                <v-alert :model-value="true" color="info">
                  <div class="text-caption">打卡了 {{ checkInDateList[0].study_count }} 个单词</div>
                </v-alert>
              </v-timeline-item>
            </v-timeline>
            <v-row v-else justify="center" class="mt-8 mb-8">
              <v-col :cols="10">
                <div class="text-h4 text-primary text-center">暂未找到学习记录哦，从今天起开始学习吧~</div>
              </v-col>
            </v-row>
          </v-card>
        </v-col>
      </v-row>
    </v-card>

    <!-- 用户密码修改弹出框 -->
    <v-dialog v-model="passwordUpdateDialog" width="95%">
      <v-container fill-height fluid>
        <v-row align="center" justify="center">
          <v-col cols="12" sm="8" md="4">
            <v-card image="">
              <v-card-title class="headline text-center mt-4">用户密码修改</v-card-title>

              <v-card-text>
                <v-form class="ml-4 mr-4">
                  <v-text-field
                      v-model="passwordUpdateForm.email"
                      label="Email"
                      :prepend-icon="mdiEmail"
                      outlined
                      disabled
                  >
                  </v-text-field>
                  <v-text-field
                      v-model="passwordUpdateForm.password"
                      label="Password"
                      type="password"
                      placeholder="请输入密码以校验账号安全性"
                      :prepend-icon="mdiLockCheck"
                      outlined
                  >
                  </v-text-field>
                  <v-text-field
                      v-model="passwordUpdateForm.modifyPw"
                      label="Modify Password"
                      type="password"
                      placeholder="请输入新密码"
                      :prepend-icon="mdiLock"
                      outlined
                  >
                  </v-text-field>

                  <v-row justify="end" class="mt-4 mb-4">
                    <v-btn variant="outlined" class="mr-2" color="indigo" :prepend-icon="mdiLogin"
                           @click="passwordUpdateDialog = false;">
                      close
                    </v-btn>
                    <v-btn class="ml-2" color="orange" :prepend-icon="mdiUpdate" @click="handleUpdateUserPassword">
                      Update
                    </v-btn>
                  </v-row>
                </v-form>
              </v-card-text>
            </v-card>
          </v-col>
        </v-row>
      </v-container>
    </v-dialog>

    <!-- 用户信息修改弹出框 -->
    <v-dialog v-model="userUpdateDialog" width="85%">
      <v-container fill-height fluid>
        <v-row align="center" justify="center">
          <v-col cols="12" sm="8" md="4">
            <v-card image="">
              <v-card-title class="headline text-center mt-4">用户信息修改</v-card-title>

              <v-card-text>
                <v-form class="ml-4 mr-4">
                  <v-text-field
                      v-model="userUpdateForm.email"
                      label="Email"
                      :prepend-icon="mdiEmail"
                      outlined
                      disabled
                  >
                  </v-text-field>
                  <v-text-field
                      v-model="userUpdateForm.username"
                      label="Username"
                      type="Username"
                      :prepend-icon="mdiAccount"
                      outlined
                  >
                  </v-text-field>
                  <v-text-field
                      v-model="userUpdateForm.password"
                      label="Password"
                      type="password"
                      placeholder="请输入密码以校验账号安全性"
                      :prepend-icon="mdiLock"
                      outlined
                  >
                  </v-text-field>
                  <v-radio-group v-model="userUpdateForm.wordbookId" inline>
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
                           @click="userUpdateDialog = false;">
                      close
                    </v-btn>
                    <v-btn class="ml-2" color="orange" :prepend-icon="mdiUpdate" @click="handleUpdateUser">
                      Update
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

</style>