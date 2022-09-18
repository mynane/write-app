import { invoke } from '@tauri-apps/api'

/**
 * 获取基础路径
 * @returns
 */
export const getBasicDir = async () => {
  return await invoke('get_basic_dir')
}

/**
 * 获取基础路径
 * @returns
 */
export const setBasicDir = async (basicDir: string) => {
  return await invoke('set_basic_dir', { basicDir })
}

/**
 * 获取基础路径
 * @returns
 */
export const createRep = async (item: string) => {
  return await invoke('create_rep', { item })
}
