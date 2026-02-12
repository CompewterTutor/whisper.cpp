# frontend-tauri TODO

Date initialized: 2026-02-12
Last updated: 2026-02-12

Legend: `[ ]` pending, `[~]` in progress, `[x]` done

## Current status snapshot

- [x] Runtime shell launches on Windows and loads bundled assets.
- [x] Core Rust backend contracts/commands/execution path implemented and tested.
- [x] Minimal interactive UI shell wired to command invocations.
- [x] Native model/audio file pickers integrated.

## Now — Next feature slice (P1)

- [ ] Add transcript export actions (`txt`, `srt`, `vtt`, `json`) in UI + backend command.
- [ ] Add clearer run-state panel (idle/running/success/error + elapsed time).
- [ ] Add inline validation messages next to model/audio fields.
- [ ] Add "open output folder" action after successful run.
- [ ] Add tests for export request validation and run-state transitions.
- [ ] Run `fmt`, `clippy`, `test`, and `build --release`.

## P1 — Complete single-file transcribe UX

- [ ] Refine layout into sections: inputs, controls, transcript, export.
- [ ] Persist last-used model/audio/output directory and options.
- [ ] Add transcript metadata summary (duration, segments, language if available).
- [ ] Ensure all user-facing errors include actionable recovery hints.

Acceptance criteria:
- [ ] User can complete full workflow without terminal interaction.
- [ ] User can export transcript in at least two formats.
- [ ] Invalid model/audio clearly blocks run and shows reason.

## P2 — Advanced controls + presets

- [ ] Add advanced options drawer (task, language, beam, best-of, temp, threads, timeout).
- [ ] Add preset save/load/delete flow.
- [ ] Add default preset selection in settings.
- [ ] Add reducer + serialization tests for preset and advanced options.

Acceptance criteria:
- [ ] One-click basic workflow remains unchanged.
- [ ] Advanced options survive app restart.

## P3 — Batch processing + history

- [ ] Add batch queue view with multi-file add/remove/reorder.
- [ ] Add per-item status and retry action.
- [ ] Add run history list with rerun and open-output actions.
- [ ] Add tests for queue transitions and rerun behavior.

Acceptance criteria:
- [ ] Queue can process >1 file reliably with visible progress.
- [ ] History enables rerun with same settings.

## P4 — Final hardening + release UX

- [ ] Add settings page (theme, defaults, diagnostics toggle, paths).
- [ ] Add keyboard accessibility pass on core controls.
- [ ] Add capability/security config review for Tauri command surface.
- [ ] Extend release smoke checklist for packaged artifact UX.

Acceptance criteria:
- [ ] Release build verified on Windows with packaged startup + transcription smoke.
- [ ] Settings and defaults persist and are reversible.

## Process checklist (per slice)

- [ ] Keep slices small and test-backed.
- [ ] Update docs (`plan.md`, `todo.md`, `memory.md`, `changelog.md`) for user-visible changes.
- [ ] Propose commit message and ask for approval before commit.

