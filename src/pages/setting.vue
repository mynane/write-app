
<template>
  <el-form ref="form" label-width="auto">
    <el-form-item :label="$t('setting.version')">
      <div>v{{ settings.version }}</div>
    </el-form-item>
    <el-form-item :label="$t('setting.themeMode')">
      <el-switch :model-value="isDark" @change="toggleTheme()" />
    </el-form-item>
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
    <el-form-item
      :label="$t('setting.spctlMasterDisable')"
      v-if="settings.platform === 'darwin'"
    >
      <el-button @click="spctlMasterDisable" type="primary">{{
        $t("setting.getPermission")
      }}</el-button>
    </el-form-item>
  </el-form>
</template>

<script setup lang="ts">
import { onBeforeMount, reactive, ref } from "vue";
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
} from "~/serviece/client/configs";
import { platform } from "@tauri-apps/api/os";

const { isDark, toggleTheme } = useClientDark();
const settings = reactive<any>({
  version: "0.0.0",
  configs: { spctlMasterDisable: true },
  platform: "",
});

onBeforeMount(async () => {
  try {
    settings.version = await getVersion();
    settings.configs = await getConfigs();
    settings.platform = await platform();

    console.log(settings);
  } catch (error) {}
});
</script>

<style lang="scss" scoped>
</style>