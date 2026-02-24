# Careless Product + UI Plan

Date: 2026-02-12
Last updated: 2026-02-24
Scope: Define the end-goal UX and feature set for a modern desktop wrapper around `whisper.cpp`.

## Product vision

Build a fast, local-first transcription workstation that makes `whisper.cpp` accessible to non-CLI users while preserving power-user control over models and decoding settings.

## Primary users

1. **Quick transcriber**: wants drag-drop + one-click transcript.
2. **Power user**: wants tuning (`language`, `threads`, timestamps, output formats).
3. **Batch operator**: wants queue processing and repeatable presets.

## UX principles

1. **One-screen happy path**: basic transcription should be visible without opening settings.
2. **Progressive disclosure**: advanced options live in collapsible sections / settings page.
3. **Local-first clarity**: clearly show file paths, model source, and no-cloud execution.
4. **Recoverable failures**: every error includes actionable next step.
5. **Deterministic runs**: users can save/load presets and rerun with same parameters.

## End-goal information architecture

## 1) `Transcribe` (default view)

Core workflow card:
- Input source: single file picker + drag/drop.
- Model selector: recent models + browse + quick validate.
- Primary actions: `Transcribe`, `Cancel`.
- Runtime status: idle/loading/running/success/error with elapsed time.

Result area:
- Transcript viewer with timestamps toggle.
- Segment table (start/end/text).
- Copy transcript / save outputs (`.txt`, `.srt`, `.vtt`, `.json`).

Advanced panel (collapsed by default):
- Task: `transcribe` / `translate`.
- Language: auto or explicit code.
- Decode controls: temperature, beam size, best-of.
- Performance controls: threads, processors/GPU toggle (where available).
- Output controls: timestamps, diarization-ready JSON flag (future-compatible).

## 2) `Batch` view

- Queue multiple audio files.
- Per-item status (`queued`, `running`, `done`, `failed`, `cancelled`).
- Reorder/remove/retry actions.
- Shared preset for the queue.
- Export all outputs to selected folder.

## 3) `Models` view

- Registered models list (name, size, path, modified date).
- Validate model file and display compatibility notes.
- Set default model.
- Add/remove model references (no destructive file delete by default).

## 4) `History` view

- Recent runs with searchable metadata.
- Open transcript, open output folder, rerun with same settings.
- Capture failure reason for diagnostics.

## 5) `Settings` view

- App: theme (system/light/dark), startup behavior.
- Execution: default timeout, default thread count, process priority mode.
- Paths: default output directory, model directory.
- Privacy: local-only statement, optional diagnostics logging toggle.

## 6) `Background Agent` mode (tray/menu bar)

- App can run hidden in background with quick actions.
- **Windows**: system tray icon menu (`Start/Stop listening`, `Open app`, `Quit`).
- **macOS**: menu bar status item with equivalent actions.
- Optional startup behavior: launch hidden at login.
- Visual indicator for active listening/transcribing state.

## 7) `Global Actions` (hotkeys)

Core global shortcuts:
- Push-to-talk hold/toggle: start/stop capture + transcription.
- Capture result to clipboard.
- Capture result to file (append/new file behavior configurable).
- Emulate typing into focused app.

Safety/UX controls:
- Enable/disable each shortcut independently.
- Shortcut conflict detection and remapping UI.
- Privacy indicator while microphone capture is active.
- Confirmation/review mode before type-emulation (optional toggle).

## Feature inventory (prioritized)

### Must-have (v0.2-v0.4)

- Single-file transcription end-to-end.
- Model + audio pickers with validation.
- Run/cancel + clear status/errors.
- Transcript preview + save to text formats.
- Persistent user settings and last-used inputs.

### Should-have (v0.5-v0.7)

- Batch queue.
- Preset management (save/load/duplicate).
- History and rerun.
- Advanced decoding controls.
- Background tray/menu bar mode.
- Global hotkeys for clipboard/file capture.

### Nice-to-have (v0.8+)

- Waveform with segment jump.
- Full push-to-talk workflow with live/near-live capture pipeline.
- Type-emulation with per-app safety controls.
- Optional speaker-segmentation integration path.
- Plugin/extension hooks for post-processing.

## Settings schema (target)

`app`:
- theme: `system | light | dark`
- show_timestamps_default: `bool`
- start_in_background: `bool`
- launch_on_login: `bool`

