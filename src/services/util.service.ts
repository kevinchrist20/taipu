import { relaunch } from '@tauri-apps/plugin-process'
import { check } from '@tauri-apps/plugin-updater'
import { invoke } from '../types/tauri_commands'

export default {
  async exitApp() {
    try {
      await invoke('exit_app')
    }
    catch (error) {
      console.error(error)
    }
  },

  async checkForUpdates() {
    try {
      const update = await check()
      if (update)
        await update.downloadAndInstall()

      await relaunch()
    }
    catch (error) {
      console.error('Error checking for updates:', error)
    }
  },
}
