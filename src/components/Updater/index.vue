<script setup lang="ts">
import { getVersion } from "@tauri-apps/api/app";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { relaunch } from "@tauri-apps/api/process";
import {
  checkUpdate,
  installUpdate,
  onUpdaterEvent,
  UpdateResult,
  UpdateStatus,
} from "@tauri-apps/api/updater";
import { ElNotification } from "element-plus";
import { marked } from "marked";
import { onMounted, ref, h, onBeforeUnmount, onBeforeMount } from "vue";

const listens: UnlistenFn[] = [];

const dialogVisible = ref<boolean>(false);
const updateInfo = ref<UpdateResult>();
const progress = ref<{ current: number; total: number }>({
  current: 0,
  total: 0,
});
const updateStatus = ref<UpdateStatus>("PENDING");
const loading = ref<boolean>(false);

onBeforeMount(async () => {
  const updateEventListens = await onUpdaterEvent(({ error, status }) => {
    updateStatus.value = status;

    if (status === "DONE") {
      progress.value = { current: 0, total: 0 };
    }
  });

  const progressListens = await listen<{
    chunkLength: number;
    contentLength?: number;
  }>("tauri://update-download-progress", function (event) {
    progress.value = {
      current: progress.value.current + event.payload.chunkLength,
      total: event.payload.contentLength ?? progress.value.total,
    };
  });

  listens.push(updateEventListens, progressListens);
});

onMounted(async () => {
  await check();

  let timer = setInterval(async () => {
    await check();
  }, 60 * 60 * 1000);

  listens.push(() => {
    clearInterval(timer);
  });
});

onBeforeUnmount(async () => {
  listens.map(async (listen) => await listen());
});

async function update() {
  loading.value = true;
  if (updateStatus.value !== "DONE") {
    await installUpdate();
  } else {
    await relaunch();
  }
  loading.value = false;
}

async function check() {
  loading.value = true;
  const update = await checkUpdate();

  if (
    dialogVisible.value &&
    update.manifest?.version === updateInfo.value?.manifest?.version
  ) {
    loading.value = false;
    return;
  }

  if (update.shouldUpdate) {
    await installUpdate();
    updateInfo.value = update;
    dialogVisible.value = true;
  }
  // else {
  //     ElNotification({
  //       title: "版本检测",
  //       message: h("i", { style: "color: teal" }, "已经是最新版本"),
  // });
  //   }

  loading.value = false;
}
</script>

<style lang="less" scoped>
</style>

<template>
  <el-dialog
    v-model="dialogVisible"
    :title="updateInfo?.manifest?.version"
    width="60%"
    :close-on-click-modal="false"
  >
    <div v-html="marked(updateInfo?.manifest?.body, { sanitize: true })" />
    <div v-if="!!progress.total">
      <el-progress
        :percentage="((progress.current / progress.total) * 100).toFixed(0)"
      />
    </div>
    <template #footer>
      <span class="dialog-footer">
        <el-button :loading="loading" @click="dialogVisible = false"
          >取消</el-button
        >
        <el-button :loading="loading" type="primary" @click="update">{{
          updateStatus !== "DONE" ? "下载" : "重启升级"
        }}</el-button>
      </span>
    </template>
  </el-dialog>
</template>