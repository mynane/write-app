
<template>
  <el-form ref="form" label-width="auto">
    <el-form-item :label="$t('setting.version')">
      <div>v{{ settings.version }}</div>
    </el-form-item>
    <el-form-item :label="$t('setting.themeMode')">
      <el-switch :model-value="isDark" @change="toggleTheme()" />
    </el-form-item>
    <!-- <el-form-item :label="$t('setting.githubProxy')">
      <el-switch
        :model-value="settings.configs.isGithubUseProxy"
        @change="toggleGithubProxy()"
      />
    </el-form-item> -->
    <el-form-item :label="$t('setting.language')">
      <el-select :model-value="$i18n.locale" @change="changeClientLang">
        <el-option
          v-for="locale in $i18n.availableLocales"
          :key="`locale-${locale}`"
          :value="locale"
          :label="$t(`lang.${locale}`)"
        >
          {{ $t(`lang.${locale}`) }}
        </el-option>
      </el-select>
    </el-form-item>
    <el-form-item :label="$t('setting.openAppDir')">
      <el-button :icon="Right" circle @click="openAppDir" />
    </el-form-item>
    <el-form-item :label="$t('setting.openLogsDir')">
      <el-button :icon="Right" circle @click="openLogsDir" />
    </el-form-item>
    <!-- <el-form-item :label="$t('setting.jira')">
      <el-button @click="jira.visible = true" type="primary">{{
        $t("common.add")
      }}</el-button>
    </el-form-item> -->
    <el-form-item
      :label="$t('setting.spctlMasterDisable')"
      v-if="settings.platform === 'darwin'"
    >
      <el-button @click="spctlMasterDisable" type="primary">{{
        $t("setting.getPermission")
      }}</el-button>
    </el-form-item>
  </el-form>
  <el-dialog v-model="jira.visible" :title="$t('setting.jira')">
    <div>
      <el-form :model="jira" label-width="80px">
        <el-form-item label="url">
          <el-input v-model="jira.url" />
        </el-form-item>
        <el-form-item label="username">
          <el-input v-model="jira.username" />
        </el-form-item>
        <el-form-item label="password">
          <el-input type="password" v-model="jira.password" />
        </el-form-item>
      </el-form>
    </div>
    <template #footer>
      <el-space>
        <el-button @click="jira.visible = false">{{
          $t("common.cancel")
        }}</el-button>
        <el-button type="primary" @click="onAddJira">{{
          $t("common.confirm")
        }}</el-button>
      </el-space>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import {
  getCurrentInstance,
  onBeforeMount,
  onMounted,
  reactive,
  ref,
} from "vue";
import { changeClientLang } from "~/locals";
import useClientDark from "~/hooks/useClientDark.vue";
import { getVersion } from "@tauri-apps/api/app";
import {
  Check,
  Delete,
  Edit,
  Message,
  Search,
  Star,
  Right,
} from "@element-plus/icons-vue";
import {
  getConfigs,
  openAppDir,
  openLogsDir,
  spctlMasterDisable,
  patch_config,
} from "~/serviece/client/configs";
import { platform } from "@tauri-apps/api/os";
import { ElMessage } from "element-plus";
let { proxy }: any = getCurrentInstance();

const { isDark, toggleTheme } = useClientDark();
const settings = reactive<any>({
  version: "0.0.0",
  configs: { spctlMasterDisable: true, isGithubUseProxy: false },
  platform: "",
});
const jira = reactive<any>({
  visible: false,
  url: "",
  username: "",
  password: "",
});

async function toggleGithubProxy() {
  patch_config({ isGithubUseProxy: !settings.configs.isGithubUseProxy });
  settings.configs = await getConfigs();
}

onMounted(async () => {
  try {
    settings.version = await getVersion();
    settings.configs = await getConfigs();
    settings.platform = await platform();

    console.log(settings);
  } catch (error) {}
});

async function onAddJira() {
  if (!jira.url || !jira.username || !jira.password) {
    ElMessage.error(proxy.$t("common.fail"));
  }

  // var jira1 = new JiraApi({
  //   protocol: "https",
  //   host: "jira.somehost.com",
  //   username: "username",
  //   password: "password",
  //   apiVersion: "2",
  //   strictSSL: true,
  // });
  // console.log(jira1);
}
</script>

<style lang="scss" scoped>
</style>