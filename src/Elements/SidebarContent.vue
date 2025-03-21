<script lang="ts" setup>
import { PencilSquareIcon, FolderPlusIcon } from "@heroicons/vue/24/outline";
import { info } from "@tauri-apps/plugin-log";
import { popAnimation } from "../utils/motion.js";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const motionProps = ref({ animate: { scale: 1 } });
const key = ref(false);
const name = ref("ABU");

async function createFolder(name) {
  try {
    await invoke("create_folder", { name: name });
    info("Folder created");
  } catch (error) {
    info("Error while creating");
  }
  data.value.push({
    label: `newFolder`,
    children: [],
  });
  motionProps.value = popAnimation;
  info("clicked folder");
  key.value = true;
  setTimeout(() => {
    key.value = false;
  }, 100);
}
const createFile = () => {
  info("clicked file");
};
const data = ref([
  {
    label: "Level one 1",
    children: [
      {
        label: "Level two 1-1",
        children: [
          {
            label: "Level three 1-1-1",
          },
        ],
      },
    ],
  },
  {
    label: "Level one 2",
    children: [
      {
        label: "Level two 2-1",
        children: [
          {
            label: "Level three 2-1-1",
          },
        ],
      },
      {
        label: "File 2-2",
      },
    ],
  },
  {
    label: "File 3",
    children: [],
  },
]);
</script>
<template>
  <div
    class="icons top-0 gap-2 flex flex-row justify-center items-center text-[white]"
  >
    <FolderPlusIcon
      :style="{ transform: key ? 'scale(1.25)' : 'scale(1)' }"
      @click="createFolder(name)"
      class="size-5"
    />
    <PencilSquareIcon
      :style="{ transform: key ? 'scale(1.25)' : 'scale(1)' }"
      @click="createFile"
      class="size-5"
    />
  </div>
  <el-tree
    class="w-full h-full bg-transparent text-[#FDF0D5]"
    :allow-drop="allowDrop"
    :allow-drag="allowDrag"
    :data="data"
    draggable
    default-expand-all
    node-key="id"
    @node-drag-start="handleDragStart"
    @node-drag-enter="handleDragEnter"
    @node-drag-leave="handleDragLeave"
    @node-drag-over="handleDragOver"
    @node-drag-end="handleDragEnd"
    @node-drop="handleDrop"
  />
</template>

<style>
* {
  user-select: none !important;
}

.el-tree-node__content .el-icon {
  color: black;
}

.el-tree-node:hover {
  background-color: transparent !important;
  color: inherit !important;
}

.el-tree-node__content:hover {
  background-color: #736b60 !important;
  color: inherit !important;
  box-shadow: none !important;
}

.el-tree-node.is-current .el-tree-node__content {
  background-color: transparent !important;
  color: inherit !important;
  box-shadow: none !important;
}

.el-tree-node:focus,
.el-tree-node.is-current:focus {
  background-color: transparent !important;
  outline: none !important;
}

.el-tree {
  --el-tree-node-hover-bg-color: transparent !important;
}
</style>
  