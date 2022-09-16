<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import VConsole from "vconsole";

import { getVersion } from "@tauri-apps/api/app";
import { platform } from "@tauri-apps/api/os";
import { useUpdaterStore } from "../stores/updater";

interface IConfigs {
  spctlMasterDisable: boolean;
}

const version = ref<string>("");
const pf = ref<string>("");
const updater = useUpdaterStore();

onMounted(async () => {
  const res = await invoke<IConfigs>("get_configs");

  const appVersion = await getVersion();
  pf.value = await platform();
  version.value = appVersion;
  const vConsole = new VConsole();
});

async function spctl_master_disable() {
  try {
    const result = await invoke("spctl_master_disable");
  } catch (error) {
    console.log(error);
  }
}
</script>

<template>
  <div>{{ version }}</div>
  <el-row class="mb-4">
    <el-button @click="updater.check">检查更新</el-button>
    <el-button v-if="pf == 'darwin'" @click="spctl_master_disable"
      >获取权限</el-button
    >
  </el-row>
</template>
