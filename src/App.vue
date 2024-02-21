<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import List from "./components/List.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from '@tauri-apps/api/window';

async function wakeup() {
  invoke("wakeup").then((res) => {
    console.log("wakeup", res);
  });
  await appWindow.show();
  await appWindow.center();
  await appWindow.setFocus();
}


import {register} from '@tauri-apps/api/globalShortcut';
register('Command+Control+V', wakeup);
register('alt+space', wakeup);

</script>

<template>


    <List/>

</template>