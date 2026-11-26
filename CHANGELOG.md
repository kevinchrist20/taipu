# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog and this project follows Semantic Versioning.

## [0.4.0] - 2026-11-26

### Added

- In-app update experience: a silent background check runs shortly after launch and, when a new release is found, a dismissible update dialog offers download, install, and restart without leaving the app.
- Custom update dialog (`UpdateDialog.vue`) showing the version transition, inline release notes, live download progress with percentage, and a final "Restart & update" confirmation step.
- `useUpdater` composable managing the full update lifecycle (idle, available, downloading, downloaded, installing, error) with retry support and silent failure when a check fails (e.g. while offline).
- Tauri process plugin registration and `process:default` capability to enable relaunching into the updated version after installation.
- Unit test setup with Vitest and jsdom, including coverage for the updater composable flow, exposed via a `test` script.

### Changed

- Replaced the built-in updater dialog with Taipu's own opt-in update dialog; dismissing keeps the current version and the offer returns on the next launch.
- Update checks no longer interrupt the user: startup checks run quietly and never block the app when offline or when the check fails.
- Updated all frontend dependencies to latest versions, including Pinia 4, Vue Router 5, Vite 8, ESLint 10, TypeScript 5.9, UnoCSS 66.13, and the renamed `@lucide/vue` icon package.
- Updated the Rust/Tauri toolchain: Tauri 2.12.7, tauri-build 2.7.6, ts-rs 12, Diesel 2.4, dirs 7, and refreshed Tauri plugins (shell 2.6, process 2.4, store 2.5, updater).
- Removed the remaining Tailwind CSS configuration in favor of UnoCSS-only styling.

---

## [0.3.0] - 2026-09-02

### Added

- Japanese Bonsai-inspired Growth Tree progression visual (`GrowthTree.vue`) with 5 evolution stages (Sprout, Sapling, Branching Bonsai, Verdant Canopy, Full Bloom).
- Dynamic foliage density reacting to accuracy precision and floral blossoms reacting to practice streaks.
- Interactive Speed & Accuracy Trend Chart (`StatsTrendChart.vue`) with 7D, 30D, and All-Time ranges and dual metric curves.
- Daily consistency streak counter and 28-day (4-week) activity punchcard (`StreakCard.vue`).
- Backend models and SQLite queries for time-series aggregation, streak tracking, and growth milestone computation.
- TypeScript bindings for `DailyActivityStat`, `StreakInfo`, and `TreeStageInfo`.

### Changed

- Modernized Stats view with a high-impact hero section containing the growth tree, streak status, and live overview metrics.

---

## [0.2.0] - 2026-05-29

### Added

- Dedicated Settings view for managing profile preferences and appearance.
- Editable user preferences after account creation (language and lesson difficulty).
- Reusable UX state components for loading, error, and empty states.
- Shared UI formatter and error utility helpers on the Vue frontend.
- Route guard protection for authenticated app routes.

### Changed

- Theme persistence migrated to Tauri Plugin Store.
- Dashboard stat cards now use live statistics data instead of placeholders.
- Release workflow now publishes stable releases using manual `release_notes.md`.
- Release workflow wiring fixed for release ID output consumption in build upload.

### Improved

- Loading/error/empty-state consistency across major views.
- Async handling resilience for view fetch flows using partial-failure-safe patterns.
- Category and stats pages now surface clearer failure states and retry options.

### Fixed

- Silent failure/blank state scenarios in key views when async loading fails.
- Release workflow mismatch between create-release outputs and tauri-action upload input.

---

## [0.1.0] - 2026-05-25

### Added

- Initial Taipu MVP foundation with lessons, completions, local storage, and Tauri app shell.
