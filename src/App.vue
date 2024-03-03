<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {appWindow} from '@tauri-apps/api/window';

const wakeupChild = () => {
  document.dispatchEvent(new CustomEvent('init'));
}
async function wakeup() {
  const minimized =await appWindow.isMinimized();
  const visible = await appWindow.isVisible();
  const focused = await appWindow.isFocused();
  if( !visible || minimized  ){
    // 调用子组件List.vue的init函数
    wakeupChild();
    invoke("wakeup").then((res) => {
      console.log("wakeup", res);
    });
    await appWindow.show();
    await appWindow.unminimize();
    await appWindow.center();
    await appWindow.setFocus();
  }else if(!focused){
    await appWindow.hide();
    await wakeup();
  } else {
    await appWindow.hide();
  }
}


import {register, unregisterAll} from '@tauri-apps/api/globalShortcut';
import Header from "./components/Header.vue";
import {onMounted, onUnmounted} from "vue";

register('Command+`', wakeup);
register('control+space', wakeup);

onMounted(() => {
  console.log('onMounted');
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