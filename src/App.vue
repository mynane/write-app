<template>
  <el-config-provider namespace="ep">
    <div class="wrap">
      <div class="base-side">
        <BaseSide />
      </div>
      <div class="content">
        <router-view />
      </div>
    </div>
    <Updater />
  </el-config-provider>
</template>

<script setup lang="ts">
import Updater from "./components/Updater/index.vue";
import useClientDark from "~/hooks/useClientDark.vue";
import { onMounted } from "@vue/runtime-core";
import { getBasicDir, setBasicDir } from "./serviece/client/rep";
import { dialog } from "@tauri-apps/api";
import { useI18n } from "vue-i18n";
import { ComponentInternalInstance, getCurrentInstance } from "vue";
const { isDark, toggleTheme } = useClientDark();

let { proxy }: any = getCurrentInstance();

onMounted(async () => {
  try {
    const basic_dir = await getBasicDir();

    if (!basic_dir) {
      const selected = await dialog.open({
        title: proxy.$t("common.basicDir"),
        directory: true,
      });

      console.log(selected);

      if (selected) {
        await setBasicDir(selected as string);
      }
    }
  } catch (error) {
    console.log(error);
  }
});
</script>

<style lang="scss">
#app {
  color: var(--ep-text-color-primary);
}

.wrap {
  width: 100vw;
  height: 100vh;
  display: flex;

  .ep-menu {
    height: 100vh;
  }
}

.content {
  height: 100vh;
  overflow-y: auto;
  padding: 20px;
  flex: 1;
  box-sizing: border-box;
}
</style>
