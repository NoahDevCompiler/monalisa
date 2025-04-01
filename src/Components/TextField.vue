<script setup lang="ts">
import { ref, onMounted } from "vue";
import { EditorView, Decoration, WidgetType, MatchDecorator, DecorationSet, ViewPlugin, ViewUpdate } from "@codemirror/view";
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
  new EditorView({
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
});
</script>

<template>
  <div ref="editorEl" class="absolute whitespace-pre-wrap break-all top-8" />
</template>

<style>
.cm-editor {
  height: 100vh !important;
  background: transparent;
  overflow: hidden;
  padding-top: 60px;
  padding-left: 60px;
}
@media (min-width: 1024px) {
  .cm-editor {
    padding-left: 140px !important;
  }
}
.cm-scroller {
  height: 100% !important;
  width: 100% !important;
  overflow: auto !important;
}
.cm-content {
  text-align: left;
}
.cm-line {
  height: auto !important;
}
</style>