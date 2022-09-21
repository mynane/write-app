import { invoke } from '@tauri-apps/api'

/**
 * 获取配置
 * @returns
 */
export const getConfigs = async () => {
  return await invoke('get_configs')
}

/**
 * 改变主题色
 * @param theme '' | 'dark'
 * @returns
 */
export const changeTheme = async (theme: '' | 'dark') => {
  return await invoke('change_theme', { theme })
}

/**
 * 改变语言
 * @param theme 'zh' | 'en'
 * @returns
 */
export const changeLang = async (lang: 'zh' | 'en') => {
  return await invoke('change_lang', { lang })
}

/**
 * 打开应用目录
 * @returns
 */
export const openAppDir = async () => {
  return await invoke('open_app_dir')
}

/**
 * 打开日志目录
 * @returns
 */
export const openLogsDir = async () => {
  return await invoke('open_logs_dir')
}

/**
 * mac 信任所有来源（自动更新需要）
 * @returns
 */
export const spctlMasterDisable = async () => {
  return await invoke('spctl_master_disable')
}

/**
 * 批量修改config
 * @returns
 */
export const patch_config = async (item: any) => {
  return await invoke('patch_config', { item })
}
