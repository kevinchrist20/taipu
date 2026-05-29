# Taipu v0.2.0

This is the first stable release line for Taipu.

## Highlights

- New Settings view for updating profile preferences and appearance.
- Language and lesson difficulty are now editable after account creation.
- Theme persistence moved to Tauri Plugin Store.
- Dashboard stats cards now show live metrics (avg WPM, accuracy, best WPM, time typed).
- UX consistency pass across major views for loading, error, and empty states.
- Route/session guard improvements and stronger async failure handling.

## Technical Updates

- Added shared UI helper utilities and reusable state components.
- Refactored frontend view state handling to reduce duplicated logic.
- CI release workflow now uses this `release_notes.md` file directly.
- CI workflow now creates stable releases (`prerelease: false`).
- Fixed release ID wiring for artifact upload in Tauri release builds.

## Notes

- This release supersedes prior prerelease workflows and is intended as the stable baseline for future iterative releases.
