<script setup lang="ts">
import {mdiAccount, mdiEmail, mdiLock, mdiLogin, mdiRegisteredTrademark} from "@mdi/js";
import {reactive, ref} from "vue";
import {User, UserLogin, UserRegister} from "../model/user";
import {loginUser, registerUser} from "../api/user";
import {useToast} from "vue-toastification";
import {useRouter} from "vue-router";
import {useAuthStore} from "../store/auth";
import {ApiResult} from "../model/res";

const toast = useToast();
const router = useRouter();
const authStore = useAuthStore();

const registerDialog = ref(false);

const loginForm = reactive<UserLogin>({
  email: "", password: "",
});

const registerForm = reactive<UserRegister>({
  email: "", password: "", username: "", wordbookId: 1
});

const handleLogin = () => {
  loginUser(loginForm).then((res: ApiResult<User>) => {
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
  registerUser(registerForm).then(res => {
    if (res.status == 200) {
      toast.success("账户注册成功");
      registerDialog.value = false;
    } else {
      toast.error("注册失败，请填写完整后重试");
    }
  });
};

</script>

<template>
  <v-container fill-height fluid>
    <v-row align="center" justify="center">
      <v-col cols="12" sm="8" md="4">
        <v-card image="/image/pexels-joão-jesus-925743.jpg">
          <v-card-title class="headline text-center mt-4">用户登录</v-card-title>

          <v-card-text>
            <v-form class="ml-4 mr-4">
              <v-text-field
                  v-model="loginForm.email"
                  label="Email"
                  :prepend-icon="mdiEmail"
                  outlined
              >
              </v-text-field>
              <v-text-field
                  v-model="loginForm.password"
                  label="Password"
                  type="password"
                  :prepend-icon="mdiLock"
                  outlined
              >
              </v-text-field>

              <v-btn variant="outlined" class="mt-4" color="orange" :prepend-icon="mdiRegisteredTrademark"
                     @click="registerDialog = true;" block>
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
                      v-model="registerForm.email"
                      label="Email"
                      :prepend-icon="mdiEmail"
                      outlined
                  >
                  </v-text-field>
                  <v-text-field
                      v-model="registerForm.username"
                      label="Username"
                      type="Username"
                      :prepend-icon="mdiAccount"
                      outlined
                  >
                  </v-text-field>
                  <v-text-field
                      v-model="registerForm.password"
                      label="Password"
                      type="password"
                      :prepend-icon="mdiLock"
                      outlined
                  >
                  </v-text-field>
                  <v-radio-group v-model="registerForm.wordbookId" inline>
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
                           @click="registerDialog = false;">
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

</style>