# Taipu MVP Completion Summary

Version: 0.3.0
Status: Core MVP loop implemented; content depth and quality hardening remain
Last Updated: 29 May 2026

---

## Current Snapshot

Taipu now supports a full local MVP training loop:

- Multi-profile user selection and account creation
- Beginner, Intermediate, and Advanced lesson tracks
- Category progression with lesson/test gating
- Real-time typing metrics, grading, pause/retry flow
- Completion history persisted in SQLite
- Stats dashboard plus live dashboard summary cards
- Editable in-app settings for language, lesson difficulty, and theme
- Theme persistence via Tauri Plugin Store

The project has moved beyond the earlier beginner-only / placeholder-dashboard phase.

---

## Implemented Features

### Typing and Lesson Experience

- Real-time character-by-character typing validation
- WPM, accuracy, elapsed time, and completion percentage tracking
- Completion grading (S/A/B/C/D)
- Pause, resume, exit confirmation, retry lesson flows
- Virtual keyboard with next-key guidance and physical-key feedback

### Curriculum and Progression

- Seeded curriculum across all difficulty levels in migration data
- Category tests tied to prerequisite lesson completion
- Category availability and progress computed from user completions

### User and Navigation

- Home profile picker and create-profile flow
- Header shell navigation for Dashboard, Stats, and Settings
- Reactive session state for active user and current theme

### Settings and Preferences

- Dedicated Settings view implemented
- Editable user preferences after account creation:
  - Language
  - Lesson difficulty
- Backend command implemented for updates:
  - `update_user_preferences`
- Theme is persisted with Tauri Plugin Store (not ini file)

### Statistics and Analytics

- Full Stats view backed by Rust/Tauri SQL queries
- Dashboard top cards now wired to live stats (no placeholder zeros)
- Metrics currently surfaced include:
  - Average WPM
  - Average accuracy
  - Best WPM
  - Total time typed
  - Grade distribution
  - Category progress
  - Recent activity

### Backend and Data Layer

- Diesel + SQLite with migration-based schema and seed data
- Tauri commands for users, lessons, completions, and statistics
- Completions model supports multi-attempt historical records

---

## Curriculum Coverage (Seeded)

### Beginner

- 7 categories
- 3 lessons + 1 test each
- Total seeded items: 28

### Intermediate

- 2 categories (`common-words`, `numbers-row`)
- 3 lessons + 1 test each
- Total seeded items: 8

### Advanced

- 1 category (`speed-drills`)
- 3 lessons + 1 test
- Total seeded items: 4

### Total Seeded Lesson Items

- 40

Note: Multi-difficulty functionality is implemented, but intermediate/advanced breadth is still limited for a broader public launch.

---

## MVP Gaps Remaining

### High Priority

1. Curriculum expansion for depth
    - Intermediate and advanced tracks need more categories and variety
    - Add broader drills for numbers, symbols, capitalization, mixed text

2. Quality and reliability coverage

    - Automated tests remain light (unit + integration + E2E needed)
    - Core user flows should be covered in CI

3. UX consistency and polish

    - Standardize loading/error empty states across all views
    - Tighten edge-case handling during async failures and route transitions

### Medium Priority

1. Keyboard layout variants

    - Keyboard rendering is still effectively a single layout
    - User-selectable layout behavior is not fully implemented

2. Accessibility hardening

    - Improve keyboard-only navigation and ARIA semantics
    - Add stronger focus management and screen-reader checks

3. Distribution channels (package managers/stores)

    - Add Homebrew Cask publishing path for macOS
    - Add Windows package manager path (Winget and/or Scoop)
    - Add Linux distribution channel path (Flatpak first, then optional Snap/AUR)
    - Keep GitHub Releases as source of truth for binaries, checksums, and notes
    - Document ownership/maintenance responsibilities for each channel

---

## Suggested Next Delivery Slice

1. Expand intermediate and advanced lesson categories
2. Add baseline automated tests for critical flows:

    - Create/select user
    - Complete lesson and record completion
    - Save settings and reload preferences
    - Validate dashboard/stat consistency

3. Improve shared async state UX patterns (loading/error/empty)
4. Begin accessibility audit pass on major screens
5. Start package manager rollout (phased):

- Phase A: Homebrew Cask (macOS)
- Phase B: Winget/Scoop (Windows)
- Phase C: Flatpak (Linux)

---

## Overall Progress Estimate

- Core product loop: complete
- Settings/profile editing: complete
- Dashboard stats wiring: complete
- Theme persistence: complete (Plugin Store)
- Content depth and production hardening: in progress

Estimated overall MVP completion: ~88-92%
