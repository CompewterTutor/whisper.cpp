# frontend-tauri Versioning Guide

Date: 2026-02-12

## Semantic Versioning policy

`frontend-tauri` follows `MAJOR.MINOR.PATCH`.

- `MAJOR`:
  - Breaking API or behavior changes in frontend command contracts/workflows.
  - User-visible flow breaks requiring migration.
- `MINOR`:
  - New backward-compatible features.
  - New commands/screens/states without breaking existing usage.
- `PATCH`:
  - Bug fixes, reliability improvements, refactors with no intended behavior break.

## Commit type guidance

Use Conventional Commits and map to release intent:

- `feat(...)` usually drives `MINOR`.
- `fix(...)` usually drives `PATCH`.
- `chore(...)`, `docs(...)`, `test(...)` usually do not force version bumps unless bundled into a release.
- Breaking changes should include explicit indicator and drive `MAJOR`.

## Standard version bump flow

1. Choose bump type (`major`/`minor`/`patch`).
2. Update version in `frontend-tauri/Cargo.toml`.
3. Update `frontend-tauri/changelog.md`:
   - create `## [x.y.z] - YYYY-MM-DD`
   - move relevant entries from `Unreleased`.
4. Run validation commands:
   - `make check`
   - optional: `cargo test`
5. Ask user approval before commit.

## Changelog verification rules

Before committing release prep:

- Ensure every user-visible change is captured in changelog.
- Ensure duplicate bullets are removed or merged.
- Ensure technical notes remain concise and grouped by Added/Changed/Fixed.
- Ensure release section date and version match `Cargo.toml`.
