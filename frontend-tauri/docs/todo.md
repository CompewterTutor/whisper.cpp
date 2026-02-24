# frontend-tauri TODO

Date initialized: 2026-02-12
Last updated: 2026-02-24

Legend: `[ ]` pending, `[~]` in progress, `[x]` done

## Current status snapshot

- [x] Runtime shell launches on Windows and loads bundled assets.
- [x] Core Rust backend contracts/commands/execution path implemented and tested.
- [x] Minimal interactive UI shell wired to command invocations.
- [x] Native model/audio file pickers integrated.
- [x] Transcript export actions integrated (`txt`, `srt`, `vtt`, `json`).
- [x] Inline validation and run gating integrated.
- [x] Open output folder action integrated after export.
- [x] Last-used model/audio/export context persisted locally.

## Now — Next feature slice (P1)

- [x] Add transcript export actions (`txt`, `srt`, `vtt`, `json`) in UI + backend command.
- [x] Add clearer run-state panel (idle/running/success/error + elapsed time).
- [x] Add inline validation messages next to model/audio fields.
- [x] Add "open output folder" action after successful run.
- [x] Add tests for export request validation and run-state transitions.
- [x] Run `fmt`, `clippy`, `test`, and `build --release`.
- [x] Add transcript metadata summary (line/segment count, estimated duration).
- [x] Add explicit recoverable error hints panel for common failures.

## P1 — Complete single-file transcribe UX

- [x] Refine layout into sections: inputs, controls, transcript, export.
- [x] Persist last-used model/audio/output directory and options.
- [x] Add transcript metadata summary (duration, segments, language if available).
- [x] Ensure all user-facing errors include actionable recovery hints.

Acceptance criteria:
- [x] User can complete full workflow without terminal interaction.
- [x] User can export transcript in at least two formats.
- [x] Invalid model/audio clearly blocks run and shows reason.

## P2 — Advanced controls + presets

- [x] Add advanced options drawer (task, language, beam, best-of, temp, threads, timeout).
- [x] Add preset save/load/delete flow.
- [x] Add default preset selection in settings.
- [x] Add reducer + serialization tests for preset and advanced options.

Acceptance criteria:
- [x] One-click basic workflow remains unchanged.
- [x] Advanced options survive app restart.

## P2.5 — Background mode + global actions foundation

- [x] Add tray/menu bar app mode with quick actions (open, start/stop, quit).
- [x] Add settings toggles: start in background and launch at login.
- [x] Add global shortcut registration UI and persistence.
- [x] Add conflict detection/rebind UX for shortcuts.

Acceptance criteria:
- [x] App can stay running hidden with tray/menu bar control.
- [x] At least two global shortcuts are configurable and persisted.

## P3 — Batch processing + history

- [x] Add batch queue view with multi-file add/remove/reorder.
- [x] Add per-item status and retry action.
- [x] Add run history list with rerun and open-output actions.
- [x] Add tests for queue transitions and rerun behavior.

Acceptance criteria:
- [x] Queue can process >1 file reliably with visible progress.
- [x] History enables rerun with same settings.

## P4 — Final hardening + release UX

- [x] Add settings page (theme, defaults, diagnostics toggle, paths).
- [x] Add keyboard accessibility pass on core controls.
- [x] Add capability/security config review for Tauri command surface.
- [x] Extend release smoke checklist for packaged artifact UX.

Acceptance criteria:
- [x] Release build verified on Windows with packaged startup + transcription smoke.
- [x] Settings and defaults persist and are reversible.

## P5 — Push-to-talk + output routing

- [x] Add microphone capture session model (`idle/listening/transcribing/error`).
- [ ] Add push-to-talk shortcut modes (`hold` and `toggle`).
- [x] Add output routing settings UI (clipboard, file, type emulation toggles).
- [x] Add clipboard commands (copy_to_clipboard, get_clipboard_text).
- [x] Add PTT routing commands (get_ptt_routing, set_ptt_routing).
- [ ] Add output routing actions:
	- [x] capture result to clipboard (UI + commands wired)
	- [ ] capture result to text file
	- [ ] emulate typing into focused app (opt-in)
- [ ] Wire global shortcut handler to PTT capture flow.
- [x] Add tests for capture lifecycle and routing adapters.

Acceptance criteria:
- [ ] User can trigger a PTT session entirely via global shortcut.
- [ ] Result can be routed to clipboard or file without opening main window.
- [ ] Type emulation is explicit opt-in and can be disabled globally.

## P6 — Push-to-talk audio capture pipeline

- [x] Add microphone device selection and capture lifecycle.
- [x] Buffer and segment captured audio for transcription.
- [x] Integrate capture session with existing execution + result routing.
- [x] Add audio capture module with cpal dependency.
- [x] Add WAV encoding with hound dependency.
- [x] Add Tauri commands for audio device enumeration and capture control.

Tests:
- [x] Capture state machine tests (`idle/listening/transcribing/error`).
- [x] Adapter tests for audio buffering/encoding boundaries.
- [ ] End-to-end PTT smoke test behind feature flag.

## Process checklist (per slice)

- [ ] Keep slices small and test-backed.
- [ ] Update docs (`plan.md`, `todo.md`, `memory.md`, `changelog.md`) for user-visible changes.
- [ ] Propose commit message and ask for approval before commit.

