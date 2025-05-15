<script lang="ts" setup>
import { DocumentPlusIcon, FolderPlusIcon, DocumentIcon, FolderIcon } from "@heroicons/vue/24/outline";
import { info } from "@tauri-apps/plugin-log";
import { popAnimation } from "../utils/motion.js";
import { ref, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { treeNodeProps } from "element-plus/es/components/tree-v2/src/virtual-tree.mjs";
import { onMounted, defineEmits } from "vue";

//Give parameter of folder to backend to save node in parent folder
//when i click the folder icon to create a new folder and i click inside the input and press enter leaving the input empty
//the folder doesnt get created BUT after clicking the folder icon again no input appears to create a new folder

const motionProps = ref({ animate: { scale: 1 } });
const key = ref(false);
const treeRef = ref<any>(null);
const selectedNode = ref<TreeNode | null>(null);
const isSaving = ref(false);

const emit = defineEmits(["file-selected"]);

async function getVault() {
  let state = await invoke('get_active_vault_path')
  console.log(state)
}

type TreeNode = {
  id: number;
  label: string;
  type: "folder" | "file";
  path?: string;
  children?: TreeNode[];
  editing?: boolean;
};
type DirEntry = {
  name: string;
  path: string;
  is_dir: boolean;
  size?: number;
};
type DirContent = {
  entries: DirEntry[];
};

function allowDrop(draggingNode: any, dropNode: any, type: string) {
  return dropNode.data.type == "folder";
}
onMounted(() => {
  getVault();
  read_dir();
});


const handleNodeClick = (data: TreeNode) => {
  selectedNode.value = data;

  if (data.type == "file") {
    emit("file-selected", data.path);
  }
};

async function read_dir() {
  console.log("Read_directory called");
  const dirContent = await invoke<DirContent>("read_directory");
  console.log(dirContent);

  const createNode = (entry) => {
    const node = {
      id: Date.now() + Math.random(),
      label: entry.name,
      type: entry.is_dir ? "folder" : "file",
      path: entry.path,
      children: entry.children ? entry.children.map(createNode) : [],
    };
    return node;
  };

  const nodes = dirContent.children.map(createNode);

  nodes.forEach((node) => {
    treeData.value.push(node);
  });

  console.log(nodes);
  return nodes;
}

const focusInput = async (id: number) => {
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
    id: Date.now(),
    label: "",
    type: "folder",
    path: parentPath,
    children: [],
    editing: true,
  };
  parent!.push(newNode);
  focusInput(newNode.id);
  //info("clicked folder");
}

async function addFile() {
  const parent =
    selectedNode.value?.type === "folder"
      ? selectedNode.value.children || treeData.value
      : treeData.value;

  const parentPath =
    selectedNode.value?.type === "folder"
      ? selectedNode.value.path || selectedNode.value.label
      : "";

  const newNode: TreeNode = {
    id: Date.now(),
    label: "",
    type: "file",
    path: parentPath,
    children: [],
    editing: true,
  };
  parent!.push(newNode);
  focusInput(newNode.id);

  //info("clicked file");
}

const saveNode = async (node: TreeNode, parent?: TreeNode[]) => {
  if (isSaving.value) {
    return;
  }
  isSaving.value = true;
  info("Called SaveNode");

  node.editing = false;
  node.label = node.label.trim();

  if (!node.label) {
    if (parent) {
      parent.splice(parent.indexOf(node), 1);
    }
    treeData.value = treeData.value.filter((n: TreeNode) => n.id !== node.id); //deletes node with id
    isSaving.value = false;
    return;
  }
  try {
    const parentPath =
      selectedNode.value?.type === "folder" ? selectedNode.value.path : "";
    if (parentPath) {
      //info("Selected parent node path: ");
      //info(parentPath);
    }
    node.path = parentPath + node.label + "/";

    if (node.type == "folder") {
      await invoke("create_folder", { name: node.label, folder: parentPath });
      info("Folder created");
    } else if (node.type == "file") {
      await invoke("create_md_file", { name: node.label, folder: parentPath });
    }
    //info("CREATED NODE PATH:");
    //info(node.path);
  } catch (error) {
    //info("Error while creating");
  } finally {
    isSaving.value = false;
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
    class="icons gap-2 flex flex-row justify-center items-center text-[white]"
  >
    <FolderPlusIcon @click="addFolder()" class="size-5 cursor-pointer" />
    <DocumentPlusIcon @click="addFile()" class="size-5 cursor-pointer" />
  </div>
  <el-scrollbar class="scroll-container" style="height: 100vh">
    <el-tree
      ref="treeRef"
      class="custom-tree w-[95%] h-full bg-transparent text-[#FDF0D5] mt-5 flex flex-col gap-2"
      :data="treeData"
      default-expand-all
      node-key="id"
      @node-click="handleNodeClick"
      @node-drop="handleDrop"
      :allow-drop="allowDrop"
      draggable
    >
      <template #default="{ node, data }">
        <div class="node-content">
          <el-icon v-if="data.type === 'folder'">
            <FolderIcon/>
          </el-icon>
          <el-icon v-if="data.type === 'file'">
           <DocumentIcon/>
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
      <template #empty>
        <div
          class="pt-10 justify-center items-center flex font-quickSand text-xl"
        >
          Empty Vault
        </div>
      </template>
    </el-tree>
  </el-scrollbar>
</template>

<style>
* {
  user-select: none !important;
}

.cm-scroller {
  overflow: hidden !important;
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
  