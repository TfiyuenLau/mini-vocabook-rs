<script setup lang="ts">
import {ref} from "vue";

const tab = ref(0);
const otp = ref("");

</script>

<template>
  <v-container>
    <v-window v-model="tab" show-arrows>
      <v-card image="/image/material.jpg" style="max-width: 688px;min-height: 493px">
        <v-tabs v-model="tab" color="indigo" align-tabs="start">
          <v-tab class="text-white">

          </v-tab>
        </v-tabs>
        <v-window-item>
          <v-row justify="center" class="mt-4">
            <v-col :cols="8" class="d-flex justify-center align-center">
              <v-card variant="elevated" color="indigo">
                <v-card-title>Language</v-card-title>
                <v-card-item>
                  <v-otp-input v-model="otp" variant="outlined"></v-otp-input>
                </v-card-item>
              </v-card>
            </v-col>
          </v-row>
        </v-window-item>
      </v-card>
    </v-window>
  </v-container>
</template>

<style scoped>

</style>