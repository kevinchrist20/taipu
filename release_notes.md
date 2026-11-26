# Taipu v0.4.0

This release introduces **In-App Updates**: Taipu can now check for new versions, download them, and restart straight into the update — without ever leaving the app. It also ships a full refresh of the frontend and Rust/Tauri toolchains.

## Highlights

- **In-App Updates**: A few seconds after launch, Taipu quietly checks GitHub Releases for a newer version. When one is found, a compact dialog slides in from the bottom-right offering the update — no more hunting for download links or running installers by hand.
- **You Stay in Control**: Updating is entirely opt-in. The dialog shows the version transition, the release notes for the new version, and live download progress — then a single "Restart & update" confirmation applies it. Prefer to wait? Hit "Later" and Taipu stays as-is until the next launch.
- **Resilient by Design**: Update checks never interrupt your practice. If you're offline or the check fails, Taipu stays silent and simply offers the update on your next launch, and a failed download can be retried with one click.

## Technical Updates

- Implemented the updater on top of the Tauri updater and process plugins, wired through a new `useUpdater` composable covering the full lifecycle (available → downloading → downloaded → installing) with typed error states, byte-level download tracking, and retry support.
- Built a custom `UpdateDialog.vue` replacing the built-in updater dialog, styled with Taipu's design tokens and light/dark theme support, featuring inline release notes, an animated progress bar, and relaunch confirmation.
- Added a Vitest + jsdom unit test setup (`pnpm test`) with coverage for the updater composable flow.
- Upgraded the entire frontend stack: Pinia 4, Vue Router 5, Vite 8, ESLint 10, TypeScript 5.9, UnoCSS 66.13, and the renamed `@lucide/vue` icon package; removed the remaining Tailwind CSS configuration.
- Upgraded the Rust/Tauri toolchain: Tauri 2.12.7, tauri-build 2.7.6, ts-rs 12, Diesel 2.4, and refreshed Tauri plugins (shell 2.6, process 2.4, store 2.5, updater).
