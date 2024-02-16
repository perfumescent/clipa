<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import List from "./components/List.vue";
import {ref} from "vue";
import {listen} from '@tauri-apps/api/event'
import {Event} from "@tauri-apps/api/helpers/event";
import { invoke } from "@tauri-apps/api/tauri";
const clipboardText = ref('');
const clipboardImage = ref('');

interface ClipboardDTO {
  data_type: "text" | "image"; // 使用联合类型限制data_type的值
  content: string; // 文本数据或 Base64 编码的图片数据
}

listen('clipboard-changed', (event:Event<ClipboardDTO>) => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
  console.log(event.event,event.id,event.windowLabel,event.payload);
  const dto = event.payload;
  if (dto.data_type === 'text') {
    // 处理文本数据
    clipboardText.value = dto.content;
  } else if (dto.data_type === 'image') {
    // 处理图片数据
    clipboardImage.value = dto.content;
  }
})
function snapshot_on_current_window() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  invoke("snapshot_on_current_window").then((window_application_id) => {
    console.log("snapshot_on_current_window",window_application_id);
    appWindow.show();
    appWindow.setFocus();
    // timeout to wait for the window to be shown

    invoke("paste_on_window_snapshot",{id:window_application_id});

  });
}
import { appWindow } from '@tauri-apps/api/window';

import {isRegistered, register} from '@tauri-apps/api/globalShortcut';
register('Command+Control+V', snapshot_on_current_window);
isRegistered('Command+Control+V').then((registered) => {
  console.log('Command+Control+V is registered:', registered);
});
</script>

<template>


    <List/>

</template>

