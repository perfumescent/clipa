
<template>

    <a-input-search  placeholder="Please enter something"/>

    <a-radio-group v-model="size" type="button">
      <a-radio value="small">Text</a-radio>
      <a-radio value="medium">Image</a-radio>
      <a-radio value="large">File</a-radio>
    </a-radio-group>
    <a-list :size="size">
      <template #header>
        List title
      </template>
      <a-list-item>Beijing Bytedance Technology Co., Ltd.

        <template #extra>
          <div className="image-area">
            <img alt="arco-design" src="//p1-arco.byteimg.com/tos-cn-i-uwbnlip3yd/29c1f9d7d17c503c5d7bf4e538cb7c4f.png~tplv-uwbnlip3yd-webp.webp"/>
          </div>
        </template>
      </a-list-item>
      <a-list-item>Bytedance Technology Co., Ltd.</a-list-item>
      <a-list-item>Beijing Toutiao Technology Co., Ltd.</a-list-item>
      <a-list-item>Beijing Volcengine Technology Co., Ltd.</a-list-item>
      <a-list-item>China Beijing Bytedance Technology Co., Ltd.</a-list-item>
    </a-list>

</template>

<script setup lang="ts">
import { ref } from 'vue';
const size = ref('medium');
</script>
<style scoped>
.image-area {
  width: 183px;
  height: 119px;
  border-radius: 2px;
  overflow: hidden;
}


.image-area img {
  width: 100%;
}

</style>