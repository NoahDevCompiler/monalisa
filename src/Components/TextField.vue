<script setup lang="ts">
import { ref, watch, onMounted, defineProps, defineEmits } from "vue";
import { Editor, EditorContent } from "@tiptap/vue-3";
import StarterKit from "@tiptap/starter-kit";
import Focus from "@tiptap/extension-focus";
import Heading from "@tiptap/extension-heading";
import Paragraph from "@tiptap/extension-paragraph";
import CodeBlock from "@tiptap/extension-code-block";

const props = defineProps({
  content: String,
});
const emit = defineEmits(["update:content"]);

const content = ref(props.content);
const editor = ref<Editor | null>(null);

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
    ],
    onUpdate: ({ editor }) => {
      emit("update:content", editor.getHTML());
    },
  });
  console.log("text content", content.value);
});

watch(
  () => props.content,
  (newContent) => {
    if (editor.value && newContent !== editor.value.getHTML()) {
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
