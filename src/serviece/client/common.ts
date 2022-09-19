import { invoke } from '@tauri-apps/api'

/**
 * 获取基础路径
 * @returns
 */
export const openDir = async (dir: string) => {
  return await invoke('open_dir', { dir })
}

/**
 * 删除
 * @returns
 */
export const removeDir = async (dir: string) => {
  return await invoke('remove_dir', { dir })
}
