
<template>

    <a-input-search  placeholder="Please enter something" @search="query"/>

  <a-empty v-if="data.length===0"/>
  <a-table v-else :columns="columns" :data="data" :column-resizable="true" size="mini">

      <template #summary="{ record }" >
        <div v-if="record.content_type === 'Image'">
          <a-image :src="record.summary" height="100"/>
        </div>
        <div v-else class="ellipsis-text">
          {{ record.summary }}
        </div>

      </template>
    <template #op="{ record }" >
      <a-button @click="$modal.info({ title:'Name', content:record.name })">view</a-button>

    </template>
  </a-table>


</template>

<script setup lang="ts">
import { ref} from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

const columns = [
  {
    title: 'summary',
    dataIndex: 'summary',slotName: 'summary',ellipsis: true
  },
  {
    title: 'op',slotName: 'op',width: 80
  }
];
interface ClipboardItemDTO {
  id: string;
  content_type: string;
  summary: string;
  timestamp: number;
}
const data = ref<ClipboardItemDTO[]>([]);
function query() {
  invoke("query_clipboard_items").then((res ) => {
    console.log("query_clipboard_items", res);
    data.value = res as ClipboardItemDTO[];
  });
}
</script>
<style scoped>
.ellipsis-text {
  display: block; /* 或者 inline-block */
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  width: 100%; /* 使得元素宽度适应父容器 */
}

</style>