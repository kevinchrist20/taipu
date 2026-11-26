import type { Update } from '@tauri-apps/plugin-updater'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

const checkMock = vi.fn()
const relaunchMock = vi.fn()

vi.mock('@tauri-apps/plugin-updater', () => ({
  check: (...args: unknown[]) => checkMock(...args),
}))

vi.mock('@tauri-apps/plugin-process', () => ({
  relaunch: (...args: unknown[]) => relaunchMock(...args),
}))

type UseUpdaterModule = typeof import('./useUpdater')

async function loadComposable(): Promise<UseUpdaterModule['useUpdater']> {
  const module = await import('./useUpdater')
  return module.useUpdater
}

function fakeUpdate(overrides: Partial<Record<string, unknown>> = {}): Update {
  const update = {
    currentVersion: '0.3.0',
    version: '0.3.1',
    body: 'Fixes and improvements',
    date: undefined,
    rawJson: {},
    download: vi.fn(async (onEvent?: (event: unknown) => void) => {
      onEvent?.({
        event: 'Started',
        data: { contentLength: 100 },
      })
      onEvent?.({
        event: 'Progress',
        data: { chunkLength: 50 },
      })
      onEvent?.({ event: 'Finished' })
    }),
    install: vi.fn(async () => {}),
    close: vi.fn(async () => {}),
    ...overrides,
  }
  return update as unknown as Update
}

describe('useUpdater', () => {
  beforeEach(() => {
    vi.resetModules()
    vi.useFakeTimers()
    vi.clearAllMocks()
    Object.defineProperty(window, '__TAURI_INTERNALS__', {
      value: {},
      configurable: true,
    })
  })

  afterEach(() => {
    vi.useRealTimers()
    Reflect.deleteProperty(window, '__TAURI_INTERNALS__')
  })

  it('does nothing outside the Tauri runtime', async () => {
    Reflect.deleteProperty(window, '__TAURI_INTERNALS__')
    const useUpdater = await loadComposable()

    const updater = useUpdater()
    await updater.initialize(0)

    expect(checkMock).not.toHaveBeenCalled()
    expect(updater.isVisible.value).toBe(false)
  })

  it('finds an update after the startup delay', async () => {
    const update = fakeUpdate()
    checkMock.mockResolvedValue(update)
    const useUpdater = await loadComposable()

    const updater = useUpdater()
    await updater.initialize(3000)
    expect(updater.status.value).toBe('idle')

    await vi.advanceTimersByTimeAsync(3000)
    expect(checkMock).toHaveBeenCalledTimes(1)
    expect(updater.status.value).toBe('available')
    expect(updater.newVersion.value).toBe('0.3.1')
    expect(updater.currentVersion.value).toBe('0.3.0')
    expect(updater.releaseNotes.value).toBe('Fixes and improvements')
    expect(updater.isVisible.value).toBe(true)
  })

  it('stays idle when no update is available', async () => {
    checkMock.mockResolvedValue(null)
    const useUpdater = await loadComposable()

    const updater = useUpdater()
    await updater.initialize(3000)
    await vi.advanceTimersByTimeAsync(3000)

    expect(updater.status.value).toBe('idle')
    expect(updater.isVisible.value).toBe(false)
  })

  it('stays silent when the startup check fails', async () => {
    checkMock.mockRejectedValue(new Error('offline'))
    const useUpdater = await loadComposable()

    const updater = useUpdater()
    await updater.initialize(3000)
    await vi.advanceTimersByTimeAsync(3000)

    expect(updater.status.value).toBe('idle')
  })

  it('downloads on request and reports progress', async () => {
    const update = fakeUpdate()
    checkMock.mockResolvedValue(update)
    const useUpdater = await loadComposable()

    const updater = useUpdater()
    await updater.initialize(3000)
    await vi.advanceTimersByTimeAsync(3000)

    await updater.startDownload()
    expect(update.download).toHaveBeenCalledTimes(1)
    expect(updater.status.value).toBe('downloaded')
    expect(updater.progressPercent.value).toBe(50)
  })

  it('installs and relaunches on confirmation', async () => {
    const update = fakeUpdate()
    checkMock.mockResolvedValue(update)
    const useUpdater = await loadComposable()

    const updater = useUpdater()
    await updater.initialize(3000)
    await vi.advanceTimersByTimeAsync(3000)

    await updater.installNow()
    expect(update.install).toHaveBeenCalledTimes(1)
    expect(relaunchMock).toHaveBeenCalledTimes(1)
    expect(updater.status.value).toBe('installing')
  })

  it('surfaces download errors with a retry', async () => {
    const update = fakeUpdate({
      download: vi.fn(async () => {
        throw new Error('network down')
      }),
    })
    checkMock.mockResolvedValue(update)
    const useUpdater = await loadComposable()

    const updater = useUpdater()
    await updater.initialize(3000)
    await vi.advanceTimersByTimeAsync(3000)

    await updater.startDownload()
    expect(updater.status.value).toBe('error')
    expect(updater.errorMessage.value).toBe('network down')
    expect(updater.isVisible.value).toBe(true)
  })

  it('dismiss releases the update resource', async () => {
    const update = fakeUpdate()
    checkMock.mockResolvedValue(update)
    const useUpdater = await loadComposable()

    const updater = useUpdater()
    await updater.initialize(3000)
    await vi.advanceTimersByTimeAsync(3000)

    updater.dismiss()
    expect(update.close).toHaveBeenCalledTimes(1)
    expect(updater.status.value).toBe('idle')
    expect(updater.isVisible.value).toBe(false)
  })

  it('only checks once per session', async () => {
    checkMock.mockResolvedValue(null)
    const useUpdater = await loadComposable()

    const updater = useUpdater()
    await updater.initialize(3000)
    await updater.initialize(3000)
    await vi.advanceTimersByTimeAsync(3000)

    expect(checkMock).toHaveBeenCalledTimes(1)
  })
})
