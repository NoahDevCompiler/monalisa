<script setup lang="ts">
import { ref, watch, onMounted, defineProps, defineEmits, nextTick } from "vue";
import { BubbleMenu, Editor, EditorContent } from "@tiptap/vue-3";
import StarterKit from "@tiptap/starter-kit";
import Focus from "@tiptap/extension-focus";
import Heading from "@tiptap/extension-heading";
import Paragraph from "@tiptap/extension-paragraph";
import CodeBlock from "@tiptap/extension-code-block";
import Dropcursor from "@tiptap/extension-dropcursor";
import Image from "@tiptap/extension-image";
import Highlight from "@tiptap/extension-highlight";
import { Node, mergeAttributes } from "@tiptap/core";
import SideBar from "./SideBar.vue";

const props = defineProps<{
  content: String;
  isLoading: boolean;
}>();
const emit = defineEmits(["update:modelValue"]);

const content = ref(props.content);
const editor = ref<Editor | null>(null);

const handleImageInput = () => {
  editor.value?.commands.setImage({
    src: "../../public/monalisalogo.png",
  });
};

onMounted(() => {
  editor.value = new Editor({
    content: content.value,
    extensions: [
      StarterKit.configure({
        bulletList: {
          HTMLAttributes: {
            class: "list-disc ml-8",
          },
        },
      }),
      Focus.configure({
        className: "has-focus",
        mode: "deepest",
      }),
      Heading.configure({
        levels: [1, 2, 3],
      }),
      Paragraph,
      CodeBlock,
      Image,
      Dropcursor,
      Highlight.configure({ multicolor: true }),
    ],
    onUpdate: ({ editor }) => {
      console.log("Content on Update", editor.getHTML());
      emit("update:modelValue", editor.getHTML());
    },
  });
  console.log("text content on load:", content.value);
});

watch(
  () => props.content,
  async (newContent) => {
    console.log("watch texteditor ausgelöst");
    await nextTick();
    if (editor.value && newContent !== editor.value.getHTML()) {
      console.log("editor value gesetzt", editor.value);
      editor.value.commands.setContent(newContent || "", false);
    }
  }
);
</script>

<template>
  <div class="editor-container">
    <SideBar />
    <el-scrollbar class="scroll-container">
      <div
        v-if="isLoading"
        class="animate-gradient-x bg-gradient-to-r w-screen fixed from-blue-500 via-purple-500 to-violet-500 h-[3px] rounded-full dark:bg-blue-500"
        style="width: 100%"
      ></div>

      <div class="editor-wrapper">
        <EditorContent
          :editor="editor"
          class="editor-content font-quickSand tiptap"
        />
      </div>

      <bubble-menu
        :editor="editor"
        :tippy-options="{ duration: 100 }"
        v-if="editor"
      >
        <div class="bubble-menu">
          <button
            class="font-quickSand"
            @click="editor.chain().focus().toggleBold().run()"
            :class="{ 'is-active': editor.isActive('bold') }"
          >
            Bold
          </button>
          <button
            class="font-quickSand"
            @click="editor.chain().focus().toggleItalic().run()"
            :class="{ 'is-active': editor.isActive('italic') }"
          >
            Italic
          </button>
          <button
            class="font-quickSand"
            @click="HandleOpenAI"
            :class="{ 'is-active': editor.isActive('strike') }"
          >
            Strike
          </button>
        </div>
      </bubble-menu>
      <div
        @click="handleImageInput"
        class="fixed flex top-[6px] w-[10vh] justify-center items-center align-middle left-1 rounded-md bg-indigo-500 hover:opacity-50 font-quickSand whitespace-nowrap text-xs"
      >
        Add Charts
      </div>
    </el-scrollbar>
  </div>
</template>

<style>
.editor-container {
  position: absolute;
  top: 20px;
  bottom: 0;
  display: flex;
  flex-direction: column;
}

.scroll-container {
  height: calc(100vh - 60px);
  overflow: auto;
}

.editor-wrapper {
  height: 100%;
  width: 70vw;
  padding-top: 80px;
  padding-left: 150px;
}

@media (max-width: 1024px) {
  .editor-wrapper {
    padding-left: 20px;
  }
}

.tiptap {
  height: 100% !important;
  overflow: hidden !important;
  background: transparent;
  outline: none !important;
}

.tiptap h1 {
  margin-top: 1.5em;
  font-size: 1.9em;
  font-weight: bold;
}
.tiptap h2 {
  margin-top: 1.5em;
  font-size: 1.6em;
  font-weight: 600;
}
.tiptap h3 {
  margin-top: 1.5em;
  font-size: 1.3em;
  font-weight: 600;
}
.tiptap code {
  color: inherit;
  font-size: 0.8rem;
}
.tiptap mark {
  border-radius: 0.4rem;
}

.cm-scroller {
  overflow: hidden !important;
}

.bubble-menu {
  background-color: white;
  border: 1px solid var(--gray-1);
  border-radius: 0.7rem;
  box-shadow: var(--shadow);
  display: flex;
  gap: 0.2rem;
  padding: 0.3rem 1rem;
}
.bubble-menu button {
  background: white;
  color: black;
  border: none;
  padding: 0.4rem 0.8rem; /* Mehr Platz im Button */
  border-radius: 0.4rem;
  font-size: 0.9rem;
  cursor: pointer;
  transition: background-color 0.2s ease;
}
.bubble-menu button:hover {
  background-color: gray;
}
</style>
