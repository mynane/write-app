<template>
  <el-config-provider namespace="ep">
    <BaseHeader />
    <div style="display: flex">
      <BaseSide />
      <router-view />
    </div>

    <Updater />
  </el-config-provider>
</template>

<script setup lang="ts">
import Updater from "./components/Updater/index.vue";
</script>

<style>
#app {
  text-align: center;
  color: var(--ep-text-color-primary);
}

.element-plus-logo {
  width: 50%;
}
</style>
