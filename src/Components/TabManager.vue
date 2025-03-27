<script setup lang="ts">
import { ref } from 'vue'
import type { TabPaneName } from 'element-plus'
</script>
<template>
    <el-tabs
      v-model="editableTabsValue"
      type="card"
      class="demo-tabs"
      editable
      @edit="handleTabsEdit"
    >
      <template #add-icon>
        <el-icon><Select /></el-icon>
      </template>
      <el-tab-pane
        v-for="item in editableTabs"
        :key="item.name"
        :label="item.title"
        :name="item.name"
      >
        {{ item.content }}
      </el-tab-pane>
    </el-tabs>
  </template>
  