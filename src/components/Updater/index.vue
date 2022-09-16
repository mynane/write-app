<script setup lang="ts">
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import {
  onUpdaterEvent,
  UpdateResult,
  UpdateStatus,
} from "@tauri-apps/api/updater";
import { marked } from "marked";
import { onMounted, ref, h, onBeforeUnmount, onBeforeMount } from "vue";
import { useUpdaterStore } from "../../stores/updater";

const listens: UnlistenFn[] = [];

const progress = ref<{ current: number; total: number }>({
  current: 0,
  total: 0,
});

const updater = useUpdaterStore();

onBeforeMount(async () => {
  const updateEventListens = await onUpdaterEvent(({ error, status }) => {
    updater.status = status;

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
  await updater.checkAndUpdate();

  let timer = setInterval(async () => {
    await updater.checkAndUpdate();
  }, 60 * 60 * 1000);

  listens.push(() => {
    clearInterval(timer);
  });
});

onBeforeUnmount(async () => {
  listens.map(async (listen) => await listen());
});
</script>

<style lang="less" scoped>
</style>

<template>
  <el-dialog
    v-model="updater.visible"
    :title="updater?.updateInfo?.version"
    width="60%"
    :close-on-click-modal="false"
  >
    <div v-html="marked(updater?.updateInfo?.body ?? '', { sanitize: true })" />
    <div v-if="!!progress.total">
      <el-progress
        :percentage="
          Number(((progress.current / progress.total) * 100).toFixed(0))
        "
      />
    </div>
    <template #footer>
      <span class="dialog-footer">
        <el-button :loading="updater.loading" @click="updater.visible = false"
          >取消</el-button
        >
        <el-button
          :loading="updater.loading"
          type="primary"
          @click="updater.update"
          >{{ updater.status !== "DONE" ? "下载" : "重启升级" }}</el-button
        >
      </span>
    </template>
  </el-dialog>
</template>