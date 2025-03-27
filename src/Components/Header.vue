<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { MinusIcon, StopIcon, XMarkIcon } from "@heroicons/vue/24/outline";
import { onMounted, ref } from "vue";
import type { WebviewWindow as WebviewWindowType } from "@tauri-apps/api/webviewWindow";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

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
      if (!appWindow.value) return;

      const clickedElement = e.target as HTMLElement;
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
    class="toolbar fixed top-0 left-0 w-full h-8 flex items-center bg-[#3A3A3A] justify-between z-10 select-none"
    id="toolbar"
  >
    <div class="flex items-center space-x-4">
      <button class="text-white">Weitere Links</button>
    </div>

    <div class="flex items-center space-x-4">
      <button @click="minimize" class="text-white hover:bg-slate-400">
        <MinusIcon class="size-5" />
      </button>
      <button @click="maximize" class="text-white hover:bg-slate-400">
        <StopIcon class="size-5" />
      </button>
      <button @click="close" class="text-white hover:bg-red-500">
        <XMarkIcon class="size-5" />
      </button>
    </div>
  </div>
</template>