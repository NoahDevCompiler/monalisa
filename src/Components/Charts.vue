<template>
  <div>
    <div ref="chartContainer"></div>
  </div>
</template>
  
<script setup>

import { ref, onMounted, nextTick } from "vue";
import mermaid from "mermaid";

const chartContainer = ref(null);

const chartDefinition = `
 ---
title: "Grades"
---
radar-beta
  axis m["Math"], s["Science"], e["English"]
  axis h["History"], g["Geography"], a["Art"]
  curve a["Alice"]{85, 90, 80, 70, 75, 90}
  curve b["Bob"]{70, 75, 85, 80, 90, 85}

  max 100
  min 0
  `;

onMounted(async () => {
  mermaid.initialize({
    startOnLoad: false,
    theme: "base",
    themeVariables: {
      primaryTextColor: "#fff",
    },
  });

  await nextTick();

  const id = "mermaid-pie-chart";

  try {
    const { svg } = await mermaid.render(id, chartDefinition);
    chartContainer.value.innerHTML = svg;
    console.log(svg);
  } catch (err) {
    console.error("Mermaid rendering error:", err);
  }
});

</script>
  <style scoped>
</style>
  