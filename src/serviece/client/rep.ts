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

/**
 * 获取基础路径
 * @returns
 */
export const patchRep = async (uri: string, item: any) => {
  return await invoke('patch_rep', { uri, item })
}

/**
 * 删除rep
 * @returns
 */
export const removeRep = async (uri: string) => {
  return await invoke('remove_rep', { uri })
}
