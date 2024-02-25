<template>


  <a-empty v-if="data.length===0"/>
  <div v-else>
    <!--      <a-input-search placeholder="Please enter something" @search="query"/>-->
    <a-table ref="myTable" :columns="columns" :data="data" :column-resizable="true" size="small"
             @cell-dblclick="clickCell" :show-header="false"
             :pagination="false" row-key="id">
      <!--        :pagination="{size:'mini',hideOnSinglePage:true,simple:true}"-->
      <template #summary="{ record }">
        <div v-if="record.content_type === 'Image'">
          <a-image :src="record.summary" height="100"/>
        </div>
        <div v-else class="ellipsis-text">
          {{ record.summary }}
        </div>

      </template>
      <template #op="{ record }">
        <a-button @click="select(record)" type="text">
          <template #icon>
            <icon-copy/>
          </template>
        </a-button>
      </template>
    </a-table>

  </div>

</template>

<script setup lang="ts">
import {onMounted, onUnmounted, Ref, ref} from 'vue';
import {invoke} from "@tauri-apps/api/tauri";
import {TableData} from "@arco-design/web-vue";
import {appWindow} from '@tauri-apps/api/window';

const columns = [
  {
    title: 'summary',
    dataIndex: 'summary', slotName: 'summary', ellipsis: true
  },
  {
    title: 'op', slotName: 'op', width: 80
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
  invoke("query_clipboard_items").then((res) => {
    console.log("query_clipboard_items", res);
    data.value = res as ClipboardItemDTO[];
  });
}

function select(record: ClipboardItemDTO) {
  appWindow.hide();
  invoke("paste", {clipboardItemId: record.id}).then((res) => {
    console.log("query_clipboard_items", res);
    data.value = res as ClipboardItemDTO[];
  });
}

function clickCell(record: TableData) {
  const tableData = record as ClipboardItemDTO;
  select(tableData);
}

query();


// 用于存储当前选中行的id
const selectedRowKeys: Ref<string[]> = ref([]);
// 当前选中行的索引
const currentRowIndex: Ref<number> = ref(-1);

// 更新选中行的处理函数
const handleRowSelectionChange = (newSelectedRowKeys: string[]) => {
  selectedRowKeys.value = newSelectedRowKeys;
};
const myTable = ref();
// 处理键盘事件的函数
const handleKeyDown = (event: KeyboardEvent) => {
  console.log(event);
  if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
    event.preventDefault(); // 防止页面滚动

    const direction = event.key === 'ArrowDown' ? 1 : -1;
    // 计算新的选中行索引，确保它在有效范围内
    let newIndex = (currentRowIndex.value + direction) % data.value.length;
    newIndex = newIndex < 0 ? 0 : newIndex;
    // 更新当前选中行的索引和键值
    currentRowIndex.value = newIndex;
    myTable.value.select(selectedRowKeys.value, false);
    selectedRowKeys.value = [data.value[newIndex].id];
    myTable.value.select(selectedRowKeys.value, true);
    console.log(data.value[newIndex]);
  }
};

// 在组件挂载时添加键盘事件监听器，在卸载时移除
onMounted(() => {
  window.addEventListener('keydown', handleKeyDown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
});
</script>
<style scoped>
.ellipsis-text {
  display: block; /* 或者 inline-block */
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  width: 100%; /* 使得元素宽度适应父容器 */
  user-select: none;
  font-size: 12px;
}

</style>