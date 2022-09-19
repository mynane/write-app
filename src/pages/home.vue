
<template>
  <div>
    <div class="home-header">
      <el-input
        v-model.trim="search"
        :placeholder="$t('home.inputRepUrl')"
        v-on:keyup.enter.native="searchRep"
        :suffix-icon="Search"
      />
      <div class="home-header-add">
        <el-tooltip :content="$t('common.add')"
          ><el-button
            type="primary"
            :icon="Edit"
            circle
            @click="visible = true"
        /></el-tooltip>
      </div>
    </div>

    <div class="home-body">
      <rep-card
        v-for="item in rep.items"
        :key="item.uri"
        :item="item"
        :basic_dir="rep.basic_dir"
      ></rep-card>
    </div>
  </div>
  <el-dialog v-model="visible" :title="$t('common.add')" width="80%">
    <el-input v-model="repository" :placeholder="$t('common.keyword')" />
    <template #footer>
      <el-space>
        <el-button @click="visible = false">{{
          $t("common.cancel")
        }}</el-button>
        <el-button type="primary" @click="onCreateRep">{{
          $t("common.confirm")
        }}</el-button>
      </el-space>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
// git@github.com:mynane/rust-swc.git
// https://github.com/mynane/rust-swc.git
import { Command } from "@tauri-apps/api/shell";
import { invoke } from "@tauri-apps/api/tauri";
import { ElMessage } from "element-plus";
import { getCurrentInstance, onMounted, reactive, ref } from "vue";
import { Search, Edit } from "@element-plus/icons-vue";
import RepCard from "~/components/repCard/index.vue";

let { proxy }: any = getCurrentInstance();

const repository = ref<string>("");
const rep = ref<any>({
  basic_dir: "",
  items: [{ uri: "1", name: "xiaoduo1" }],
});
const visible = ref<boolean>(false);
const search = ref<string>("");

async function get_rep() {
  try {
    rep.value = await invoke("get_repositories");
  } catch (error) {}
}

onMounted(async () => {
  await get_rep();
});

async function searchRep() {}

async function onCreateRep() {
  var repRegex =
    /^(https|git)(@|:\/\/)([a-z0-9\.-]+)(:|\/)([a-z0-9-]+)\/([a-z0-9-]+)\.git$/;

  let result: RegExpMatchArray | null = repository.value.match(repRegex);

  if (!result) {
    ElMessage.error(proxy.$t("common.fail"));
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
    ElMessage.success(proxy.$t("common.success"));
    visible.value = false;
  } catch (error) {
    ElMessage.error(proxy.$t("common.fail"));
  }
}
</script>

<style scoped lang="scss">
.home-header {
  display: flex;
  &-add {
    margin-left: 20px;
  }
}
</style>