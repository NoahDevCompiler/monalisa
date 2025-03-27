<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { info } from "@tauri-apps/plugin-log";
import VaultTab from "./Components/VaultTab.vue";

const appWindow = getCurrentWindow();

const closeWindow = () => {
  info("called");
  appWindow.close();
};

import { ref } from "vue";
import type { TabsInstance } from "element-plus";

const tabPosition = "left";
</script>

<template>
  <div class="bg-[#2d2d2d] overflow-hidden w-full h-screen">
    <div class="window-content h-full">
      <el-tabs
        :tab-position="tabPosition"

        class="settings-tabs h-full flex-shrink-0"
      >
        <el-tab-pane label="Vaults"><VaultTab/></el-tab-pane>
        <el-tab-pane label="Config">Config</el-tab-pane>
        <el-tab-pane label="Role">Role</el-tab-pane>
        <el-tab-pane label="Task">Task</el-tab-pane>
      </el-tabs>
    </div>
  </div>
</template>

<style>
body {
  background-color: transparent;
}

.settings-tabs > .el-tabs__content {
  padding: 32px;
  color: #a69888;
  font-size: 32px;
  font-weight: 600;
  width: ;
}
.settings-tabs .el-tabs__item.is-active {
  color: #a69888;
}
.settings-tabs .el-tabs__item:hover {
  color: #ffffff;
  transform: translateX(6px);

  transition-timing-function: cubic-bezier(
    0.33,
    1.5,
    0.68,
    1
  ); 
  transition-duration: 0.4s;
}
.settings-tabs.el-tabs--left .el-tabs__active-bar {
  background-color: white;
}
.settings-tabs.el-tabs--left .el-tabs__nav-wrap::after {
  background-color: #3a3a3a;

}
.settings-tabs.el-tabs--left .el-tabs__nav-wrap {
  height: 100px;
  max-height: 200px;
  background: #252525;
  border-radius: 8px;
  padding: 12px 0;
}

.settings-tabs .el-tabs__item {
  color: #a69888;
  transform: translateX(0);

  transition: transform 0.5s cubic-bezier(0.33, 1, 0.68, 1); 
  will-change: transform; 
  backface-visibility: hidden;
}

.window-content {
  padding: 20px;
  color: white;
  height: 100%;
  box-sizing: border-box;
}
</style>