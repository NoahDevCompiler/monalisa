<script setup>
import { ref } from "vue";
import {
  PencilSquareIcon,
  FolderPlusIcon,
  CodeBracketIcon,
  PlusIcon,
} from "@heroicons/vue/24/outline";
import { info } from "@tauri-apps/plugin-log";
const isOpen = ref(false);
const icons = ref([
  { icon: PencilSquareIcon, color: "9984D4" },
  { icon: FolderPlusIcon, color: "CAA8F5" },
  { icon: CodeBracketIcon, color: "8A4FFF" },
]);

const toggleMenu = () => {
  isOpen.value = !isOpen.value;
};

const positionStyle = (index, total) => {
  const angle = (index / total) * 2 * Math.PI;
  const radius = isOpen.value ? 80 : 0;
  return {
    transform: `translate(${Math.cos(angle) * radius}px, ${
      Math.sin(angle) * radius
    }px)`,
    transition: "transform 0.3s ease-in-out",
  };
};
</script>

<template>
    <div class="fixed bottom-5 right-5 flex items-center justify-center">
      <div
        class="absolute flex items-center justify-center rounded-tl-full transition-all duration-500 ease-in-out"
        :class="isOpen ? 'w-32 h-32 opacity-100' : 'w-14 h-14 opacity-0'"
      >
        <div
          v-for="(item, index) in icons"
          :key="index"
          class="absolute w-12 h-12 flex items-center justify-center bg-[#CAA8F5] shadow-none hover:shadow-lg hover:shadow-indigo-500 rounded-full  transition-all duration-500 ease-in-out"
          :style="{
            transform: isOpen
              ? `translate(${Math.cos((index / (icons.length - 1)) * (Math.PI / 2)) * -80}px, ${
                  Math.sin((index / (icons.length - 1)) * (Math.PI / 2)) * -80
                }px)`
              : 'translate(0, 0)',
            opacity: isOpen ? '1' : '0',
            transitionDelay: `${index * 100}ms`,
          }"
        >
          <component :is="item.icon" class="w-6 h-6 text-white" />
        </div>
      </div>
  
      <button
        @click="toggleMenu"
        class="relative w-14 h-14 bg-[#463F3A] rounded-full flex items-center justify-center transition-transform duration-300"
        :class="{ 'rotate-45': isOpen }"
      >
        <PlusIcon class="w-7 h-7 text-white transition-transform duration-300" />
      </button>
    </div>
  </template>
