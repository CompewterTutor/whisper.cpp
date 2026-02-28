# Careless Memory

## 2026-02-27 (R1.6 Batch Queue Component)

### What was learned

- `$derived` in Svelte 5 creates computed values that automatically update when dependencies change.
- Keyed each blocks with `(item.id)` ensure proper DOM reconciliation when list items change.
- Status updates during async iteration require explicit reactivity triggers (`queue = queue`).
- Callback props like `onrunitem` allow parent components to control batch execution logic.

### Architecture decisions

- Created `BatchQueue.svelte` as a self-contained component with:
  - Queue list with add/remove/reorder functionality
  - Per-item status display (Pending, Running, Success, Error)
  - Run all and clear completed actions
  - Empty state message
- Added queue types and API functions to `tauri.ts`:
  - `QueueItem`, `QueueItemStatus`, `QueueItemStatusUpdate` types
  - `getQueue`, `addToQueue`, `removeFromQueue`, `reorderQueue`, `clearCompletedQueue`, `updateQueueItemStatus`
- Component uses `onrunitem` callback for batch execution, keeping transcription logic in parent.

### Files created/modified

- `web/src/lib/components/BatchQueue.svelte` - New component
- `web/src/lib/components/index.ts` - Added export
- `web/src/lib/services/tauri.ts` - Added queue API functions
- `web/src/routes/+page.svelte` - Integrated BatchQueue for testing

### Next immediate action

- R1.7: Extract UI components - History panel (history list, rerun action, clear history)

## 2026-02-27 (R1.5 Transcript Section Component)

### What was learned

- `$effect()` in Svelte 5 is used for side effects that react to state changes (similar to `$:` in Svelte 4).
- The `$effect()` runs both on mount and when reactive dependencies change.
- Two-way binding with `bind:transcript` allows parent components to read and write component state.
- Callback props like `onstatus` provide a clean way for child-to-parent communication.

### Architecture decisions

- Created `TranscriptSection.svelte` as a self-contained component with:
  - Metadata display (segment count, estimated duration)
  - Export action buttons (Copy, TXT, SRT, VTT, JSON, Open folder)
  - Transcript viewer with loading/empty states
- Component manages its own export path state with localStorage persistence.
- Uses callback prop `onstatus` for status messages instead of global state.
- Export API functions already existed in `tauri.ts` service layer.

### Files created/modified

- `web/src/lib/components/TranscriptSection.svelte` - New component
- `web/src/lib/components/index.ts` - Added export
- `web/src/routes/+page.svelte` - Integrated TranscriptSection for testing

### Next immediate action

- R1.6: Extract UI components - Batch queue (queue list, add/remove/reorder, run all)

## 2026-02-27 (R1.4 Options Drawer Component)

### What was learned

- Svelte 5 component state can be exposed via `$props()` with `$bindable()` for two-way binding.
- The `onMount` lifecycle hook is used for initialization (loading presets, restoring state).
- CSS class toggling uses `class:open` syntax for conditional classes.
- Form elements can directly bind to state with `bind:value={variable}` for automatic updates.

### Architecture decisions

- Created `OptionsDrawer.svelte` as a self-contained component with:
  - Collapsible drawer toggle with arrow indicator
  - Preset management (load/save/delete/set default)
  - Advanced options grid (task, language, threads, beam size, best of, temperature)
- Extended `tauri.ts` service layer with preset API functions:
  - `listPresets()`, `getPreset()`, `savePreset()`, `deletePreset()`
  - `setDefaultPreset()`, `getDefaultPreset()`
- Options are persisted to localStorage and synced with backend presets.
- Added `AdvancedOptions` and `Preset` TypeScript interfaces.

### Files created/modified

- `web/src/lib/components/OptionsDrawer.svelte` - New component
- `web/src/lib/components/index.ts` - Added export
- `web/src/lib/services/tauri.ts` - Added preset API functions
- `web/src/routes/+page.svelte` - Integrated OptionsDrawer for testing

### Next immediate action

- R1.5: Extract UI components - Transcript display (metadata, export actions, transcript viewer)

## 2026-02-24 (R1.3 Input Section Components)

### What was learned

- Svelte 5 components use `$props()` with `$bindable()` for two-way binding.
- Snippets are the new way to pass content/children to components (`Snippet` type from 'svelte').
- Type declarations for Tauri's `window.__TAURI__` need to be added to avoid TypeScript errors.
- The `@tauri-apps/api/core` package provides the `invoke` function for calling Tauri commands.
- Component exports use barrel files (`index.ts`) for clean imports.

### Architecture decisions

