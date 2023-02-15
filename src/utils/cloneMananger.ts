import { Command } from '@tauri-apps/api/shell'

export type TCloneManagerOptions = {
  maxLength: number
}

class CloneManager {
  public maxLength: number
  public listeners = new Map<string, Function>()
  public loadings = new Map<string, boolean>()

  constructor(options: TCloneManagerOptions) {
    this.maxLength = options.maxLength ?? 10
  }

  public async clone(uri: string, path: string) {
    if (this.listeners.has(uri)) {
      this.listeners.get(uri)!(true)
      this.loadings.set(uri, true)
    }
    const command = new Command(
      'git',
      [
        'clone',
        uri,
        // `${uri.startsWith('https://github.com') ? 'https://github.91chi.fun//' : ''}${uri}`,
      ],
      { cwd: path }
    )
    command.on('close', async (data) => {
      if (data.code) {
        if (this.listeners.has(uri)) {
          this.listeners.get(uri)!(false, true)
        }
      } else {
        if (this.listeners.has(uri)) {
          this.listeners.get(uri)!(false, false)
        }
      }
      this.loadings.delete(uri)
    })

    command.on('error', (error) => console.error(`command error: "${error}"`))
    command.stdout.on('data', (line) => console.log(`command stdout: "${line}"`))
    command.stderr.on('data', (line) => console.log(`command stderr: "${line}"`))

    const child = await command.spawn()

    console.log('pid:', child.pid)
  }

  public addEventListener(key: string, callback: Function) {
    if (this.loadings.has(key)) {
      callback(true)
    }
    this.listeners.set(key, callback)
  }

  public removeEventListener(key: string) {
    this.listeners.delete(key)
  }
}

export const cloneManager = new CloneManager({ maxLength: 10 })
