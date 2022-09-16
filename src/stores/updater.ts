import { getVersion } from '@tauri-apps/api/app'
import { relaunch } from '@tauri-apps/api/process'
import { checkUpdate, installUpdate, UpdateManifest, UpdateStatus } from '@tauri-apps/api/updater'
import { defineStore } from 'pinia'

export interface IState {
  loading: boolean
  visible: boolean
  updateInfo?: UpdateManifest
  status: UpdateStatus
}

export const useUpdaterStore = defineStore('updater', {
  state: (): IState => {
    return {
      loading: false,
      visible: false,
      updateInfo: undefined,
      status: 'PENDING',
    }
  },
  actions: {
    async check() {
      this.loading = true
      const update = await checkUpdate()

      if (this.visible && update.manifest?.version === this.updateInfo?.version) {
        this.loading = false
        return
      }

      if (update.shouldUpdate) {
        this.updateInfo = update.manifest
        this.visible = true
      }
      this.loading = false
    },
    async checkAndUpdate() {
      this.loading = true
      const update = await checkUpdate()

      if (this.visible && update.manifest?.version === this.updateInfo?.version) {
        this.loading = false
        return
      }

      if (update.shouldUpdate) {
        await installUpdate()
        this.updateInfo = update.manifest
        this.visible = true
      }
      this.loading = false
    },
    async update() {
      this.loading = true
      if (this.status !== 'DONE') {
        await installUpdate()
      } else {
        await relaunch()
      }
      this.loading = false
    },
  },
})
