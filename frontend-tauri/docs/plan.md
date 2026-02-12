# frontend-tauri Product + UI Plan

Date: 2026-02-12  
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

### Nice-to-have (v0.8+)

- Waveform with segment jump.
- Hotkeys for core actions.
- Optional speaker-segmentation integration path.
- Plugin/extension hooks for post-processing.

## Settings schema (target)

`app`:
- theme: `system | light | dark`
- show_timestamps_default: `bool`

`execution`:
- timeout_ms_default: `u64`
- thread_count_default: `u16`
- default_task: `transcribe | translate`
- language_default: `auto | <lang_code>`

`io`:
- default_model_path: `string`
- default_output_dir: `string`
- output_formats: `txt | srt | vtt | json` (multi)

`advanced`:
- beam_size: `u8`
- best_of: `u8`
- temperature: `f32`

## Delivery phases from current state

### P1 — Solid MVP UI completion

Goal: polish existing single-page shell into a complete single-file transcribe experience.
- Add clear layout sections (input, run controls, transcript, export).
- Add save/export actions for transcript outputs.
- Add stronger error surfaces and validation hints.

Tests:
- UI integration: happy path from pickers to transcript render.
- Command-level tests for export request validation.

### P2 — Advanced controls + presets

Goal: keep one-click flow while exposing power-user controls.
- Add advanced options drawer.
- Add preset save/load.

Tests:
- State reducer tests for option changes and preset application.
- Serialization round-trip tests for presets.

### P3 — Batch and history

Goal: support production workflows.
- Add queue orchestration and run history.
- Add rerun and retry flows.

Tests:
- Queue state transition tests.
- Integration tests for rerun from history.

### P4 — Hardening + release UX

Goal: make desktop app robust and supportable.
- Add settings page and diagnostics toggles.
- Add accessibility/keyboard passes and release smoke checks.

Tests:
- Packaged artifact smoke tests.
- Settings persistence regression tests.

## Definition of done (per feature slice)

1. UX behavior documented in `todo.md` acceptance checklist.
2. Backend + UI wiring implemented.
3. Unit/integration tests added and passing.
4. Changelog updated for user-visible changes.
5. Commit proposed and user approves before commit.

