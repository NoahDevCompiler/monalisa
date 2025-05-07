<script setup lang="ts">
import { ref, watch, onMounted, defineProps, defineEmits, nextTick } from "vue";
import { Editor, EditorContent } from "@tiptap/vue-3";
import StarterKit from "@tiptap/starter-kit";
import Focus from "@tiptap/extension-focus";
import Heading from "@tiptap/extension-heading";
import Paragraph from "@tiptap/extension-paragraph";
import CodeBlock from "@tiptap/extension-code-block";
import Dropcursor from "@tiptap/extension-dropcursor";
import Image from "@tiptap/extension-image";

const props = defineProps<{
  content: String;
}>();
const emit = defineEmits(["update:modelValue"]);
const defaultView = ref(false);

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
    ],
    onUpdate: ({ editor }) => {
<<<<<<< HEAD
      console.log("Content on Update", editor.getHTML());
=======
      console.log("Content on Update", editor.getHTML())
>>>>>>> a1c51f146ddd533656bc92b4961c423d8e7b0598
      emit("update:modelValue", editor.getHTML());
    },
  });
  console.log("text content on load:", content.value);
});

watch(
  () => props.content,
  async (newContent) => {
<<<<<<< HEAD
    console.log("watch texteditor ausgelöst");
    await nextTick();
    if (editor.value && newContent !== editor.value.getHTML()) {
      console.log("editor value gesetzt", editor.value);
=======
    console.log("watch texteditor ausgelöst")
    await nextTick();
    if (editor.value && newContent !== editor.value.getHTML()) {
      console.log("editor value gesetzt", editor.value)
>>>>>>> a1c51f146ddd533656bc92b4961c423d8e7b0598
      editor.value.commands.setContent(newContent || "", false);
    }
  }
);
</script>

<template>
  <div class="editor-container">
    <el-scrollbar class="scroll-container">
      <div class="editor-wrapper">
        <EditorContent
          :editor="editor"
          class="editor-content font-quickSand tiptap" 
        />
      </div>
      <div
        @click="handleImageInput"
        class="fixed flex top-[6px] w-[10vh] justify-center items-center align-middle left-1 rounded-md bg-indigo-500 hover:opacity-50 font-quickSand whitespace-nowrap text-xs"
      >Add Charts</div>
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

.cm-scroller {
  overflow: hidden !important;
}
</style>