- Created `web/src/lib/services/tauri.ts` as a service layer for Tauri API calls with type-safe wrappers.
- Created `web/src/lib/components/FilePicker.svelte` as a reusable file path input with validation.
- Created `web/src/lib/components/InputSection.svelte` combining model and audio pickers.
- Created `web/src/lib/types/tauri.d.ts` for Tauri API type declarations.
- Components handle their own state internally with localStorage persistence.

### Files created

- `web/src/lib/services/tauri.ts` - Tauri API service layer
- `web/src/lib/components/FilePicker.svelte` - Reusable file picker component
- `web/src/lib/components/InputSection.svelte` - Input files section
- `web/src/lib/components/index.ts` - Component barrel export
- `web/src/lib/types/tauri.d.ts` - Tauri type declarations

### Next immediate action

- R1.4: Extract UI components - Options drawer (advanced options, presets)

## 2026-02-24 (R1.2 Base Layout + Theming)

### What was learned

- CSS custom properties work well for theming in SvelteKit - define in `:root` and override with `[data-theme='light']` or `[data-theme='dark']`.
- Svelte stores can be used for theme state with localStorage persistence.
- The `$app/environment` module provides `browser` flag to detect client-side execution.
- `window.matchMedia('(prefers-color-scheme: dark)')` can detect system theme preference.
- CSS files can be imported directly in Svelte components with `<script>` imports.

### Architecture decisions

- Created `web/src/lib/styles/theme.css` with CSS custom properties for colors, spacing, typography.
- Created `web/src/lib/styles/base.css` with global element styles (reset, body, buttons, inputs, etc.).
- Created `web/src/lib/stores/theme.ts` with a Svelte store for theme management (system/light/dark).
- Theme is applied via `data-theme` attribute on `<html>` element.
- Styles are imported in `+layout.svelte` to apply globally.

### Files created

- `web/src/lib/styles/theme.css` - CSS custom properties for theming
- `web/src/lib/styles/base.css` - Global base styles
- `web/src/lib/stores/theme.ts` - Theme store with localStorage persistence

### Next immediate action

- R1.3: Extract UI components - Input section (model/audio pickers, validation)

## 2026-02-24 (SvelteKit Refactor)

### What was learned

- SvelteKit 5 with `sv create` is the modern way to scaffold (replaces deprecated `create-svelte`).
- Static adapter (`@sveltejs/adapter-static`) is required for Tauri desktop apps - no SSR needed.
- SvelteKit requires `export const ssr = false` and `export const prerender = true` in `+layout.ts` for SPA mode.
- `npm --prefix` has issues on Windows with path resolution - use npm workspaces instead.
- Tauri dev server expects port 1420 by convention.
- CSRF config in SvelteKit needs `trustedOrigins` including `tauri://localhost` for custom protocol.
- Vite config should set `strictPort: true` for Tauri compatibility.
- `@tauri-apps/api` package provides TypeScript types for Tauri APIs.

### Architecture decisions

- Frontend code now lives in `web/` subdirectory (SvelteKit project)
- Root `package.json` uses npm workspaces to manage `web/` package
- Build output goes to `web/build/` (configured in `svelte.config.js`)
- Tauri config updated with `beforeDevCommand` and `beforeBuildCommand` pointing to npm scripts
- App renamed from "frontend-tauri" to "Careless" across all configs

### Constraints to preserve

- Keep implementation incremental and test-first.
- Keep steps small and verifiable.
- Always ask user before committing.
- Use proper Conventional Commit messages.
- Follow Semantic Versioning for releases.
- Maintain `changelog.md` as source of release notes.

### Current progress

- Completed: R1.1 SvelteKit project structure setup
  - Created `web/` directory with SvelteKit 5 + TypeScript
  - Configured static adapter for Tauri compatibility
  - Set up npm workspaces for clean Windows compatibility
  - Added `@tauri-apps/api` for type-safe Tauri integration
  - Updated `tauri.conf.json` with new build paths and commands
  - Updated `Makefile` with `web-*` targets
  - Renamed app to "Careless"

### Next immediate action

- R1.2: Create base layout and theming system (extract CSS from `dist/index.html`)

## 2026-02-24 (P6 Audio Capture)

### What was learned

- P6 audio capture uses `cpal` crate for cross-platform microphone input.
- `hound` crate handles WAV encoding for whisper-cli compatibility.
- Audio samples are captured as f32, converted to mono, and can be resampled to 16kHz.
- `AudioCaptureSession` manages device selection, capture lifecycle, and sample buffering.
- Capture state machine: Idle -> Listening -> (Transcribing|Error) -> Idle.
- Tauri state with `Mutex<AudioCaptureSession>` allows thread-safe access from commands.
- `tauri-plugin-dialog` is preferred over `rfd` for file dialogs in Tauri apps.
- **Tauri v2 requires `withGlobalTauri: true` in `app` section of `tauri.conf.json`** to inject `window.__TAURI__` globally.

### Current progress (P6)

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