`execution`:
- timeout_ms_default: `u64`
- thread_count_default: `u16`
- default_task: `transcribe | translate`
- language_default: `auto | <lang_code>`

`shortcuts`:
- ptt_mode: `hold | toggle`
- ptt_binding: `string`
- capture_to_clipboard_binding: `string`
- capture_to_file_binding: `string`
- type_emulation_binding: `string`
- shortcuts_enabled: `bool`

`capture`:
- microphone_device_id: `string | null`
- auto_copy_to_clipboard: `bool`
- auto_append_to_file: `bool`
- default_capture_file: `string | null`
- confirm_before_typing: `bool`

`io`:
- default_model_path: `string`
- default_output_dir: `string`
- output_formats: `txt | srt | vtt | json` (multi)

`advanced`:
- beam_size: `u8`
- best_of: `u8`
- temperature: `f32`

## Delivery phases from current state

### R1 — SvelteKit Refactor (CURRENT)

Goal: break up monolithic HTML into modular SvelteKit components.
- Set up Vite + SvelteKit 5 with TypeScript.
- Configure static adapter for Tauri compatibility.
- Extract CSS theming system into Svelte stores.
- Extract UI components: Input, Options, Transcript, Batch, History, Settings, PTT.
- Create Tauri service layer with type-safe API wrappers.
- Wire up state management with Svelte stores.
- Remove legacy `dist/index.html`.

Tests:
- TypeScript compilation and type checking.
- Build output verification.
- Hot reload development workflow.

### P1 — Solid MVP UI completion (DONE)

Goal: polish existing single-page shell into a complete single-file transcribe experience.
- Add clear layout sections (input, run controls, transcript, export).
- Add save/export actions for transcript outputs.
- Add stronger error surfaces and validation hints.

Tests:
- UI integration: happy path from pickers to transcript render.
- Command-level tests for export request validation.

### P2 — Advanced controls + presets (DONE)

Goal: keep one-click flow while exposing power-user controls.
- Add advanced options drawer.
- Add preset save/load.

Tests:
- State reducer tests for option changes and preset application.
- Serialization round-trip tests for presets.

### P3 — Batch and history (DONE)

Goal: support production workflows.
- Add queue orchestration and run history.
- Add rerun and retry flows.

Tests:
- Queue state transition tests.
- Integration tests for rerun from history.

### P4 — Hardening + release UX (DONE)

Goal: make desktop app robust and supportable.
- Add settings page and diagnostics toggles.
- Add accessibility/keyboard passes and release smoke checks.

Tests:
- Packaged artifact smoke tests.
- Settings persistence regression tests.

### P5 — Background mode + global shortcuts (IN PROGRESS)

Goal: make transcription available system-wide without keeping the main window open.
- Add tray/menu bar presence and stateful quick actions.
- Add global shortcut registration and conflict-safe configuration.
- Add output routing actions: clipboard, file, type-emulation.

Tests:
- Shortcut registration lifecycle tests (register/unregister/rebind).
- Output routing tests (clipboard/file/type adapters mocked).
- Background startup and tray/menu action smoke tests.

### P6 — Push-to-talk audio capture pipeline (MOSTLY DONE)

Goal: capture microphone input directly and transcribe on hotkey-driven sessions.
- Add microphone device selection and capture lifecycle.
- Buffer and segment captured audio for transcription.
- Integrate capture session with existing execution + result routing.

Tests:
- Capture state machine tests (`idle/listening/transcribing/error`).
- Adapter tests for audio buffering/encoding boundaries.
- End-to-end PTT smoke test behind feature flag.

## Feasibility and constraints

- **Feasible in Tauri**: tray/menu bar, global shortcuts, clipboard/file actions are standard desktop capabilities.
- **Typing emulation** is feasible but platform-sensitive and should be behind explicit opt-in.
- **Push-to-talk** is feasible but requires a new mic capture pipeline (currently app is file-based transcription).
- Security/permissions and clear user affordances are required (especially mic access on macOS and global input behavior).

## Definition of done (per feature slice)

1. UX behavior documented in `todo.md` acceptance checklist.
2. Backend + UI wiring implemented.
3. Unit/integration tests added and passing.
4. Changelog updated for user-visible changes.
5. Commit proposed and user approves before commit.

