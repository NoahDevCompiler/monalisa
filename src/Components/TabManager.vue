<template>
  <div class="tab-container">
    <ul class="tabs clearfix">
      <li
        v-for="tab in tablist"
        :key="tab"
        :class="{ active: activeTab === tab }" 
        @click="activeTab = tab"
  
      >
        <a href="#">{{ tab }}</a>
      </li>
    </ul>
  </div>
  <div class="outer-circle"></div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue';

const tablist = ["Newtab1", "Newtab2", "Newtab2133", "Newtab3", "Newtab6", "Newtab12353", "Newt123ab33", "Newtab31233", "Newtab31233", "Ned2dwta12db33", "1d", "Newta21d2db33"];
const activeTab = ref("Newtab1");
</script>

<style scoped>
body {
  user-select: all;
}

.tab-container {
  box-sizing:content-box;
  background: transparent;
  display: flex;
  flex-direction: row;
  margin-left: 300px;
}

ul.tabs {
  margin: 0;
  list-style-type: none;
  line-height: 35px;
  max-height: 35px;
  overflow: hidden;
  display: inline-block;
  padding-right: 20px;
}

ul.tabs > li.active {
  z-index: 2;
  background: #12100e;
  color: #FDF0D5;
}

ul.tabs > li.active:before {
  border-color: transparent #efefef transparent transparent;
}

ul.tabs > li.active:after {
  border-color: transparent transparent transparent #22ff00;
}

ul.tabs > li {
  float: right;
  margin: 5px -10px 0;
  border-top-right-radius: 25px 170px;
  border-top-left-radius: 20px 90px;
  padding: 0 30px 0 25px;
  height: 170px;
  background: #ddd;
  position: relative;
  box-shadow: 0 10px 20px rgba(0, 0, 0, 0.5);
  max-width: 200px;
}

ul.tabs > li > a {
  display: inline-block;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  text-decoration: none;
  color: #000000;
}

ul.tabs > li.active > a {
  color: #FDF0D5;
}

ul.tabs > li:before,
ul.tabs > li:after {
  content: "";
  background: transparent;
  height: 20px;
  width: 20px;
  border-radius: 100%;
  border-width: 10px;
  top: 0px;
  border-style: solid;
  position: absolute;
}

ul.tabs > li:before {
  border-color: transparent hsl(0, 0%, 87%) transparent transparent;
  -webkit-transform: rotate(48deg);
  left: -23px;
}

ul.tabs > li:after {
  border-color: transparent transparent transparent #ddd;
  -webkit-transform: rotate(-48deg);
  right: -17px;
}

.clearfix:before,
.clearfix:after {
  content: "";
  display: table;
}

.clearfix:after {
  clear: both;
}

.clearfix {
  zoom: 1;
}
</style>
