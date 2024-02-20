<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import List from "./components/List.vue";
import { invoke } from "@tauri-apps/api/tauri";



function wakeup() {
  invoke("wakeup").then((res) => {
    console.log("wakeup", res);
  });
  
  // sleep 1 second
  setTimeout(() => {
    invoke("paste", ).then((res) => {
      console.log("wakeup", res);
    });
  }, 1000);
}


import {register} from '@tauri-apps/api/globalShortcut';
register('Command+Control+V', wakeup);

</script>

<template>


    <List/>

</template>

