<script setup lang="ts">

import {appWindow} from "@tauri-apps/api/window";
</script>

<template>
  <div :style="{
    height: '100%',
    backgroundImage: 'radial-gradient(var(--color-fill-3) 1px, rgba(0, 0, 0, 0) 1px)',
    backgroundSize: '16px 16px',
    padding: '35px 16px 16px 16px',
    userSelect: 'none', cursor: 'move'
  }" data-tauri-drag-region>
    <a-page-header title="Clipa" subtitle=" alpha" style="cursor: default;padding: 0">
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

    </a-page-header>

  </div>
</template>

<style scoped>

</style>