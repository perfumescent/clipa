<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import List from "./components/List.vue";
import {invoke} from "@tauri-apps/api/tauri";
import {appWindow} from '@tauri-apps/api/window';

// appWindow.setDecorations(false);
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








  <a-layout class="a-container" style="height: 100vh;">
    <!-- 假设这里是Header或其他内容 -->
    <a-layout-header>  <div :style="{
    backgroundImage: 'radial-gradient(var(--color-fill-3) 1px, rgba(0, 0, 0, 0) 1px)',
    backgroundSize: '16px 16px',
    padding: '35px 16px 16px 16px',
    userSelect: 'none', cursor: 'move'
  }" data-tauri-drag-region>
      <a-page-header title="Clipa" subtitle=" 2.0" style="cursor: default;padding: 0">
        <template #back-icon>
          <icon-close @click="appWindow.minimize()"/>
        </template>
        <template #extra>
          <a-checkbox-group :default-value="['1']">
            <a-checkbox value="1">
              <template #checkbox="{ checked }">
                <a-tag :checked="checked" checkable>
                  <icon-language/>
                </a-tag>
              </template>
            </a-checkbox>
            <a-checkbox value="2">
              <template #checkbox="{ checked }">
                <a-tag :checked="checked" checkable>
                  <icon-image/>
                </a-tag>
              </template>
            </a-checkbox>
          </a-checkbox-group>
        </template>
        <a-input-search placeholder="Please enter something" @search="query"/>
      </a-page-header>

    </div></a-layout-header>

    <!-- B组件填充剩余空间 -->

      <a-layout-content >
        <a-scrollbar type="embed">
      <List/>
        </a-scrollbar>
      </a-layout-content>


  </a-layout>
</template>
<style>
body {
  background-color: rgba(255, 255, 255, 0.95); /* 白色背景，50% 透明度 */
}
.b-container::-webkit-scrollbar {
  width: 0px; /* 滚动条宽度 */background-color: transparent; /* 背景色为透明 */
  border: none; /* 没有边框 */
}

.b-container::-webkit-scrollbar-track {
  background: #f1f1f1; /* 滚动条轨道颜色 */
}

.b-container::-webkit-scrollbar-thumb {
  background: #888; /* 滚动条滑块颜色 */
}

.b-container::-webkit-scrollbar-thumb:hover {
  background: #555; /* 滚动条滑块悬浮颜色 */
}

</style>