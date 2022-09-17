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
