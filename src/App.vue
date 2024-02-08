<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
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
  <div class="container">
    <h1>Welcome to Tauri!</h1>

    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo"/>
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo"/>
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo"/>
      </a>
    </div>

    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <p>
      Recommended IDE setup:
      <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
      +
      <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
      +
      <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank"
      >Tauri</a
      >
      +
      <a href="https://github.com/rust-lang/rust-analyzer" target="_blank"
      >rust-analyzer</a
      >
    </p>

    <Greet/>
    {{ clipboardText }}
    <img :src="clipboardImage" alt="image">
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
