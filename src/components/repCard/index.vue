<template>
  <div class="rep-card ep-card">
    <div class="rep-content">
      <a href="javascript:void(0);" @click="openLink"
        >{{ props.item.group }}/{{ props.item.name }}</a
      >
    </div>
    <div class="rep-footer">
      <el-button
        @click="onClone"
        :loading="loading.clone"
        :disabled="item.is_cloned"
        v-if="!item.is_cloned"
        >{{ $t("common.clone") }}</el-button
      >
      <el-button
        @click="onOpenDir"
        :loading="loading.open"
        v-if="item.is_cloned"
        >{{ $t("common.open") }}</el-button
      >
      <el-button
        @click="onOpenCode"
        :loading="loading.code"
        v-if="item.is_cloned"
        >{{ $t("common.code") }}</el-button
      >
      <el-button
        @click="onRemoveDir"
        :loading="loading.delete"
        v-if="item.is_cloned"
        >{{ $t("common.delete") }}</el-button
      >
    </div>
  </div>
</template>

<script setup lang="ts">
// 导入withDefaults父默认值
import { fs } from "@tauri-apps/api";
import { Command } from "@tauri-apps/api/shell";
import { Action, ElMessage, ElMessageBox } from "element-plus";
import {
  getCurrentInstance,
  reactive,
  ref,
  withDefaults,
  defineEmits,
  onMounted,
  onUnmounted,
} from "vue";
import { openDir, removeDir } from "~/serviece/client/common";
import { createRep, patchRep, removeRep } from "~/serviece/client/rep";
import { open } from "@tauri-apps/api/shell";
import { cloneManager } from "~/utils/cloneMananger";

let { proxy }: any = getCurrentInstance();
const emit = defineEmits(["reload"]);

const loading = reactive({
  clone: false,
  delete: false,
  open: false,
  code: false,
});

// 定义需要传入的props类型
interface PropsType {
  item?: any;
  basic_dir?: string;
}

// 定义props
const props = withDefaults(defineProps<PropsType>(), {
  item: {},
  basic_dir: "",
  useProxy: false,
});

async function openLink() {
  const { host, group, name, protocol } = props.item;

  await open(`https://${host}/${group}/${name}`);
}

async function onOpenDir() {
  try {
    loading.open = true;
    const { basic_dir } = props;
    const { host, group, name } = props.item;
    await openDir(`${basic_dir}/${host}/${group}/${name}`);
    ElMessage.success(proxy.$t("common.success"));
  } catch (error) {
    ElMessage.error(proxy.$t("common.fail"));
  } finally {
    loading.open = false;
  }
}

async function onRemoveDir() {
  ElMessageBox.alert(
    proxy.$t("common.continueToDelete"),
    proxy.$t("common.important"),
    {
      showCancelButton: true,
      cancelButtonText: proxy.$t("common.cancel"),
      confirmButtonText: proxy.$t("common.confirm"),
      callback: async (action: Action) => {
        if (action === "confirm") {
          await remvePath();
        }
      },
    }
  );
}

async function remvePath() {
  try {
    loading.delete = true;
    const { uri } = props.item;
    await removeRep(uri);
    ElMessage.success(proxy.$t("common.success"));
  } catch (error) {
    ElMessage.error(proxy.$t("common.fail"));
  } finally {
    emit("reload");
    loading.delete = false;
  }
}

onMounted(async () => {
  cloneManager.addEventListener(
    props.item.uri,
    async (isloading: boolean, hasError: boolean) => {
      console.log(isloading, hasError);
      loading.clone = isloading;
      if (typeof hasError === "boolean") {
        if (hasError) {
          ElMessage.error(proxy.$t("common.fail"));
          try {
            await removeDir(
              `${props.basic_dir}/${props.item.host}/${props.item.group}/${props.item.name}`
            );
          } catch (error) {}
        } else {
          ElMessage.success(proxy.$t("common.success"));
          console.log(123123);
          await patchRep(props.item.uri, { is_cloned: true });
          emit("reload");
        }
      }
    }
  );
});

onUnmounted(async () => {
  cloneManager.removeEventListener(props.item.uri);
});

async function onClone() {
  const { item } = props;
  try {
    const path: any = await createRep(item);
    cloneManager.clone(item.uri, path);
  } catch (error) {
    console.log(error);
  }
}

async function onOpenCode() {
  const { item } = props;
  try {
    loading.code = true;
    const command = new Command("code", ["."], {
      cwd: `${props.basic_dir}/${item.host}/${item.group}/${item.name}`,
    });

    command.on("close", (data) => {
      if (!data.code) {
        ElMessage.success(proxy.$t("common.success"));
      } else {
        ElMessage.error(proxy.$t("common.fail"));
      }
      loading.code = false;
    });
    command.on("error", (error) => console.error(`command error: "${error}"`));
    command.stdout.on("data", (line) =>
      console.log(`command stdout: "${line}"`)
    );
    command.stderr.on("data", (line) =>
      console.log(`command stderr: "${line}"`)
    );
    const child = await command.spawn();

    console.log("pid:", child.pid);
  } catch (error) {
  } finally {
    loading.code = false;
  }
}
console.log(props.item);
</script>

<style scoped lang="scss">
.rep-card {
  margin: 10px 0;
}
.rep-content {
  padding: 10px;
  min-height: 50px;
  display: flex;
  align-items: center;
}
.rep-footer {
  display: flex;
  flex-direction: row;
  border-top: 1px solid var(--ep-card-border-color);

  .ep-button + .ep-button {
    margin-left: 0;
    border-left: 1px solid var(--ep-card-border-color);
    border-radius: 0;
  }

  & > .ep-button {
    border: none;
    flex: 1;
    padding: initial;
  }
}
</style>