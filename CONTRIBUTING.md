# Contributing to Taipu

Thanks for contributing to Taipu. This guide keeps contributions simple and consistent.

## Workflow

1. Open or find an issue describing the bug/feature.
2. Create a focused branch from `master`.
3. Make small, reviewable commits.
4. Open a pull request with context and validation notes.

## Branch Naming

Use one of these patterns:

- `feat/<short-description>`
- `fix/<short-description>`
- `chore/<short-description>`
- `docs/<short-description>`

Examples:

- `feat/settings-theme-store`
- `fix/stats-card-loading-state`

## Commit Messages

Keep commit messages clear and scoped. A simple prefix style is enough:

- `feat: add profile update validation`
- `fix: handle category fetch partial failures`
- `docs: update installation bypass notes`

## Local Validation

Before opening a PR, run:

```bash
pnpm build
```

If your change touches Rust/Tauri backend code, also run:

```bash
cargo build --manifest-path src-tauri/Cargo.toml
```

## Pull Request Checklist

- [ ] The change is focused and tied to an issue (or clearly explained).
- [ ] `pnpm build` passes locally.
- [ ] `cargo build --manifest-path src-tauri/Cargo.toml` passes for backend changes.
- [ ] UI changes include screenshots or short notes.
- [ ] Relevant docs are updated.

## Release-Related Docs

For user-facing changes or release-impacting fixes, update as needed:

- `CHANGELOG.md` for release history.
- `release_notes.md` for the next release body used by CI.
- `MVP_COMPLETION_SUMMARY.md` for milestone/progress tracking.

## PR Description Template

Use this format in your PR body:

```md
## Summary
- What changed and why

## Validation
- Commands run and key results

## Notes
- Risks, follow-ups, or rollout details
```

## Code of Conduct

Be respectful, constructive, and collaborative in all discussions and reviews.
