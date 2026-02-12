# Changelog

All notable changes to `frontend-tauri` will be documented in this file.

The format is based on Keep a Changelog, and the project follows Semantic Versioning.

## [Unreleased]

### Added

- Initialized planning and progress-tracking docs for frontend implementation.
- Added `frontend-tauri/README.md` with setup, quality gates, and workflow constraints.
- Added `frontend-tauri/Makefile` task aliases for `fmt`, `lint`, `test`, `check`, and `dev`.
- Added frontend-only CI workflow at `.github/workflows/frontend-tauri-ci.yml`.
- Added smoke unit test for backend health in `src/main.rs`.

### Changed

- Updated `src/main.rs` from hello-world to a minimal health function with test coverage.
- Verified Windows workflow supports `make check` for frontend quality gates.

### Fixed

- N/A

## [0.1.0] - 2026-02-12

### Added

- Initial `frontend-tauri` crate scaffold (`tauri` dependency, Rust entrypoint).
