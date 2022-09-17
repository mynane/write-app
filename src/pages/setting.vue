
<template>
  <el-form ref="form" label-width="auto">
    <el-form-item :label="$t('setting.version')">
      <div>{{ version }}</div>
    </el-form-item>
    <el-form-item :label="$t('setting.version')">
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
  </el-form>
</template>

<script setup lang="ts">
import { onBeforeMount, ref } from "vue";
import { changeClientLang } from "~/locals";
import useClientDark from "~/hooks/useClientDark.vue";
import { getVersion } from "@tauri-apps/api/app";
const { isDark, toggleTheme } = useClientDark();
const version = ref<string>("0.0.0");

onBeforeMount(async () => {
  try {
    version.value = await getVersion();
  } catch (error) {}
});
</script>

<style lang="scss" scoped>
</style>