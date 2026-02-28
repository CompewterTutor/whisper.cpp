# Careless TODO

Date initialized: 2026-02-12
Last updated: 2026-02-27

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
- [x] SvelteKit + Vite project structure initialized (Phase 1 of refactor)

## Now — SvelteKit Refactor (R1)

Breaking up the monolithic `dist/index.html` into modular SvelteKit components.

- [x] R1.1: Set up Vite + SvelteKit project structure
- [x] R1.2: Create base layout and theming system
- [x] R1.3: Extract UI components - Input section
- [x] R1.4: Extract UI components - Options drawer
- [x] R1.5: Extract UI components - Transcript display
- [x] R1.6: Extract UI components - Batch queue
- [x] R1.7: Extract UI components - History panel
- [x] R1.8: Extract UI components - Settings panel
- [ ] R1.9: Extract UI components - PTT interface
- [ ] R1.10: Create Tauri service layer
- [ ] R1.11: Wire up state management and stores
- [ ] R1.12: Update build configuration and test

Acceptance criteria:

- [ ] All existing UI functionality replicated in SvelteKit
- [ ] `make dev` works with hot reload
- [ ] `make check` passes
- [ ] Legacy `dist/index.html` removed

## P1 — Complete single-file transcribe UX (DONE)

- [x] Refine layout into sections: inputs, controls, transcript, export.
- [x] Persist last-used model/audio/output directory and options.
- [x] Add transcript metadata summary (duration, segments, language if available).
- [x] Ensure all user-facing errors include actionable recovery hints.

## P2 — Advanced controls + presets (DONE)

- [x] Add advanced options drawer (task, language, beam, best-of, temp, threads, timeout).
- [x] Add preset save/load/delete flow.
- [x] Add default preset selection in settings.

## P2.5 — Background mode + global actions foundation (DONE)

- [x] Add tray/menu bar app mode with quick actions.
- [x] Add settings toggles: start in background and launch at login.
- [x] Add global shortcut registration UI and persistence.
- [x] Add conflict detection/rebind UX for shortcuts.

## P3 — Batch processing + history (DONE)

- [x] Add batch queue view with multi-file add/remove/reorder.
- [x] Add per-item status and retry action.
- [x] Add run history list with rerun and open-output actions.

## P4 — Final hardening + release UX (DONE)

- [x] Add settings page (theme, defaults, diagnostics toggle, paths).
- [x] Add keyboard accessibility pass on core controls.
- [x] Add capability/security config review for Tauri command surface.

## P5 — Push-to-talk + output routing (IN PROGRESS)

- [x] Add microphone capture session model
- [ ] Add push-to-talk shortcut modes (`hold` and `toggle`)
- [x] Add output routing settings UI
- [ ] Wire global shortcut handler to PTT capture flow
- [ ] End-to-end PTT smoke test

## P6 — Audio capture pipeline (MOSTLY DONE)

- [x] Add microphone device selection and capture lifecycle.
- [x] Buffer and segment captured audio for transcription.
- [x] Integrate capture session with existing execution + result routing.
- [x] Add audio capture module with cpal dependency.
- [x] Add WAV encoding with hound dependency.

## Process checklist (per slice)

- [ ] Keep slices small and test-backed.
- [ ] Update docs (`plan.md`, `todo.md`, `memory.md`, `changelog.md`) for user-visible changes.
- [ ] Propose commit message and ask for approval before commit.
