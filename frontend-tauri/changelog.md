# Changelog

All notable changes to `frontend-tauri` will be documented in this file.

The format is based on Keep a Changelog, and the project follows Semantic Versioning.

## [Unreleased]

### Added

- Added recoverable error hints panel in UI that displays actionable recovery suggestions when errors occur.
- Added `hint` field to `ApiError` struct with automatic derivation from error code.
- Added `ApiError::new` constructor that auto-populates hints for known error codes.
- Added error hint mapping for common failures: missing paths, file not found, invalid extensions, I/O errors, execution timeouts, cancellations, and shortcut registration failures.
- Added UI styles and JavaScript logic to display error hints alongside error messages.
- Added tests for error hint coverage on all common error types.
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
- Added execution timeout and cancellation controls via `RunTranscriptionOptions` and `CliRunOptions`.
- Added timeout-aware process execution path and cancellation short-circuit behavior.
- Added timeout/cancel-aware Tauri command endpoint (`run_transcription_with_options_command`).
- Added optional feature-gated real whisper-cli smoke test (`real-whisper-smoke`) with env-configured paths.
- Added Tauri command registration wiring in `main.rs` for implemented command handlers.
- Added release-readiness documentation: `docs/release-checklist.md` and `docs/versioning.md`.
- Added `make smoke-real` and `make verify-release` automation targets for repeatable release checks.

### Changed

- Updated `src/main.rs` from hello-world to a minimal health function with test coverage.
- Verified Windows workflow supports `make check` for frontend quality gates.
- Refactored crate layout from bin-only modules to `lib + main` to satisfy strict linting with reusable backend modules.
- Updated MVP transcription path to reuse shared transcript parsing logic from `execution` service.
- Added Cargo feature flags to isolate optional real smoke test from default CI/dev runs.
- Disabled binary target tests via `[[bin]] test = false` to avoid platform-specific Tauri bin harness runtime issues while keeping `cargo test` for standard quality gates.
- Documented changelog verification rules and SemVer release flow for frontend releases.
- Aligned release checklist and README with automated release verification commands.

### Fixed

- Verified real Windows execution path using `whisper-cli.exe` and validated frontend feature-gated smoke test with local model/audio inputs.

## [0.1.0] - 2026-02-12

### Added

- Initial `frontend-tauri` crate scaffold (`tauri` dependency, Rust entrypoint).
