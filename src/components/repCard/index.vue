<template>
  <div class="rep-card ep-card">
    <div class="rep-content">{{ props.item.group }}/{{ props.item.name }}</div>
    <div class="rep-footer">
      <el-button
        @click="onClone"
        :loading="loading.clone"
        :disabled="item.is_cloned"
        >{{ $t("common.clone") }}</el-button
      >
      <el-button @click="onOpenDir" :loading="loading.open">{{
        $t("common.open")
      }}</el-button>
      <el-button @click="onOpenCode" :loading="loading.code">{{
        $t("common.code")
      }}</el-button>
      <el-button @click="onRemoveDir" :loading="loading.delete">{{
        $t("common.delete")
      }}</el-button>
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
} from "vue";
import { openDir, removeDir } from "~/serviece/client/common";
import { createRep, patchRep, removeRep } from "~/serviece/client/rep";

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

async function onClone() {
  const { item, useProxy } = props;
  const { basic_dir } = props;
  const { host, group, name } = item;
  try {
    const path: any = await createRep(item);
    loading.clone = true;
    const command = new Command(
      "git",
      [
        "clone",
        `${
          item.uri.startsWith("https://github.com")
            ? "https://github.91chi.fun//"
            : ""
        }${item.uri}`,
      ],
      { cwd: path }
    );
    console.log(command);
    command.on("close", async (data) => {
      if (!data.code) {
        ElMessage.success(proxy.$t("common.success"));
        await patchRep(item.uri, { is_cloned: true });
        emit("reload");
      } else {
        ElMessage.error(proxy.$t("common.fail"));
      }

      loading.clone = false;
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
  margin: 20px 0;
}
.rep-content {
  padding: 10px;
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
  }
}
</style>