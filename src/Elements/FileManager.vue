<script lang="ts" setup>
import { PencilSquareIcon, FolderPlusIcon } from "@heroicons/vue/24/outline";
import { info } from "@tauri-apps/plugin-log";
import { popAnimation } from "../utils/motion.js";
import { ref, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";

//A) Edit name on creation, stay with vanilla design and icons, edit mode till out of focus (Enter or clicked out of element), 
//B) Edit name in file textarea
//C) Call backend when item out of focus (Enter or clicked out of element)
//D) when create while focus on Folder, create child of parent(focused node)


const motionProps = ref({ animate: { scale: 1 } });
const key = ref(false);
const treeRef = ref<any>(null);

type TreeNode = {
  id: string;
  label: string;
  type: "folder" | "file";
  children?: TreeNode[];
  editing?: boolean;
};

const focusInput = async (id: string) => {
  await nextTick();
  const input = document.getElementById(`input-${id}`) as HTMLInputElement;
  if (input) input.focus();
};

async function addFolder(node: TreeNode | null) {
  const parent = node || { children: treeData.value };

  const newNode: TreeNode = {
    id: Date.now().toString(),
    label: "",
    type: "folder",
    editing: true,
  };
  parent.children!.push(newNode);
  focusInput(newNode.id);

  info("clicked folder");
}
const saveNode = async (node: TreeNode, parent?: TreeNode[]) => {
  if(node.editing){
    info(node.editing.toString());
  }
  node.label = node.label.trim();
  if (!node.label) {
    if (parent) {
      parent.splice(parent.indexOf(node), 1);
    }
  }
  try {
    await invoke("create_folder", { name: node.label });
    info("Folder created");
  } catch (error) {
    info("Error while creating");
  }
};
const createFile = () => {
  info("clicked file");
};
const treeData = ref<TreeNode[]>([]);
</script>
<template>
  <div
    class="icons top-0 gap-2 flex flex-row justify-center items-center text-[white]"
  >
    <FolderPlusIcon @click="addFolder(null)" class="size-5 cursor-pointer" />
    <PencilSquareIcon class="size-5 cursor-pointer" />
  </div>

  <el-tree
    class="w-full h-full bg-transparent text-[#FDF0D5]"
    :allow-drop="allowDrop"
    :allow-drag="allowDrag"
    :data="treeData"
    draggable
    default-expand-all
    node-key="id"
    @node-drop="handleDrop"
  >
    <template #default="{ node, data }">
      <span v-if="data.editing">
        <input
          :id="'input-' + data.id"
          v-model="data.label"
          @blur="saveNode(data, node.parent?.data.children)"
          @keyup.enter="saveNode(data, node.parent?.data.children)"
          class="edit-input"
        />
      </span>
      <span
        v-else
        @dblclick="
          data.editing = true;
          focusInput(data.id);
        "
      >
        {{
          data.label || (data.type === "folder" ? "Neuer Ordner" : "Neue Datei")
        }}
      </span>
    </template>
  </el-tree>
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
  