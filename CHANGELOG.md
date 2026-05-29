# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog and this project follows Semantic Versioning.

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
