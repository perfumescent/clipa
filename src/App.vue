<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {appWindow} from '@tauri-apps/api/window';
import List from "./components/List.vue";
let listVue:<InstanceType<typeof List> | null>;
async function wakeup() {
  if(await appWindow.isMinimized() || !await appWindow.isVisible() ){
    // 调用子组件List.vue的init函数
    listVue.value?.init();
    invoke("wakeup").then((res) => {
      console.log("wakeup", res);
    });
    await appWindow.show();
    await appWindow.unminimize();
    await appWindow.center();
    await appWindow.setFocus();
  }else {
    await appWindow.hide();
  }
}


import {register, unregisterAll} from '@tauri-apps/api/globalShortcut';
import Header from "./components/Header.vue";
import {onMounted, onUnmounted, ref} from "vue";

register('Command+Control+V', wakeup);
register('control+space', wakeup);

onMounted(() => {
  console.log('onMounted');
  listVue = ref<InstanceType<typeof List> | null>(null)
});
onUnmounted(() => {
  unregisterAll().then(() => {
    console.log('unregisterAll');
  });
});
</script>

<template>
  <a-layout class="container">
    <a-layout-header  class="header">
      <Header/>
    </a-layout-header>
    <a-layout-content class="content">
        <router-view></router-view>
    </a-layout-content>
  </a-layout>
</template>

<style>
.container {
  display: flex;
  flex-direction: column;
  height: 100vh; /* 使容器填充整个视口高度 */
}
.header {
  /* 确定头部的高度 */
  height: 100px;
}
.content {
  /* 这里不需要设置高度，它将自动填充剩余空间 */
  overflow-y: auto;
}
</style>