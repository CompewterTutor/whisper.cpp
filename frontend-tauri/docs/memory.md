# frontend-tauri Memory

## 2026-02-24

### What was learned

- P6 audio capture uses `cpal` crate for cross-platform microphone input.
- `hound` crate handles WAV encoding for whisper-cli compatibility.
- Audio samples are captured as f32, converted to mono, and can be resampled to 16kHz.
- `AudioCaptureSession` manages device selection, capture lifecycle, and sample buffering.
- Capture state machine: Idle -> Listening -> (Transcribing|Error) -> Idle.
- Tauri state with `Mutex<AudioCaptureSession>` allows thread-safe access from commands.
- `tauri-plugin-dialog` is preferred over `rfd` for file dialogs in Tauri apps - better integration.
- `rfd` (rust-file-dialog) can cause issues with Tauri's webview on Windows.
- JavaScript in Tauri must wait for `window.__TAURI__` to be available before calling invoke.
- Duplicate variable declarations in inline scripts cause silent JS failures.
- Error hints can be derived from error codes and serialized to frontend for display.
- `ApiError` struct now includes optional `hint` field that is automatically populated from error code.
- UI layout benefits from clear section headers for progressive disclosure (Input Files, Controls, Transcript, Settings).
- Grouping related inputs (model + audio) into a single section improves visual hierarchy.
- `tauri-plugin-autostart` uses `autolaunch()` method with `enable()`/`disable()` (not `set_enabled()`).
- Settings that affect app behavior at startup (start-in-background, launch-on-login) must be persisted via backend, not localStorage.
- Tauri state management uses `State<'_, T>` in command signatures and `app.manage(store)` in setup.
- Temperature is stored as integer * 100 in `TranscriptionAdvancedOptions` to avoid f32 comparison issues in Eq.
- Collapsible drawer UI pattern uses CSS classes `.drawer-toggle`, `.drawer-content`, and `.open` for visibility.
- Presets are stored in a `HashMap<String, TranscriptionPreset>` within `AppConfig`.
- `tauri-plugin-global-shortcut::is_registered()` returns `bool` directly, not `Result<bool>`.
- Shortcut conflict detection can be done purely client-side by comparing input values.
- Queue items use `QueueItemStatus` enum: Pending, Running, Success, Error.
- History is limited to 50 items and stored in `AppConfig.history` with newest first.
- Queue processing is sequential (not parallel) to avoid resource contention.

### Constraints to preserve

- Keep implementation incremental and test-first.
- Keep steps small and verifiable.
- Always ask user before committing.
- Use proper Conventional Commit messages.
- Follow Semantic Versioning for frontend releases.
- Maintain `frontend-tauri/changelog.md` as source of release notes.

### Current progress

- **BLOCKING ISSUE**: File dialogs still not working - "Tauri invoke API not available" error persists
	- Attempted fixes:
		1. Added `initApp()` to wait for `window.__TAURI__` before initialization
		2. Fixed duplicate `audioHintEl` variable declaration causing JS syntax error
		3. Switched from `rfd` to `tauri-plugin-dialog` for better Tauri integration
	- Issue persists - needs further investigation in new chat session
- In progress: P6 push-to-talk audio capture pipeline:
	- Added `cpal` dependency for cross-platform audio capture
	- Added `hound` dependency for WAV encoding
	- Added `tempfile` dependency for temporary file handling
	- Created `src/audio.rs` module with `AudioCaptureSession`
	- Added audio device enumeration and selection
	- Added capture lifecycle (start/stop) with sample buffering
	- Added WAV file writing with resampling to 16kHz
	- Added 6 Tauri commands for audio control
	- Added 5 new tests for audio module
- Completed: P4 final hardening + release UX:
	- Extended `AppConfig` with theme, default_output_dir, default_model_dir, diagnostics_enabled, default_threads, default_timeout_ms
	- Added 8 new Tauri commands for settings management
	- Added `pick_directory_command` for directory picker
	- Added settings UI sections: Appearance, Default Paths, Execution Defaults, Diagnostics
	- Added keyboard accessibility: focus styles, Ctrl+R shortcut, accesskey attributes
	- Created `capabilities/default.json` with explicit Tauri v2 permissions
	- Updated `tauri.conf.json` with security configuration
	- Extended `release-checklist.md` with P4 verification steps
