<script setup>
import { ref } from "vue";
import {
  PencilSquareIcon,
  FolderPlusIcon,
  CodeBracketIcon,
  PlusIcon,
  CursorArrowRaysIcon,
} from "@heroicons/vue/24/outline";
import { info } from "@tauri-apps/plugin-log";
import learncards from "../assets/learncards.png";

const emit = defineEmits(["handle-button"]);

const isOpen = ref(false);
const icons = ref([
  { id: 1, icon: learncards, color: "9984D4", label: "Show LearnCard Sets" },
  {
    id: 2,
    icon: CursorArrowRaysIcon,
    color: "CAA8F5",
    label: "AI Polish & Diagrams",
  },
  { id: 3, icon: CodeBracketIcon, color: "8A4FFF", label: "Add Code Snipet" },
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
        :style="{
          transform: isOpen
            ? `translate(${
                Math.cos((index / (icons.length - 1)) * (Math.PI / 2)) * -80
              }px, ${
                Math.sin((index / (icons.length - 1)) * (Math.PI / 2)) * -80
              }px)`
            : 'translate(0, 0)',
          opacity: isOpen ? '1' : '0',
          transitionDelay: `${index * 100}ms`,
        }"
        class="absolute flex items-center justify-center transition-all duration-500 ease-in-out"
      >
        <div
          @click="$emit('handle-button', index)"
          class="group relative w-12 h-12 flex items-center justify-center bg-[#CAA8F5] shadow-none hover:shadow-lg hover:shadow-indigo-500 hover:transition-none rounded-full cursor-pointer"
        >
          <div
            class="absolute bg-gray-800 text-white text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-80 transition-opacity duration-200 z-50 whitespace-nowrap"
            :class="
              item.id === 1 || item.id === 2
                ? '-left-[90px] -translate-x-[50px]'
                : '-top-11 left-1/3 -translate-x-1/2'
            "
          >
            {{ item.label || "Button" + (index + 1) }}
          </div>
          <img
            v-if="typeof item.icon === 'string'"
            :src="item.icon"
            class="w-6 h-6 filter invert"
          />
          <component v-else :is="item.icon" class="w-6 h-6 text-white" />
        </div>
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
