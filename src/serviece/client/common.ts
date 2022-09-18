import { invoke } from '@tauri-apps/api'

/**
 * 获取基础路径
 * @returns
 */
export const openDir = async (dir: string) => {
  console.log('🚀 ~ file: common.ts ~ line 8 ~ openDir ~ dir', dir)
  return await invoke('open_dir', { dir })
}
