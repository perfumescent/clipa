<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import List from "./components/List.vue";
import {ref} from "vue";
import {listen} from '@tauri-apps/api/event'
import {Event} from "@tauri-apps/api/helpers/event";

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


</script>

<template>
  <div>
<a-layout>
      <a-layout-header>Header</a-layout-header>
      <a-layout-content>
    <List/>
    {{ clipboardText }}
    <img :src="clipboardImage" alt="image">
      </a-layout-content>
      <a-layout-footer>Footer</a-layout-footer>
    </a-layout>
    
  </div>
</template>

