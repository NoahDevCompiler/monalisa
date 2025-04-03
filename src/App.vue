<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
  MinusIcon,
  StopIcon,
  XMarkIcon,
  ViewfinderCircleIcon,
} from "@heroicons/vue/24/outline";
import Navigation from "./Components/Navigation.vue";
import SidebarContent from "./Components/FileManager.vue";
import TextField from "./Components/TextField.vue";
import { openSettingsWindow } from "./utils/windows";
import { info } from "@tauri-apps/plugin-log";
import Header from "./Components/Header.vue";

const appWindow = getCurrentWindow();

//const minimize = async () => {
//  appWindow.minimize();
//};
//
//const maximize = async () => {
//  appWindow.toggleMaximize();
//};
//
//const close = async () => {
//  appWindow.close();
//};

const openSettings = async () => {
  try{
    await openSettingsWindow();
  }
  catch(e){
    info("Failed ")
  }

};


</script>

<template>
  <main class="flex">
    <Header />

    <div
      class="sidebar overflow-hidden top-7 left-0 w-[300px] h-screen bg-[#A69888] pt-2 break-words relative"
    >
      <div><SidebarContent class="text-[#FDF0D5] font-quickSand" /></div>
    </div>
    <div class="flex-1 relative">
      <TextField class="absolute inset-0" />

      <ViewfinderCircleIcon
        @click="openSettings"
        class="size-9 flex fixed m-2 bottom-0 left-0 text-white"
      />

      <Navigation class="fixed z-1000" />
    </div>
  </main>
</template>

<style scoped>
* {
  user-select: none;

  pointer-events: auto;
}
html,
body {
  overflow: hidden;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  overflow: hidden;
  color: #0f0f0f;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
  -webkit-user-drag: auto;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #12100e;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>