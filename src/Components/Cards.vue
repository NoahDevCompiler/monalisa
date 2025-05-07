<script setup>
import { invoke } from "@tauri-apps/api/core";

async function load() {
    await invoke("get_cards")
}
</script>

<template>
    <div class="items-center justify-center flex bg-green-500">
        <div class="cards bg-orange-700 rounded-lg w-[50vh] h-[70vh]">

            <div class="relative top-20 w-full  bg-black"></div>
        </div>
    </div>
</template>
