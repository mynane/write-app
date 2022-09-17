<script lang="ts">
import Vue, { onBeforeMount, onMounted, ref, watch } from "vue";
import { changeTheme, getConfigs } from "~/serviece/client/configs";

export default () => {
  const isDark = ref<boolean>(false);

  onBeforeMount(async () => {
    let configs: any = null;
    try {
      configs = await getConfigs();
    } catch (error) {
      configs = {
        theme: localStorage.getItem("client_theme") ?? "",
      };
    }
    isDark.value = configs.theme === "dark";
  });

  const changeClientTheme = async (ntheme: any) => {
    let configs: any = null;
    try {
      configs = await changeTheme(ntheme);
    } catch (error) {
      localStorage.setItem("client_theme", ntheme);
      configs = {
        theme: ntheme,
      };
    }

    console.log(ntheme);
    isDark.value = configs.theme === "dark";
  };

  const toggleTheme = async () => {
    let ntheme = isDark.value ? "" : "dark";
    await changeClientTheme(ntheme);
  };

  watch(
    isDark,
    (newVal, oldVal) => {
      const root = document.querySelector("html") as HTMLHtmlElement;
      root.className = newVal ? "dark" : "";
    },
    { immediate: true }
  );

  return {
    isDark,
    changeClientTheme,
    toggleTheme,
  };
};
</script>