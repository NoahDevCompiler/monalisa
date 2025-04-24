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


</script>

<template>
  <div class="toolbar z-[1000] select-none pointer-events-auto items-stretch">
    <div class="flex right-0 items-stretch">
      <button
        @click="minimize"
        class="button text-white hover:bg-[#169976ae] rounded-none"
      >
        <MinusIcon class="size-5" />
      </button>
      <button
        @click="maximize"
        class="button text-white hover:bg-[#1699769f] rounded-none"
      >
        <StopIcon class="size-5" />
      </button>
      <button
        @click="close"
        class="button text-white hover:bg-[#ff3636cd] rounded-none"
      >
        <XMarkIcon class="size-5" />
      </button>
    </div>
  </div>
</template>
<style>
.button {
  flex: 1;
  justify-content: center;
  display: flex;
  height: 30px;
  align-items: center;
  width: 36px;
  border: none;
  box-shadow: none;
}
</style>