<script lang="ts" setup>
import { PencilSquareIcon, FolderPlusIcon } from "@heroicons/vue/24/outline";
import { info } from "@tauri-apps/plugin-log";
import { popAnimation } from "../utils/motion.js";
import { ref, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { treeNodeProps } from "element-plus/es/components/tree-v2/src/virtual-tree.mjs";

//Give parameter of folder to backend to save node in parent folder
//when i click the folder icon to create a new folder and i click inside the input and press enter leaving the input empty
//the folder doesnt get created BUT after clicking the folder icon again no input appears to create a new folder

const motionProps = ref({ animate: { scale: 1 } });
const key = ref(false);
const treeRef = ref<any>(null);
const selectedNode = ref<TreeNode | null>(null);

type TreeNode = {
  id: string;
  label: string;
  type: "folder" | "file";
  path?: string;
  children?: TreeNode[];
  editing?: boolean;
};

const handleDrop = () => {
};

const handleNodeClick = (data: TreeNode) => {
  selectedNode.value = data;
  if (selectedNode.value) {
    info(selectedNode.value.label)
  }
};

const focusInput = async (id: string) => {
  await nextTick();
  const input = document.getElementById(`input-${id}`) as HTMLInputElement;
  if (input) {
    input.focus();
    input.select();
  }
};

async function addFolder() {
  const parent =
    selectedNode.value?.type === "folder"
      ? selectedNode.value.children
      : treeData.value;

  const parentPath =
    selectedNode.value?.type === "folder"
      ? selectedNode.value.path || selectedNode.value.label
      : "";

  const newNode: TreeNode = {
    id: Date.now().toString(),
    label: "",
    type: "folder",
    path: parentPath,
    children: [],
    editing: true,
  };
  parent!.push(newNode);
  focusInput(newNode.id);
  info("clicked folder");
}

async function addFile() {
  const parent =
    selectedNode.value?.type === "folder"
      ? selectedNode.value.children
      : treeData.value;

  const parentPath =
    selectedNode.value?.type === "folder"
      ? selectedNode.value.path || selectedNode.value.label
      : "";

  const newNode: TreeNode = {
    id: Date.now().toString(),
    label: "",
    type: "file",
    path: parentPath,
    children: [],
    editing: true,
  };
  parent!.push(newNode);
  focusInput(newNode.id);

  info("clicked file");
}

const saveNode = async (node: TreeNode, parent?: TreeNode[]) => {
  node.editing = false;
  node.label = node.label.trim();

  if (!node.label) {
    if (parent) {
      parent.splice(parent.indexOf(node), 1);
    }
    treeData.value = treeData.value.filter((n: TreeNode) => n.id !== node.id); //deletes node with id
    return;
  }
  try {
    const parentPath =
      selectedNode.value?.type === "folder"
        ? selectedNode.value.path?.endsWith("/")
          ? selectedNode.value.path.slice(0, -1)
          : selectedNode.value.path || selectedNode.value.label
        : "";

    node.path = parentPath + `/${node.path}`;
    if (selectedNode.value?.path) {
      info("Selected node: ");
      info(selectedNode.value?.path);
      info("Node Path: ");
      info(node.path);
    }
    if (node.type == "folder") {
      await invoke("create_folder", { name: node.label });
      info("Folder created");
    } else if (node.type == "file") {
      await invoke("create_md_file", { name: node.label, folder: parentPath });
    }
    info(node.path);
  } catch (error) {
    info("Error while creating");
  }
};
const handleDoubleClick = (data: TreeNode) => {
  if (!data.editing) {
    data.editing = true;
    focusInput(data.id);
    return false;
  }
};
const treeData = ref<TreeNode[]>([]);
</script>

<template>
  <div
    class="icons top-0 gap-2 flex flex-row justify-center items-center text-[white]"
  >
    <FolderPlusIcon @click="addFolder()" class="size-5 cursor-pointer" />
    <PencilSquareIcon @click="addFile()" class="size-5 cursor-pointer" />
  </div>

  <el-tree
    ref="treeRef"
    class="custom-tree w-full h-full bg-transparent text-[#FDF0D5]"
    :data="treeData"
    default-expand-all
    node-key="id"
    @node-click="handleNodeClick"
    @node-drop="handleDrop"
    draggable
  >
    <template #default="{ node, data }">
      <div class="node-content">
        <el-icon v-if="data.type === 'folder'">
          <svg
            viewBox="0 0 1024 1024"
            xmlns="http://www.w3.org/2000/svg"
            width="1em"
            height="1em"
          >
            <path
              fill="currentColor"
              d="M128 192v640h768V320H485.76L357.504 192H128zm-32-64h287.872l128.384 128H928a32 32 0 0 1 32 32v576a32 32 0 0 1-32 32H96a32 32 0 0 1-32-32V160a32 32 0 0 1 32-32z"
            />
          </svg>
        </el-icon>
        <span v-if="data.editing" class="edit-wrapper">
          <input
            :id="'input-' + data.id"
            v-model="data.label"
            @blur="saveNode(data, node.parent?.data.children)"
            @keyup.enter="saveNode(data, node.parent?.data.children)"
            class="edit-input"
          />
        </span>
        <span v-else @dblclick.stop="handleDoubleClick(data)">
          {{
            data.label || (data.type === "folder" ? "New Folder" : "New File")
          }}
        </span>
      </div>
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

.el-tree > .el-tree__drop-indicator {
  background-color: transparent;
}
.el-tree-node__placeholder {
  background: #5a5248 !important;
}

.el-tree-node__content:hover {
  background-color: #736b60 !important;
  color: inherit !important;
  box-shadow: none !important;
}

.el-tree-node.is-current .el-tree-node__content {
  background-color: #736b60 !important;
  color: inherit !important;
  box-shadow: none !important;
}

.el-tree-node:focus,
.el-tree-node.is-current:focus {
  background-color: transparent !important;
  outline: none !important;
}
.custom-tree .el-tree-node__content:hover {
  background-color: #736b60;
  border-radius: 4px;
}
.edit-input {
  background: transparent;
  border: none;
  color: inherit;
  font: inherit;
  padding: 0;
  margin: 0;
  width: 100%;
  outline: none;
}
.node-content {
  display: flex;
  align-items: center;
  gap: 6px;
  width: 100%;
}

.edit-wrapper {
  flex: 1;
}
</style>
  