<script lang="ts" setup>
import useClientDark from "~/hooks/useClientDark.vue";
import { changeClientLang } from "~/locals";

const { isDark, toggleTheme } = useClientDark();

function changeLang(lang: any) {
  changeClientLang(lang);
}
</script>

<template>
  <el-menu class="el-menu-demo" mode="horizontal">
    <el-menu-item index="1">Element Plus</el-menu-item>
    <el-menu-item index="3" disabled>Info</el-menu-item>
    <el-menu-item index="4">Orders</el-menu-item>
    <el-menu-item index="5"
      ><el-switch :model-value="isDark" @change="toggleTheme()"
    /></el-menu-item>
    <el-menu-item index="6">
      <el-select
        :model-value="$i18n.locale"
        class="m-2"
        placeholder="Select"
        @change="changeLang"
      >
        <el-option
          v-for="locale in $i18n.availableLocales"
          :key="`locale-${locale}`"
          :value="locale"
          :label="$t(`lang.${locale}`)"
        >
          {{ $t(`lang.${locale}`) }}
        </el-option>
      </el-select>
    </el-menu-item>
  </el-menu>
</template>
