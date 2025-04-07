<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { MinusIcon, StopIcon, XMarkIcon } from "@heroicons/vue/24/outline";
import { onMounted, ref } from "vue";
import type { WebviewWindow as WebviewWindowType } from "@tauri-apps/api/webviewWindow";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import TabManager from "./TabManager.vue";

const props = defineProps({
  windowLabel: {
    type: String,
    default: null,
  },
});

const appWindow = ref<WebviewWindow | null>(null);

const initializeWindow = async () => {
  if (props.windowLabel) {
    const window = await WebviewWindow.getByLabel(props.windowLabel);
    appWindow.value = window;
  } else {
    appWindow.value = getCurrentWindow() as unknown as WebviewWindowType;
  }
};

initializeWindow();
const minimize = async () => {
  if (!appWindow.value) return;
  await appWindow.value.minimize();
};

const maximize = async () => {
  if (!appWindow.value) return;
  await appWindow.value.toggleMaximize();
};

const close = async () => {
  if (!appWindow.value) return;
  await appWindow.value.close();
};

onMounted(() => {
  const titlebar = document.getElementById("toolbar");

  if (titlebar) {
    titlebar.addEventListener("mousedown", async (e) => {
      console.log("Mouse down event fired");
      if (!appWindow.value) return;

      const clickedElement = e.target as HTMLElement;
      console.log("Clicked element:", clickedElement.tagName);
      if (
        clickedElement.tagName !== "BUTTON" &&
        !clickedElement.closest("button")
      ) {
        if (e.buttons === 1) {
          if (e.detail === 2) {
            await appWindow.value.toggleMaximize();
          } else {
            await appWindow.value.startDragging();
          }
        }
      }
    });
  }
});
</script>

<template>
  <div
    class="toolbar fixed top-0 left-0 h-8 flex bg-[#3A3A3A] justify-between z-1000 select-none pointer-events-auto"
    id="toolbar"
  >
    <div class="h-8 overflow-hidden justify-center items-center flex-grow"><TabManager/></div>
    <div class=""></div>
    <div class="flex items-center">
      <button @click="minimize" class="button text-white hover:bg-slate-400 h-full rounded-none">
        <MinusIcon class="size-5" />
      </button>
      <button @click="maximize" class="button text-white hover:bg-slate-400 rounded-none h-full">
        <StopIcon class="size-5" /> 
      </button>
      <button @click="close" class="button text-white hover:bg-red-500 rounded-none h-full">
        <XMarkIcon class="size-5" />
      </button>
    </div>
  </div>
</template>
<style>
#toolbar {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 30px;
  z-index: 9999; 
  -webkit-app-region: drag; 
}
.button {
  justify-content: center;
  display: flex;
  align-items: center;
  width: 36px;
  border: none;
  box-shadow: none;
}
</style>