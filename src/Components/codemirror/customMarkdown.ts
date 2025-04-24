
import { HighlightStyle, tags } from '@codemirror/highlight';

export const markdownHighlight = HighlightStyle.define([
  { tag: tags.heading1, fontSize: "2em", color: "red" },
  { tag: tags.heading2, fontSize: "1.5em", color: "green" },
  { tag: tags.emphasis, color: "yellow" },
]);
