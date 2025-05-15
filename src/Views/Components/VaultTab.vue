<script setup>
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

const emit = defineEmits(['closeWindow'])

const createVault = async () => {
  try {
    await invoke("set_active_vault", {
      path: "C:\\Users\\Noah\\Documents\\VaultMault",
    });
    console.log("Gespeichert");
  } catch (e) {
    console.error("Error while saving vault", e);
  }
};

async function selectVaultFolder() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "Select existing Vault",
  });

  if (selected) {
    console.log("selected path:", selected);
    await invoke("set_active_vault", { path: selected });
    await invoke("config_add_vault", {path})
    emit('closeWindow', true)
  }
}
</script>
<template>
  <div class="flex flex-col items-center justify-center">
    <img
      src="/monalisalogo.png"
      class="w-[30vw] h-auto max-w-[300px] min-w-[150px]"
    />
    <div class="gap-3 flex flex-col">
      <div
        @click="selectVaultFolder"
        class="rounded-lg bg-[#a69888] w-[300px] justify-center items-center transition-shadow flex h-11 hover:shadow-lg hover:shadow-[#766c60] text-white text-[20px] tracking-widest font-quickSand"
      >
        Open Vault
      </div>
      <div
        @click="createVault"
        class="rounded-lg bg-[#a69888] w-[300px] justify-center items-center transition-shadow flex h-11 hover:shadow-lg hover:shadow-[#766c60] text-white text-[20px] tracking-widest font-quickSand"
      >
        Create Default Vault
      </div>
      <div
        @click="selectVaultFolder"
        class="rounded-lg bg-[#a69888] w-[300px] justify-center items-center transition-shadow flex h-11 hover:shadow-lg hover:shadow-[#766c60] text-white text-[20px] tracking-widest font-quickSand"
      >
        Delete Vaults
      </div>
    </div>
  </div>
</template>