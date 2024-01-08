<script setup lang="ts">
import {useToast} from "vue-toastification";
import {useAuthStore} from "../store/auth";
import {
  mdiAccount,
  mdiAccountBox,
  mdiAirballoon,
  mdiBookVariant,
  mdiEmail,
  mdiLock,
  mdiLogin,
  mdiRegisteredTrademark,
  mdiStar
} from "@mdi/js";
import {onMounted, reactive, ref} from "vue";
import {User, UserRegister} from "../model/user.ts";
import {updateUser} from "../api/user.ts";
import {ApiResult} from "../model/res.ts";

const toast = useToast();
const authStore = useAuthStore();

const userUpdateDialog = ref(false);
const userUpdateForm = reactive<UserRegister>({
  email: authStore.user!.email,
  password: "",
  username: authStore.user!.username,
  wordbookId: authStore.user!.wordbookId,
});

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
      }
    });
  }
}

onMounted(() => {

});

</script>

<template>
  <v-container>
    <v-card image="/image/material-pattern-wallpaper.jpg">
      <v-row justify="start" class="mt-4 mb-2">
        <v-col :cols="10" offset="1">
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
                <v-card-actions class="d-flex justify-end">
                  <v-btn variant="outlined" color="indigo" rounded="xm" @click="userUpdateDialog = true;">修改密码</v-btn>
                  <v-btn variant="elevated" color="indigo" rounded="xm" @click="userUpdateDialog = true;">修改信息</v-btn>
                </v-card-actions>
              </v-col>
              <v-col :cols="4">
                <v-card class="d-flex justify-center align-center">
                  <v-avatar class="ma-3" size="128">
                    <v-img src="/image/o_8cc2c399.png"></v-img>
                  </v-avatar>
                </v-card>
              </v-col>
            </v-row>
            <v-divider></v-divider>
            <v-timeline direction="horizontal" class="mt-4 mb-2">
              <v-timeline-item dot-color="indigo" :icon="mdiStar">
                <template v-slot:opposite>
                  2024-01-06
                </template>
                <v-alert :model-value="true" color="info">
                  <div class="text-caption">打卡了 30 个单词</div>
                </v-alert>
              </v-timeline-item>
              <v-timeline-item dot-color="indigo" :icon="mdiBookVariant">
                <template v-slot:opposite>
                  2024-01-07
                </template>
                <v-alert :model-value="true" color="info">
                  <div class="text-caption">打卡了 120 个单词</div>
                </v-alert>
              </v-timeline-item>
              <v-timeline-item dot-color="indigo" :icon="mdiAirballoon">
                <template v-slot:opposite>
                  2024-01-08
                </template>
                <v-alert :model-value="true" color="info">
                  <div class="text-caption">打卡了 90 个单词</div>
                </v-alert>
              </v-timeline-item>
            </v-timeline>
          </v-card>
        </v-col>
      </v-row>
    </v-card>

    <!-- 注册弹出框 -->
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
                    <v-btn class="ml-2" color="orange" :prepend-icon="mdiRegisteredTrademark" @click="handleUpdateUser">
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

</style>