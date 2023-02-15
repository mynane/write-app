<template>
  <el-menu
    :default-active="$route.path"
    class="el-menu-vertical-demo"
    :collapse="true"
    :router="true"
  >
    <el-menu-item index="/">
      <el-icon><HomeFilled /></el-icon>
      <template #title>{{ $t("menu.home") }}</template>
    </el-menu-item>
    <el-menu-item index="/regulex">
      <el-icon><Finished /></el-icon>
      <template #title>{{ $t("menu.regulex") }}</template>
    </el-menu-item>
    <el-menu-item index="/setting">
      <el-icon><setting /></el-icon>
      <template #title>{{ $t("menu.setting") }}</template>
    </el-menu-item>
  </el-menu>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const active = ref<string>("/");

const handleSelect = (key: string, keyPath: string[]) => {
  router.push(key);
  active.value = key;
  console.log(key, keyPath);
};
</script>

