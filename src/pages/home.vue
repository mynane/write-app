
<template>
  <div>
    <el-input
      v-model="repository"
      placeholder="输入仓库地址"
      v-on:keyup.enter.native="searchPerson"
    />
    <div>
      <el-row :gutter="12">
        <el-col :span="8" v-for="item in rep.items" :key="item.uri">
          <el-card shadow="hover">
            <template #header>
              <div class="card-header">
                <span>{{ item.name }}</span>
                <el-button class="button" icon="ArrowRight" @click="openHome"
                  >打开目录页</el-button
                >
                <el-button
                  :loading="cloneLoading"
                  class="button"
                  @click="createRepFn(item)"
                  >clone</el-button
                >
                <el-button
                  :loading="openLoading"
                  class="button"
                  @click="openCode(item)"
                  >code</el-button
                >
              </div>
            </template>
            <div>{{ item.name }}</div>
          </el-card>
        </el-col>
      </el-row>
    </div>
  </div>
</template>

<script setup lang="ts">
// git@github.com:mynane/rust-swc.git
// https://github.com/mynane/rust-swc.git
import { Command } from "@tauri-apps/api/shell";
import { invoke } from "@tauri-apps/api/tauri";
import { ElMessage } from "element-plus";
import { onMounted, reactive, ref } from "vue";
import { openDir } from "~/serviece/client/common";
import { createRep } from "~/serviece/client/rep";

const repository = ref<string>("");
const rep = ref<any>({ items: [] });
const openLoading = ref<boolean>(false);
const cloneLoading = ref<boolean>(false);

async function openHome() {
  console.log(rep.value);
  await openDir(rep.value.basic_dir);
}

async function get_rep() {
  try {
    rep.value = await invoke("get_repositories");
  } catch (error) {}
}

onMounted(async () => {
  await get_rep();
});

async function openCode(item: any) {
  console.log(`${rep.value.basic_dir}/${item.host}/${item.host}/${item.name}`);
  openLoading.value = true;
  const command = new Command("code", ["."], {
    cwd: `${rep.value.basic_dir}/${item.host}/${item.group}/${item.name}`,
  });

  command.on("close", (data) => {
    if (!data.code) {
      ElMessage.success(`打开${item.name}成功`);
    }
    console.log(
      `command finished with code ${data.code} and signal ${data.signal}`
    );
    openLoading.value = false;
  });
  command.on("error", (error) => console.error(`command error: "${error}"`));
  command.stdout.on("data", (line) => console.log(`command stdout: "${line}"`));
  command.stderr.on("data", (line) => console.log(`command stderr: "${line}"`));
  const child = await command.spawn();
  console.log("pid:", child.pid);
}

async function createRepFn(item: any) {
  try {
    const path: any = await createRep(item);
    //${path} && git clone ${item.uri}
    cloneLoading.value = true;
    const command = new Command("git", ["clone", item.uri], { cwd: path });
    command.on("close", (data) => {
      if (!data.code) {
        ElMessage.success(`下载${item.name}成功`);
      }
      console.log(
        `command finished with code ${data.code} and signal ${data.signal}`
      );

      cloneLoading.value = false;
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

async function searchPerson() {
  var repRegex =
    /^(https|git)(@|:\/\/)([a-z0-9\.-]+)(:|\/)([a-z0-9-]+)\/([a-z0-9-]+)\.git$/;

  let result: RegExpMatchArray | null = repository.value.match(repRegex);

  if (!result) {
    ElMessage.error("输入链接不是一个仓库");
    return;
  }
  let result2 = {
    protocol: result[1],
    host: result[3],
    group: result[5],
    name: result[6],
  };

  try {
    await invoke("append_rep", {
      item: {
        ...result2,
        uri: repository.value,
      },
    });
    await get_rep();
  } catch (error) {
    ElMessage.error("添加链接失败");
  }

  console.log(result);
  console.log(repository);
}
</script>

<style scoped>
</style>