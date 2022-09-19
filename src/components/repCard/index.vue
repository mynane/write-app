<template>
  <div class="rep-card ep-card">
    <div class="rep-content">{{ props.item.name }}</div>
    <div class="rep-footer">
      <el-button
        @click="onClone"
        :loading="loading.clone"
        :disabled="item.isCloned"
        >{{ $t("common.clone") }}</el-button
      >
      <el-button @click="onOpenDir" :loading="loading.open">{{
        $t("common.open")
      }}</el-button>
      <el-button @click="onOpenCode" :loading="loading.code">{{
        $t("common.code")
      }}</el-button>
      <el-button @click="onDelete" :loading="loading.delete">{{
        $t("common.delete")
      }}</el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
// 导入withDefaults父默认值
import { Command } from "@tauri-apps/api/shell";
import { ElMessage } from "element-plus";
import { getCurrentInstance, reactive, ref, withDefaults } from "vue";
import { openDir } from "~/serviece/client/common";
import { createRep } from "~/serviece/client/rep";

let { proxy }: any = getCurrentInstance();

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
});

async function onOpenDir() {
  try {
    const { basic_dir } = props;
    const { host, group, name } = props.item;
    await openDir(`${basic_dir}/${host}/${group}/${name}`);
    ElMessage.success(proxy.$t("common.success"));
  } catch (error) {
    ElMessage.error(proxy.$t("common.fail"));
  }
}

async function onClone() {
  const { item } = props;
  try {
    const path: any = await createRep(item);
    //${path} && git clone ${item.uri}
    loading.clone = true;
    const command = new Command("git", ["clone", item.uri], { cwd: path });
    command.on("close", (data) => {
      if (!data.code) {
        ElMessage.success(proxy.$t("common.success"));
      } else {
        ElMessage.error(proxy.$t("common.fail"));
      }

      loading.clone = false;
    });
    // command.on("error", (error) => console.error(`command error: "${error}"`));
    // command.stdout.on("data", (line) =>
    //   console.log(`command stdout: "${line}"`)
    // );
    // command.stderr.on("data", (line) =>
    //   console.log(`command stderr: "${line}"`)
    // );
    const child = await command.spawn();
    // console.log("pid:", child.pid);
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
      console.log(
        `command finished with code ${data.code} and signal ${data.signal}`
      );
      loading.code = false;
    });
    //   command.on("error", (error) => console.error(`command error: "${error}"`));
    //   command.stdout.on("data", (line) => console.log(`command stdout: "${line}"`));
    //   command.stderr.on("data", (line) => console.log(`command stderr: "${line}"`));
    const child = await command.spawn();
    //   console.log("pid:", child.pid);
  } catch (error) {
  } finally {
    loading.code = false;
  }
}

async function onDelete() {
  ElMessage.warning("not impl");
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