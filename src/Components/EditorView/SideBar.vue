<script setup>
import { ref } from "vue";
const isExpanded = ref(false);

function toggleSidebar() {
  isExpanded.value = !isExpanded.value;
}

const items = [
  { emoji: "🧠", label: "AI Tools" },
  { emoji: "📝", label: "Markdown Syntaxer" },
  {emoji: "🛠️", label: "Settings"},
];
</script>

<template>
  <div
    :class="[
      'fixed top-1/2 right-0 -translate-y-1/2 z-50 justify-center items-center bg-transparent rounded-md flex flex-col gap-3 border-none transition-all duration-300',
      isExpanded ? 'w-[25vh] h-[65vh]' : 'w-[8vh] h-[65vh] items-center',
    ]"
  >
    <div
      v-for="(item, i) in items"
      :key="i"
      class="flex items-center hover:bg-indigo-500 rounded px-2 py-1 align-center h-[60px] cursor-pointer w-full font-quickSand text-black transition"
      :class="{
        'justify-center': !isExpanded,
        'text-xl' : !isExpanded
      }"
    >
      {{ item.emoji }}
      <span v-if="isExpanded" class="ml-2">{{ item.label }}</span>
    </div>

    <div class="w-full bg-gray-800 h-[1px]"></div>
  </div>
</template>

<style scoped>
#buttons {
  height: 5vh;
}
</style>