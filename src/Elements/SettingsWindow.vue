<script setup lang="ts">

import { getCurrentWindow } from "@tauri-apps/api/window";
import { info } from "@tauri-apps/plugin-log";

const appWindow = getCurrentWindow();

const closeWindow = () => {
  info("called")
  appWindow.close();
};

</script>

<template>
  <div class="rounded-window">
    <div class="window-content">
      <h1>Settings</h1>
      <button class="bg-white size-8" @click="closeWindow"></button>
    </div>
  </div>
</template>

<style scoped>
body {
  background-color: transparent;
}
.rounded-window {
  width: 100%;
  height: 100vh;
  background: #2d2d2d;
  border-radius: 16px; 
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
}

.window-content {
  padding: 20px;
  color: white;
  height: 100%;
  box-sizing: border-box;
}
</style>