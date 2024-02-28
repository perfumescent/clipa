<template>
  <a-empty v-if="data.length === 0" />
  <div v-else>
    <a-input ref="myInput" v-model="inputKeyword" :style="{width:'320px'}" placeholder="Please enter something" allow-clear />
    <a-table
      ref="myTable"
      :columns="columns"
      :data="data"
      :column-resizable="true"
      size="small"
      @cell-dblclick="clickCell"
      :show-header="false"
      :pagination="false"
      row-key="id"
    >
      <template #summary="{ record }">
        <div v-if="record.content_type === 'Image'">
          <a-image :src="record.summary" height="100" />
        </div>
        <div v-else class="ellipsis-text">
          {{ record.summary }}
        </div>
      </template>
      <template #op="{ record }">
        <a-button @click="select(record)" type="text">
          <template #icon>
            <icon-copy />
          </template>
        </a-button>
      </template>
    </a-table>
  </div>
</template>

<script setup lang="ts">
// 处理键盘事件的函数
import { onMounted, onUnmounted, ref, nextTick, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { TableData } from "@arco-design/web-vue";
import { appWindow } from "@tauri-apps/api/window";

const myTable = ref();
const myInput = ref();
const columns = [
  {
    title: "summary",
    dataIndex: "summary",
    slotName: "summary",
    ellipsis: true,
  },
  {
    title: "op",
    slotName: "op",
    width: 80,
  },
];

interface ClipboardItemDTO {
  id: string;
  content_type: string;
  summary: string;
  timestamp: number;
}

const data = ref<ClipboardItemDTO[]>([]);
const inputKeyword = ref("");
const init = async () => {
  inputKeyword.value = "";
  query();
}
defineExpose({
  init
})
function query(keyword?: string) {
  if (keyword === undefined) {
    keyword = "";
  }
  invoke("query_clipboard_items",{keyword:keyword}).then((res) => {
    console.log("query_clipboard_items", res);
    data.value = res as ClipboardItemDTO[];
    nextTick().then(() => {
      selectedRowIds.value = [data.value[0].id];
      myTable.value.select(selectedRowIds.value, true);
    });
  });
}

function select(record: ClipboardItemDTO) {
  appWindow.hide();
  invoke("paste", { clipboardItemId: record.id }).then((res) => {
    console.log("query_clipboard_items", res);
  });
}

function clickCell(record: TableData) {
  const tableData = record as ClipboardItemDTO;
  select(tableData);
}

// 用于存储当前选中行的id
const selectedRowIds: Ref<string[]> = ref([]);
// 当前选中行的索引
const currentRowIndex: Ref<number> = ref(0);

const handleKeyDown = async (event: KeyboardEvent) => {
  console.log(event);
  if (event.key === "Enter") {
    clickCell(data.value[currentRowIndex.value]);
  } else if (event.key === "ArrowDown" || event.key === "ArrowUp") {
    event.preventDefault(); // 防止页面滚动

    const direction = event.key === "ArrowDown" ? 1 : -1;
    let oldIndex = currentRowIndex.value;
    let newIndex = (currentRowIndex.value + direction) % data.value.length;
    newIndex = newIndex < 0 ? data.value.length - 1 : newIndex; // 修正负索引
    currentRowIndex.value = newIndex;

    const tableElement = document.querySelector("#app > section > main"); // 获取表格滚动容器
    const rows = myTable.value.$el.querySelectorAll(".arco-table-tr"); // 获取所有行元素
    const lastSelectedRowElement = rows[oldIndex]; // 通过索引访问
    const selectedRowElement = rows[newIndex]; // 通过索引访问
    if (selectedRowElement && tableElement) {
      lastSelectedRowElement.style.backgroundColor =
        selectedRowElement.style.backgroundColor;
      selectedRowElement.style.backgroundColor = "rgb(106,161,255)";
      console.log(lastSelectedRowElement.style.backgroundColor);
      console.log(selectedRowElement.style.backgroundColor);
      if (newIndex >= 0 && newIndex < rows.length) {
        // 计算选中行顶部距离容器顶部的距离
        let elementTop = selectedRowElement.offsetTop;

        // 计算应该滚动的距离：元素顶部距离 - 容器高度的一半 + 元素高度的一半
        let scrollTarget =
          elementTop -
          tableElement.clientHeight / 2 +
          selectedRowElement.clientHeight / 2;

        // 使用Math.max确保不会滚动到负值
        tableElement.scrollTop = Math.max(0, scrollTarget);
      }
    }
    myTable.value.select(selectedRowIds.value, false);
    selectedRowIds.value = [data.value[newIndex].id];
    myTable.value.select(selectedRowIds.value, true);
  } else {
    // 如果有其他动作，则是选中输入框a-input，让输入框获取焦点
    myInput.value.focus();
  }
};

// 在组件挂载时添加键盘事件监听器，在卸载时移除
onMounted(() => {
  window.addEventListener("keydown", handleKeyDown);
  query();
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeyDown);
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
