<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
<<<<<<< HEAD
import { ref, onBeforeUnmount, onMounted, watch, nextTick } from "vue";
=======
import { ref, onBeforeUnmount, onMounted, watch } from "vue";
>>>>>>> a1c51f146ddd533656bc92b4961c423d8e7b0598
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
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
import learncards from "./assets/learncards.png";
import Header from "./Components/Header.vue";
import TabManager from "./Components/TabManager.vue";
import Charts from "./Components/Charts.vue";
import { callOpenAI } from "./api/ai-api.ts";

const appWindow = getCurrentWindow();
<<<<<<< HEAD
const selectedFilePath = ref<string>("");
const loadContent = ref("");
const updateContent = ref("");
const isSaving = ref(false);
const showCards = ref(false);
=======
const selectedFilePath = ref(null);
const loadContent = ref("");
const updateContent = ref("");
const isSaving = ref(false);
>>>>>>> a1c51f146ddd533656bc92b4961c423d8e7b0598

onMounted(() => {
  //listen SubWindow
  listen("tauri://destroyed", (event) => {
    settingsOpen.value = false;
  });
  //Watch Dragging Window
  const titlebar = document.getElementById("toolbar");

  if (titlebar) {
    titlebar.addEventListener("mousedown", async (e) => {
      if (!appWindow) return;

      const clickedElement = e.target as HTMLElement;
      if (
        clickedElement.tagName !== "BUTTON" &&
        !clickedElement.closest("button") &&
        clickedElement.id !== "resizer"
      ) {
        if (e.buttons === 1) {
          if (e.detail === 2) {
            await appWindow.toggleMaximize();
          } else {
            await appWindow.startDragging();
          }
        }
      }
    });
  }
});

<<<<<<< HEAD
async function handleFileSelected(newPath: any) {
  if (isSaving.value) return;
  const currentPath = selectedFilePath.value;
  const currentContent = updateContent.value;

  selectedFilePath.value = newPath;

  if (currentPath != newPath) {
    isSaving.value = true;
    console.log("isSaving");
    try {
      console.log("Call SaveFile");
      await saveFile(currentPath, currentContent);
    } finally {
      isSaving.value = false;
    }
  }
  try {
    loadContent.value = await invoke("read_file", { path: newPath });
    updateContent.value = loadContent.value;
  } catch (e) {
    console.error("Error on loading new Content", e);
  }
}

document.addEventListener("keydown", async (e) => {
  if (e.ctrlKey && e.key === "s") {
    e.preventDefault();
    await saveFile(selectedFilePath.value);
    console.log("CTRL + S");
  }
});

watch(
  () => updateContent.value,
  async () => {
    const currentPath = selectedFilePath.value
    const currentContent = updateContent.value
    setTimeout(async function () {
      await saveFile(currentPath, currentContent);
    }, 1000);
  }
);
async function saveFile(path: string | null, content?: string) {
  if (path == null) return;
  await invoke("write_file", {
    path: path,
    content: content,
  });
  console.log("Saved file with content", updateContent.value);
}

const handleClickNavigation = (index) => {
  console.log(showCards.value);
  if (index == 0) {
    showCards.value = true;
  }
  if (index == 1) {
    console.log(updateContent.value);
    callOpenAI(updateContent.value)
      .then((response: string) => {
        loadContent.value = response;
        console.log("ai response", response);
        console.log("new Content for TextEditor", loadContent);
      })
      .catch((e: string) => {
        console.log(e);
      });
  }
};
=======
async function handleFileSelected(path: any) {
  //if (updateContent.value !== null && selectedFilePath !== path) {
  //  await saveFile();
  //}
  //updateContent.value = "";
  selectedFilePath.value = path;
  loadContent.value = await invoke("read_file", { path: path });
}

document.addEventListener('keydown', e => {
  if (e.ctrlKey && e.key === 's') {
    
    e.preventDefault();
    saveFile();
    console.log('CTRL + S');
  }
});

//function debounce(fn: Function, delay: number) {
//  let timeout: ReturnType<typeof setTimeout>;
//  return (...args: any[]) => {
//    clearTimeout(timeout);
//    timeout = setTimeout(() => fn(...args), delay);
//  };
//}
//const debouncedSave = debounce(saveFile, 2000);
//
async function saveFile() {
  if (isSaving.value || !selectedFilePath.value) return;
  isSaving.value = true;
  await invoke("write_file", {
    path: selectedFilePath.value,
    content: updateContent.value,
  });
  console.log("Saved file with content", updateContent.value);
  isSaving.value = false;
}
//watch(updateContent, (newContent) => {
//  debouncedSave();
//});
>>>>>>> a1c51f146ddd533656bc92b4961c423d8e7b0598

