<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { marked } from "marked";
import { invoke } from "@tauri-apps/api/tauri";
import VConsole from "vconsole";

import {
  checkUpdate,
  installUpdate,
  onUpdaterEvent,
  UpdateStatus,
} from "@tauri-apps/api/updater";
import { relaunch } from "@tauri-apps/api/process";

const dialogVisible = ref<any>(false);
const updateInfo = ref<any>(null);
const unlisten = ref<any>(null);
const progress = ref<any>({ current: 0, total: 0 });
const updateStatus = ref<UpdateStatus>("PENDING");

async function greet() {
  const update = await checkUpdate();

  console.log("🚀 ~ file: Greet.vue ~ line 22 ~ greet ~ update", update);
  if (update.shouldUpdate) {
    updateInfo.value = update;
    dialogVisible.value = true;
  }
  // greetMsg.value = await invoke("greet", { name: name.value });
  // tasks.value = await invoke("get_all_tasks", {});

  // console.log(tasks);
}

onMounted(async () => {
  const vConsole = new VConsole();
  unlisten.value = await onUpdaterEvent(({ error, status }) => {
    console.log("Updater event", error, status);
    updateStatus.value = status;

    if (status === "DONE") {
      progress.value = { current: 0, total: 0 };
    }
  });
  listen<{ chunkLength: number; contentLength?: number }>(
    "tauri://update-download-progress",
    function (event) {
      progress.value = {
        current: progress.value.current + event.payload.chunkLength,
        total: event.payload.contentLength ?? progress.value.total,
      };
      console.log(
        event.payload,
        `downloaded ${event.payload.chunkLength} of ${event.payload.contentLength}`
      );
    }
  );
});

onUnmounted(() => {
  unlisten();
});

async function update() {
  if (updateStatus.value !== "DONE") {
    await installUpdate();
  } else {
    await relaunch();
  }
}
</script>

<template>
  <div>升级后查看</div>
  <el-row class="mb-4">
    <el-button @click="greet">检查更新</el-button>
  </el-row>

  <el-dialog
    v-model="dialogVisible"
    :title="updateInfo?.manifest?.version"
    width="60%"
  >
    <div v-html="marked(updateInfo?.manifest.body, { sanitize: true })" />
    <div v-if="!!progress.total">
      <el-progress
        :percentage="((progress.current / progress.total) * 100).toFixed(0)"
      />
    </div>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="update">{{
          updateStatus !== "DONE" ? "下载" : "重启升级"
        }}</el-button>
      </span>
    </template>
  </el-dialog>
</template>
