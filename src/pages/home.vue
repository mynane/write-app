
<template>
  <div>
    <div class="home-header">
      <el-input
        v-model.trim="search"
        :placeholder="$t('common.keyword')"
        :suffix-icon="Search"
      />
      <div class="home-header-add" v-if="!!rep.basic_dir">
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
        v-for="item in rep.showItems"
        :key="item.uri"
        :item="item"
        :basic_dir="rep.basic_dir"
        :useProxy="configs.isGithubUseProxy"
        @reload="get_rep"
      ></rep-card>
    </div>
  </div>
  <el-dialog v-model="visible" :title="$t('common.add')" width="80%">
    <el-input v-model="repository" :placeholder="$t('home.inputRepUrl')" />
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
import { getConfigs } from "~/serviece/client/configs";
import { watch } from "vue";

let { proxy }: any = getCurrentInstance();

const repository = ref<string>("");
const rep = ref<any>({
  basic_dir: "",
  items: [],
  showItems: [],
});
const configs = ref<any>();
const visible = ref<boolean>(false);
const search = ref<string>("");

async function get_rep() {
  try {
    let temp: any = await invoke("get_repositories");
    rep.value = {
      basic_dir: temp.basic_dir,
      items: temp.items,
      showItems: temp.items,
    };
  } catch (error) {}
}

onMounted(async () => {
  configs.value = await getConfigs();
  await get_rep();
});

async function searchRep() {}

watch(
  () => search.value,
  (nval, oval) => {
    const { items } = rep.value;
    if (!nval.trim()) {
      rep.value.showItems = items;
      return;
    }
    rep.value.showItems = items.filter((item: any) => item.uri.includes(nval));
  }
);

async function onCreateRep() {
  var repRegex = /^(https|git)(@|:\/\/)([A-Za-z0-9\.-]+)(:|\/)(.+)\/(.+)\.git$/;

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
    repository.value = "";
  } catch (error) {
    ElMessage.error(proxy.$t("common.fail"));
  }
}
</script>

<style scoped lang="scss">
.home-header {
  position: sticky;
  top: 0;
  display: flex;
  background-color: var(--ep-card-bg-color);
  &-add {
    margin-left: 20px;
  }
}

.home-body {
  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-gap: 0 20px;
}
</style>