const sidebarWidth = ref(300);
let isResizing = false;

const startResizing = (e) => {
  isResizing = true;
  document.addEventListener("mousemove", resize);
  document.addEventListener("mouseup", stopResizing);
};

const resize = (e) => {
  if (isResizing) {
    const maxWidth = window.innerWidth * 0.5;
    sidebarWidth.value = Math.max(Math.min(e.clientX, maxWidth), 200);
  }
};

const stopResizing = () => {
  isResizing = false;
  document.removeEventListener("mousemove", resize);
  document.removeEventListener("mouseup", stopResizing);
};

onBeforeUnmount(() => {
  stopResizing();
});

const settingsOpen = ref(false);
const openSettings = async () => {
  try {
    await openSettingsWindow();
    settingsOpen.value = true;
  } catch (e) {
    info("Failed ");
  }
};
</script>

<template>
  <main class="app-grid" :class="settingsOpen ? 'blur-sm' : 'blur-none'">
    <div class="toolbar-grid" id="toolbar">
      <div class="toolbar-tabs" :style="{ width: sidebarWidth + 'px' }"></div>

      <div
        class="w-[1px] fixed h-full rounded-lg z-10000 cursor-col-resize bg-gray-500 transition-colors hover:bg-[#1DCD9F] hover:w-1"
        @mousedown="startResizing"
        :style="{ left: sidebarWidth + 'px' }"
        id="resizer"
      ></div>

      <div class="flex w-full h-[30px] overflow-hidden">
        <div class="flex-1 min-w-0 overflow-hidden items-center flex px-2">
          <TabManager class="w-full h-[30px] overflow-hidden" />
        </div>

        <div class="shrink-0 bg-gray-400/40 flex justify-end items-center">
          <Header />
        </div>
      </div>
    </div>

    <div
      class="sidebar bg-[#222222]"
      style="grid-column: 1; grid-row: 2"
      :style="{ width: sidebarWidth + 'px' }"
      ref="sidebar"
    >
      <SidebarContent
        @file-selected="handleFileSelected"
        class="text-[#FDF0D5]"
      />
    </div>

    <div class="main-content relative flex flex-col">
      <TextField
<<<<<<< HEAD
        v-if="!showCards"
=======
>>>>>>> a1c51f146ddd533656bc92b4961c423d8e7b0598
        :content="loadContent"
        v-model="updateContent"
        class="absolute inset-0"
      />
<<<<<<< HEAD
      <Charts v-else class="absolute inset-0"></Charts>
=======
>>>>>>> a1c51f146ddd533656bc92b4961c423d8e7b0598
      <ViewfinderCircleIcon
        @click="openSettings"
        class="size-9 flex fixed m-2 bottom-0 left-0 text-white"
      />
      <Navigation class="fixed z-1000" @handle-button="handleClickNavigation" />
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
.app-grid {
  display: grid;
  grid-template-columns: auto 4px 1fr;
  grid-template-rows: 30px 1fr;
  height: 100vh;
  overflow: hidden;
}

.toolbar-grid {
  display: grid;
  grid-template-columns: auto 1fr auto;
  grid-column: 1 / 4;
  background-color: #2e2e2e;
  color: white;
  height: 30px;
  border-bottom: 1px solid gray;
}

.toolbar-sidebar {
  display: flex;
  align-items: center;
  padding-left: 8px;
  border-right: 1px solid gray;
}

.toolbar-tabs {
  flex: 1 1 auto;

  display: flex;
  align-items: center;
  justify-content: flex-start;
  padding-left: 8px;
  flex-grow: 1;
}

.toolbar-buttons {
  min-width: 100px;
  display: flex;
  justify-content: flex-end;
  align-items: center;
}

.sidebar {
  grid-column: 1;
  grid-row: 2;
  overflow: hidden;
  width: 100%;
}

.main-content {
  grid-column: 3 / 4;
  grid-row: 2;
  background-color: #000000;
  position: relative;
}

.w-8 {
  position: absolute;
  top: 0;
  left: var(--sidebar-width, 300px);
  cursor: col-resize;
  background-color: #555;
  z-index: 10000;
  height: 100%;
}
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  overflow: hidden;
  color: #000000;

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
    background-color: #000000;
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