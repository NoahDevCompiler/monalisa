<script setup>
import Prism from "prismjs";
import "prismjs/components/prism-csharp.js";
import "prismjs/components/prism-rust.js";
import "prismjs/components/prism-python.js";
import "prismjs/components/prism-javascript";
import { marked } from "marked";

const inputText = ref("#Untitled");

marked.setOptions({
  breaks: true,
  gfm: true,
  sanitize: true, 
  smartypants: true
});

const renderedMarkdown = computed(() => {
  return marked(inputText.value)
});
</script>

<template>
  <textarea
    v-html="compiledMarkdown"
    v-model="inputText"
    class="resize-none border-none focus:outline-none bg-black text-white p-2"
  ></textarea>
  <div class="preview" v-html="inputText"></div>
</template>

<style scoped>
.editor-container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  height: 100vh;
  padding: 16px;
}

.editor, .preview {
  padding: 12px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  font-family: -apple-system, BlinkMacSystemFont, sans-serif;
  font-size: 14px;
  line-height: 1.5;
}

.editor {
  background: #f8f8f8;
  resize: none;
}

.preview {
  overflow-y: auto;
}
</style>
  