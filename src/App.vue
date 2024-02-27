<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import {invoke} from "@tauri-apps/api/tauri";
import {appWindow} from '@tauri-apps/api/window';

// appWindow.setDecorations(false);
async function wakeup() {
  if(await appWindow.isMinimized() || !await appWindow.isVisible() ){
    invoke("wakeup").then((res) => {
      console.log("wakeup", res);
    });
    await appWindow.show();
    appWindow.center();
    appWindow.setFocus();
  }else {
    appWindow.hide();
  }
}


import {register, unregisterAll} from '@tauri-apps/api/globalShortcut';
import Header from "./components/Header.vue";
import {onUnmounted} from "vue";

register('Command+Control+V', wakeup);
register('control+space', wakeup);
onUnmounted(() => {
  unregisterAll().then(() => {
    console.log('unregisterAll');
  });
});
</script>
<template>


  <a-layout class="container">
    <!-- 假设这里是Header或其他内容 -->

    <a-layout-header  class="header">
      <Header/>
    </a-layout-header>

    <!-- B组件填充剩余空间 -->

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