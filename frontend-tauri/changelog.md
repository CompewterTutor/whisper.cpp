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
- Added typed backend contracts for health/capability/model/audio commands in `src/contracts.rs`.
- Added backend command shell for system capability and path validation in `src/commands.rs`.
- Added config persistence service in `src/config.rs`.
- Added typed frontend error mapping (`FrontendError` -> `ApiError`) in `src/errors.rs`.
- Added backend module export surface in `src/lib.rs`.
- Added unit tests for config persistence, validators, and error mapping.
- Added MVP transcription run contracts (`RunTranscriptionRequest`, `RunTranscriptionResponse`, `TranscriptionRunStatus`) in `src/contracts.rs`.
- Added `run_transcription_mvp` workflow command in `src/commands.rs`.
- Added `MvpUiState` workflow model with loading/success/error transitions in `src/ui_state.rs`.
- Added tests for MVP run success/failure and workflow state transitions.
- Added MVP interaction helpers for model picker, audio picker, start button eligibility, transcript panel text, and error banner text.
- Added integration test covering select inputs -> run -> render transcript flow.
- Added Tauri command wrappers for health, capability, path validation, and transcription execution in `src/tauri_commands.rs`.
- Added MVP webview binding layer (`MvpAction`, `MvpViewModel`, `dispatch_action`) in `src/mvp_binding.rs`.
- Added tests for command wrapper success paths and reducer-driven interaction flow.
- Added `execution` service module with `CliRunner` abstraction and `ProcessCliRunner` implementation.
- Added whisper-cli argument builder and stdout transcript parser utilities.
- Added mocked-runner integration tests for command execution and transcript parsing.

### Changed

- Updated `src/main.rs` from hello-world to a minimal health function with test coverage.
- Verified Windows workflow supports `make check` for frontend quality gates.
- Refactored crate layout from bin-only modules to `lib + main` to satisfy strict linting with reusable backend modules.
- Updated MVP transcription path to reuse shared transcript parsing logic from `execution` service.

### Fixed

- N/A

## [0.1.0] - 2026-02-12

### Added

- Initial `frontend-tauri` crate scaffold (`tauri` dependency, Rust entrypoint).
