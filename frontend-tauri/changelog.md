# Changelog

All notable changes to `frontend-tauri` will be documented in this file.

The format is based on Keep a Changelog, and the project follows Semantic Versioning.

## [Unreleased]

### Added

- P6: Added audio capture module with `cpal` for cross-platform microphone input.
- P6: Added `AudioCaptureSession` for device selection and capture lifecycle management.
- P6: Added WAV encoding with `hound` for whisper-cli compatibility.
- P6: Added audio resampling to convert any sample rate to 16kHz.
- P6: Added Tauri commands: `list_audio_devices_command`, `select_audio_device_command`, `get_capture_state_command`, `start_capture_command`, `stop_capture_command`, `get_current_audio_device_command`.
- P6: Added `AudioDeviceInfo` and `CaptureState` contracts for frontend integration.
- P4: Added comprehensive settings page with theme selection (system/light/dark).
- P4: Added default output directory and default model directory settings with browse buttons.
- P4: Added execution defaults settings: default threads and default timeout.
- P4: Added diagnostics logging toggle setting.
- P4: Added keyboard accessibility: focus indicators on all controls, Ctrl+R shortcut for transcription, accesskey attributes.
- P4: Added `capabilities/default.json` with explicit Tauri v2 permissions for security hardening.
- P4: Added new Tauri commands: `set_theme_command`, `set_default_output_dir_command`, `set_default_model_dir_command`, `set_diagnostics_enabled_command`, `set_default_threads_command`, `set_default_timeout_command`, `pick_directory_command`.
- P4: Added new request contracts: `ThemeSettingRequest`, `PathSettingRequest`, `BoolSettingRequest`, `U16SettingRequest`, `U64SettingRequest`.
- P4: Extended `AppSettingsResponse` to include all new settings fields.
- Added clipboard commands: `copy_to_clipboard_command`, `get_clipboard_text_command` for PTT output routing.
- Added PTT output routing contracts: `PttSessionState`, `PttMode`, `PttOutputRouting`, `PttSession`.
- Added PTT routing Tauri commands: `get_ptt_routing_command`, `set_ptt_routing_command`.
- Added PTT Output Routing settings card with toggles for clipboard, file, and type emulation.
- Added Copy button to transcript section for quick clipboard export.
- Added `tauri-plugin-clipboard-manager` dependency for clipboard integration.
- Added `ptt_routing` field to `AppConfig` for persistent output routing preferences.
- Added batch queue view with multi-file add/remove/reorder functionality.
- Added per-item status display (Pending, Running, Success, Error) for queue items.
- Added run history list with timestamp, success status, and rerun action.
- Added `QueueItem` and `HistoryItem` contracts for batch processing.
- Added queue and history management to `ConfigStore` with max 50 history items.
- Added Tauri commands: `add_to_queue_command`, `remove_from_queue_command`, `reorder_queue_command`, `get_queue_command`, `clear_completed_queue_command`, `update_queue_item_status_command`, `add_to_history_command`, `get_history_command`, `clear_history_command`.
- Added shortcut conflict detection: visual warning when same shortcut is assigned to multiple actions.
- Added `is_shortcut_registered_command` to check if a shortcut is already registered.
- Added real-time conflict feedback with orange highlight on conflicting shortcut fields.
- Added advanced transcription options: task (transcribe/translate), language, threads, beam size, best-of, temperature.
- Added preset management: save, load, delete, and set default presets.
- Added collapsible advanced options drawer in UI with preset dropdown.
- Added `TranscriptionAdvancedOptions` contract for whisper-cli advanced flags.
- Added `TranscriptionPreset` struct and preset management to `ConfigStore`.
- Added Tauri commands: `list_presets_command`, `get_preset_command`, `save_preset_command`, `delete_preset_command`, `set_default_preset_command`, `get_default_preset_command`.
- Added backend persistence for start-in-background and launch-on-login settings.
- Added `tauri-plugin-autostart` for OS-level launch-on-login registration.
- Added `get_app_settings_command`, `set_start_in_background_command`, and `set_launch_on_login_command` Tauri commands.
- Added startup behavior that hides window when start-in-background is enabled.
- Added `dirs` crate dependency for cross-platform config directory resolution.
- Added clear section headers for UI layout: Input Files, Controls, Transcript, Settings.
- Added visual divider between main workflow and settings sections.
- Combined model and audio pickers into a unified Input Files section for better grouping.
- Added recoverable error hints panel in UI that displays actionable recovery suggestions when errors occur.

### Changed

- Extended `WhisperCliRequest` to support advanced transcription options.
- Updated `build_whisper_cli_args` to include language, task, threads, beam size, best-of, and temperature flags.
- Settings toggles now persist via backend config instead of localStorage.
- Updated UI hint text to reflect implemented background mode functionality.
- Refined UI layout into clear sections with headers: inputs, controls, transcript, and settings.
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

- Refined UI layout into clear sections with headers: inputs, controls, transcript, and settings.
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
