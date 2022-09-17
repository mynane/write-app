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
const { isDark, toggleTheme } = useClientDark();
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
