# frontend-tauri Memory

## 2026-02-24

### What was learned

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

### Constraints to preserve

- Keep implementation incremental and test-first.
- Keep steps small and verifiable.
- Always ask user before committing.
- Use proper Conventional Commit messages.
- Follow Semantic Versioning for frontend releases.
- Maintain `frontend-tauri/changelog.md` as source of release notes.

### Current progress

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

- P1, P2, P2.5 are now fully complete.
- Next phases: P3 (Batch processing + history) or P4 (Final hardening + release UX).

- P1, P2, P2.5 are complete.
- Remaining P2.5 item: conflict detection/rebind UX for shortcuts (optional).
- Next phases: P3 (Batch processing + history) or P4 (Final hardening + release UX).

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

