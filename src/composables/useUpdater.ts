import type { DownloadEvent, Update } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import { check } from '@tauri-apps/plugin-updater'
import { computed, ref } from 'vue'
import { extractErrorMessage } from '../utils/error-utils'

export type UpdateStatus
  = | 'idle'
    | 'available'
    | 'downloading'
    | 'downloaded'
    | 'installing'
    | 'error'

const status = ref<UpdateStatus>('idle')
const errorMessage = ref('')
const currentVersion = ref('')
const newVersion = ref('')
const releaseNotes = ref('')
const downloadedBytes = ref(0)
const totalBytes = ref(0)

let update: Update | null = null
let initialized = false

function isTauriRuntime(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
}

function onDownloadEvent(event: DownloadEvent) {
  switch (event.event) {
    case 'Started':
      totalBytes.value = event.data.contentLength ?? 0
      break
    case 'Progress':
      downloadedBytes.value += event.data.chunkLength
      break
    case 'Finished':
      break
  }
}

/**
 * Singleton updater state for the opt-in update dialog.
 *
 * Flow: a background check runs shortly after launch. When an update is found, a
 * dialog in the bottom-left corner offers the download. After the download
 * finishes, the user confirms to install and restart immediately. Dismissing at
 * any point keeps the current version; the offer returns on the next launch.
 */
export function useUpdater() {
  async function initialize(delayMs = 3000) {
    if (initialized || !isTauriRuntime()) {
      return
    }
    initialized = true
    setTimeout(() => {
      void runCheck()
    }, delayMs)
  }

  async function runCheck() {
    try {
      update = await check()

      if (!update) {
        return
      }

      currentVersion.value = update.currentVersion
      newVersion.value = update.version
      releaseNotes.value = update.body ?? ''
      status.value = 'available'
    }
    catch (error) {
      // The startup check is silent: a failed check (e.g. offline) should not
      // interrupt the user with a dialog.
      console.warn('Update check failed:', error)
      status.value = 'idle'
    }
  }

  async function startDownload() {
    if (!update) {
      return
    }

    status.value = 'downloading'
    errorMessage.value = ''
    downloadedBytes.value = 0
    totalBytes.value = 0

    try {
      await update.download(onDownloadEvent)
      status.value = 'downloaded'
    }
    catch (error) {
      errorMessage.value = extractErrorMessage(error, 'Could not download the update.')
      status.value = 'error'
    }
  }

  async function installNow() {
    if (!update) {
      return
    }

    status.value = 'installing'

    try {
      await update.install()
      // On Windows the installer exits the app itself; on macOS/Linux this
      // restarts into the new version.
      await relaunch()
    }
    catch (error) {
      errorMessage.value = extractErrorMessage(error, 'Could not install the update.')
      status.value = 'error'
    }
  }

  function dismiss() {
    void update?.close().catch(() => {})
    update = null
    status.value = 'idle'
    errorMessage.value = ''
  }

  const isVisible = computed(() => status.value !== 'idle')
  const progressPercent = computed(() => {
    if (totalBytes.value <= 0) {
      return 0
    }
    return Math.min(100, Math.round((downloadedBytes.value / totalBytes.value) * 100))
  })

  return {
    status,
    errorMessage,
    currentVersion,
    newVersion,
    releaseNotes,
    downloadedBytes,
    totalBytes,
    progressPercent,
    isVisible,
    initialize,
    startDownload,
    installNow,
    dismiss,
  }
}