- Completed: P3 batch processing + history:
	- Added `QueueItem` and `HistoryItem` contracts with status tracking
	- Added queue management to `ConfigStore`: add, remove, reorder, update status
	- Added history management with 50-item limit
	- Added 9 Tauri commands for queue and history operations
	- Added batch queue UI with add/remove/reorder and status display
	- Added history UI with timestamp, success status, and rerun action
	- Queue processes files sequentially with visible progress
- Completed: P2.5 shortcut conflict detection:
	- Added `is_shortcut_registered_command` to check if shortcut is registered
	- Added `detectShortcutConflicts()` function to find duplicate shortcuts
	- Added `updateShortcutConflictUI()` for visual feedback (orange border, hint text)
	- Added real-time conflict checking on input change
	- Blocks registration when conflicts exist
- Completed: P2 advanced controls + presets:
	- Added `TranscriptionAdvancedOptions` with task, language, threads, beam_size, best_of, temperature
	- Extended `WhisperCliRequest` and `build_whisper_cli_args` for advanced flags
	- Added `TranscriptionPreset` struct and preset methods to `ConfigStore`
	- Added preset Tauri commands: list, get, save, delete, set_default, get_default
	- Added collapsible advanced options drawer in UI
	- Added preset dropdown with save/load/delete and default preset toggle
	- Advanced options persist via localStorage, presets via backend config
- Completed: P2.5 background mode + global actions foundation:
	- Added `tauri-plugin-autostart` for OS-level launch-on-login
	- Added `AppConfig.start_in_background` and `AppConfig.launch_on_login` fields
	- Added `ConfigStore::set_start_in_background` and `set_launch_on_login` methods
	- Added `get_app_settings_command`, `set_start_in_background_command`, `set_launch_on_login_command`
	- Window now hides on startup when start-in-background is enabled
	- UI toggles now persist via backend commands instead of localStorage
	- Tray/menu bar already working with Open/Hide/Quit actions
	- Global shortcut registration already working
- Completed: P1 layout refinements:
	- Added section headers: "Input Files", "Controls", "Transcript", "Settings"
	- Combined model and audio pickers into unified Input Files section
	- Added visual divider between main workflow and settings
	- Updated CSS with section, section-header, field-group, and divider styles
- Completed: P1 error hints panel implementation:
	- Added `hint` field to `ApiError` struct in `src/errors.rs`
	- Added `recovery_hint_for_code` function mapping error codes to actionable hints
	- Added `ApiError::new` constructor that auto-derives hints
	- Updated all direct `ApiError` constructions to use the new constructor
	- Added UI panel for displaying error hints in `dist/index.html`
	- Added JavaScript logic to show/hide hint panel based on error response
	- Added tests for hint coverage on common error codes

### Next immediate action

- **CRITICAL**: Debug "Tauri invoke API not available" error on file dialogs
	- Check if `tauri-plugin-dialog` is properly initialized
	- Verify dialog permissions in capabilities
	- Check if async commands are being called correctly from frontend
	- Consider enabling dev tools in release build for debugging
- P1, P2, P2.5, P3, P4 are fully complete.
- P6 audio capture backend is complete. Remaining: wire global shortcut to PTT flow, add UI for device selection.
- P5 output routing UI is partially complete.

## 2026-02-12

### What was learned

- Root project builds via CMake and ships reusable C API in `include/whisper.h`.
- Typical inference workflow relies on model files under `models/` and executable flows like `whisper-cli`.
- `frontend-tauri` currently starts as a minimal Rust crate with Tauri dependency and no implemented frontend architecture yet.

### Current progress (historical)

- Completed: repository skim and frontend planning initialization.
- Completed: M0 baseline implementation (README, task aliases, CI workflow, smoke test).
- Completed: M1 backend shell implementation.
- Completed: M2 MVP UI bootstrap (backend-first).
- Completed: M3 whisper execution bridge (initial service slice).
- Completed: M4 release readiness docs.
- Latest test count: 41 passing tests in lib target.

### Next immediate action

- Update changelog.md with error hints feature and ask user approval for commit.

