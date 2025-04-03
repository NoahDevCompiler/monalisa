<script setup lang="ts">
import { ref, onMounted } from "vue";
import {
  EditorView,
  Decoration,
  WidgetType,
  MatchDecorator,
  DecorationSet,
  ViewPlugin,
  ViewUpdate,
} from "@codemirror/view";
import { minimalSetup } from "codemirror";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { tags } from "@lezer/highlight";
import { oneDark } from "@codemirror/theme-one-dark";

const editorEl = ref(null);

const markdownHighlight = HighlightStyle.define([
  {
    tag: tags.heading1,
    fontSize: "1.7em",
    fontWeight: "bold",
    color: "#ff5e62",
  },
  {
    tag: tags.heading2,
    fontSize: "1.4em",
    fontWeight: "bold",
    color: "#ff9966",
  },
  {
    tag: tags.heading3,
    fontSize: "1.2em",
    fontWeight: "bold",
    color: "#ffcc66",
  },
  { tag: tags.monospace, fontFamily: "monospace" },
]);

onMounted(() => {
  const view = new EditorView({
    doc: "# Start writing here...",
    extensions: [
      minimalSetup,
      markdown(),
      oneDark,
      syntaxHighlighting(markdownHighlight),
      EditorView.lineWrapping,
    ],
    parent: editorEl.value,
  });

  // Editor anpassen, damit Höhe richtig berechnet wird
  setTimeout(() => {
    view.requestMeasure();
  }, 50);
});
</script>

<template>
  <div class="editor-container">
    <el-scrollbar class="scroll-container">
      <div class="editor-wrapper">
        <div ref="editorEl" class="editor-content" />
      </div>
    </el-scrollbar>
  </div>
</template>

<style>
.editor-container {
  position: absolute;
  top: 60px; 
  bottom: 0;
  left: 0;
  right: 0;
  display: flex;
  flex-direction: column;
}

.scroll-container {

  height: calc(100vh - 60px); 
  overflow: auto;
}

.editor-wrapper {
  min-height: 100%;
  width: 100%;
  padding-top: 80px;
  padding-left: 60px;
}
@media (max-width: 1024px) {
  .editor-wrapper {
    padding-left: 20px;
  }
}

.editor-content {
  min-height: 100%;
  width: 100%;
}


.cm-editor {
  height: auto !important;
  overflow: hidden !important;
  background: transparent;
  outline: none !important;
}

.cm-scroller {
  overflow: hidden !important;
}

</style>