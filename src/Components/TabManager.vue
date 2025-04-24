<template>
  <div class="tabs">
    <div class="tab-buttons">
      <button
        class="buttons"
        v-for="tab in tabs"
        :key="tab.id"
        :class="{ active: activeTab === tab.id }"
        @click="activeTab = tab.id"
      >
        {{ tab.name }}
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";

const activeTab = ref(0)

let tabs = [
  {
    id: 1,
    name: "tab1",
  },
  {
    id: 2,
    name: "tab2",
  },
  {
    id: 3,
    name: "tab3",
  },
  {
    id: 4,
    name: "tab4",
  },
  {
    id: 5,
    name: "tab5",
  },
];

const props = defineProps({
  tabs: Array,
  activeTab: String,
});
const emit = defineEmits(["close"]);

function closeTab(id) {
  emit("close", id);
}
</script>

<style scoped>
.buttons {
  border: none;
  height: 30px;
  width: 100px;
}
.buttons.active {
  background-color: #18122B;
}
</style>

