# Careless Memory

## 2026-02-27 (R1.12 Build Configuration and Test - SvelteKit Refactor Complete)

### What was learned

- npm workspaces in root package.json enable `npm run dev` to work from frontend-tauri directory.
- `make check` runs Rust tests (56 passing) covering all backend functionality.
- Legacy HTML can be safely removed once SvelteKit components replicate all functionality.

### Architecture decisions

- Verified tauri.conf.json configuration:
  - `frontendDist: "web/build"` points to SvelteKit output
  - `beforeDevCommand: "npm run dev"` uses workspace scripts
  - `beforeBuildCommand: "npm run build"` uses workspace scripts
- Removed legacy `dist/index.html` (79KB monolithic file)
- All SvelteKit refactor (R1) tasks complete

### Files removed

- `frontend-tauri/dist/index.html` - Legacy monolithic HTML (no longer needed)

### R1 SvelteKit Refactor Summary

Completed all 12 subtasks:
- R1.1: SvelteKit project structure
- R1.2: Base layout and theming
- R1.3: InputSection component
- R1.4: OptionsDrawer component
- R1.5: TranscriptSection component
- R1.6: BatchQueue component
- R1.7: HistoryPanel component
- R1.8: SettingsPanel component
- R1.9: PttInterface component
- R1.10: Tauri service layer with types
- R1.11: State management stores
- R1.12: Build configuration and test

### Next phase

- P5: Push-to-talk + output routing (wire global shortcuts to PTT capture)

## 2026-02-27 (R1.11 State Management and Stores)

### What was learned

- Svelte stores (`writable`, `derived`) provide reactive shared state across components.
- The `$` prefix in Svelte automatically subscribes to stores and unwraps values.
- `initPersistence()` on mount ensures localStorage state is restored before components render.
- Separating stores into categories (Input, Transcript, Options, PTT, Shortcuts, UI) improves organization.
- `derived` stores compute values from other stores (e.g., `canRunTranscription` from `modelValid` and `audioValid`).

### Architecture decisions

- Created `stores/app.ts` with shared reactive state:
  - **Input state**: `modelPath`, `audioPath`, `modelValid`, `audioValid`, `canRunTranscription` (derived)
  - **Transcript state**: `transcript`, `transcriptLoading`, `lastExportPath`
  - **Options state**: `advancedOptions`
  - **PTT state**: `pttMode`, `pttRouting`
  - **Shortcuts state**: `shortcuts`
  - **UI state**: `statusMessage`, `statusType`
- Created `stores/index.ts` as barrel file re-exporting all stores (including theme)
- Added `initPersistence()` function to restore and persist state to localStorage
- Updated `InputSection.svelte` to use shared stores instead of local state
- Updated `+page.svelte` to:
  - Initialize persistence on mount
  - Use store values with `$` prefix
  - Pass store values to components as props

### Files created/modified

- `web/src/lib/stores/app.ts` - New file with all shared stores
- `web/src/lib/stores/index.ts` - New barrel file
- `web/src/lib/components/InputSection.svelte` - Updated to use shared stores
- `web/src/routes/+page.svelte` - Updated to initialize persistence and use stores

### Next immediate action

- R1.12: Update build configuration and test (final verification, remove legacy HTML)

## 2026-02-27 (R1.10 Tauri Service Layer)

### What was learned

- Separating types into a dedicated `types.ts` file improves code organization and enables cleaner imports.
- Using section headers with `// ============================================================================` comments makes large files more navigable.
- A `safeInvoke` wrapper provides a Result-style error handling pattern for optional use.
- Re-exporting with `export * from './types'` allows consumers to import all types from the main module.

### Architecture decisions

- Created `types.ts` with all TypeScript interfaces and types:
  - `ValidationResult`, `ApiError`, `AppHealthResponse`
  - `AdvancedOptions`, `Preset`
  - `QueueItem`, `QueueItemStatus`, `QueueItemStatusUpdate`
  - `HistoryItem`
  - `AppSettings`
  - `PttRouting`, `ShortcutSettings`
  - `AudioDevice`, `PttState`, `CaptureResult`, `TranscriptionResult`
- Refactored `tauri.ts` to:
  - Import types from `types.ts`
  - Re-export all types with `export * from './types'`
  - Group functions into logical sections with clear headers
  - Add `safeInvoke` utility for Result-style error handling
- Created `services/index.ts` as a barrel file for cleaner imports
- Organized functions into sections: Core API utilities, File Validation, File Pickers, App Health, Transcription, Presets, Batch Queue, History, App Settings, PTT Routing, Global Shortcuts, Audio Capture

### Files created/modified

- `web/src/lib/services/types.ts` - New file with all type definitions
- `web/src/lib/services/tauri.ts` - Refactored with sections and type imports
- `web/src/lib/services/index.ts` - New barrel file

### Next immediate action

- R1.11: Wire up state management and stores (connect components with shared state)

## 2026-02-27 (R1.9 PTT Interface Component)

### What was learned

- CSS animations with `@keyframes pulse` create smooth status indicator effects.
- `onMount` and `onDestroy` lifecycle hooks are used to add/remove window event listeners.
- Window events (`ptt-start`, `ptt-stop`) allow backend-to-frontend communication for global shortcuts.
- `onmousedown`, `onmouseup`, `onmouseleave` events enable hold-to-record functionality.
- Timer display uses `setInterval` with cleanup in `onDestroy` to prevent memory leaks.
- Imported function names can be aliased to avoid conflicts with local functions.

### Architecture decisions

- Created `PttInterface.svelte` as a self-contained component with:
  - PTT status indicator with animated dot (idle/gray, listening/red pulsing, transcribing/yellow pulsing, error/red)
  - Status text and timer display
  - Support for hold and toggle PTT modes
  - Start/Stop buttons for manual control
  - Event listeners for global shortcut triggers (`ptt-start`, `ptt-stop`)
  - Reset button for error state recovery
- Added capture API functions to `tauri.ts`:
  - `PttState` type for component state
  - `CaptureResult` interface for transcription result
  - `startCapture`, `stopCapture` functions
- Component uses callback props (`onstatus`, `ontranscript`) for parent communication.
- Props include `modelPath` and `pttMode` for transcription and mode selection.

### Files created/modified

- `web/src/lib/components/PttInterface.svelte` - New component
- `web/src/lib/components/index.ts` - Added export
- `web/src/lib/services/tauri.ts` - Added capture API functions and types
- `web/src/routes/+page.svelte` - Integrated PttInterface

### Next immediate action

- R1.10: Create Tauri service layer (consolidate API calls and state management)

## 2026-02-27 (R1.8 Settings Panel Component)

### What was learned

- Complex settings can be organized into sub-cards within a main section for better UX.
- Shortcut conflict detection uses a Map to track duplicate values across multiple inputs.
- The `class:conflict` directive in Svelte allows dynamic styling based on validation state.
- Settings are loaded on mount and persisted through individual change handlers.
- localStorage is used for shortcut bindings (not Tauri backend) for faster UI response.

### Architecture decisions

- Created `SettingsPanel.svelte` as a comprehensive settings component with:
  - Appearance: Theme selector (synced with theme store)
  - Default Paths: Output directory and model directory with directory pickers
  - Execution Defaults: Threads and timeout settings
  - Startup Behavior: Start in background and launch on login toggles
  - Global Shortcuts: Enable toggle + 4 shortcut inputs with conflict detection
  - PTT Audio Settings: Microphone selector and PTT mode (hold/toggle)
  - PTT Output Routing: Clipboard, file, and type emulation toggles
  - Diagnostics: Enable logging toggle
- Extended `tauri.ts` service layer with settings API functions:
  - `getAppSettings`, `setTheme`, `setDefaultOutputDir`, `setDefaultModelDir`
  - `setDefaultThreads`, `setDefaultTimeout`, `setDiagnosticsEnabled`
  - `setStartInBackground`, `setLaunchOnLogin`, `pickDirectory`
  - `getPttRouting`, `setPttRouting`
  - `getShortcuts`, `setShortcuts`, `registerGlobalShortcut`, `unregisterGlobalShortcut`
  - `listAudioDevices`, `selectAudioDevice`
- Added types: `AppSettings`, `PttRouting`, `ShortcutSettings`, `AudioDevice`
- Component uses `onstatus` callback for parent communication (consistent pattern).

### Files created/modified

- `web/src/lib/components/SettingsPanel.svelte` - New component
- `web/src/lib/components/index.ts` - Added export
- `web/src/lib/services/tauri.ts` - Added settings API functions and types
- `web/src/routes/+page.svelte` - Integrated SettingsPanel, removed Theme Test section

### Next immediate action

- R1.9: Extract UI components - PTT interface (status indicator, capture controls)

## 2026-02-27 (R1.7 History Panel Component)

### What was learned

- Timestamp formatting uses `Date.toLocaleTimeString()` for locale-aware display.
- Conditional CSS classes use `class:error={!item.success}` syntax in Svelte 5.
- Confirmation dialogs (`confirm()`) should be used before destructive actions like clearing history.
- The `onrerun` callback pattern allows parent to control what happens when user clicks rerun.

### Architecture decisions

- Created `HistoryPanel.svelte` as a self-contained component with:
  - History list with timestamp, filename, success/error indicator
  - Rerun action button per item
  - Clear history button with confirmation
  - Empty state message
- Added history types and API functions to `tauri.ts`:
  - `HistoryItem` type with id, audio_path, output_path, timestamp_ms, success
  - `getHistory`, `addToHistory`, `clearHistory` functions
- Component uses callbacks (`onstatus`, `onrerun`) for parent communication.

### Files created/modified

- `web/src/lib/components/HistoryPanel.svelte` - New component
- `web/src/lib/components/index.ts` - Added export
- `web/src/lib/services/tauri.ts` - Added history API functions
- `web/src/routes/+page.svelte` - Integrated HistoryPanel

### Next immediate action

- R1.8: Extract UI components - Settings panel (theme, paths, execution defaults, shortcuts, PTT settings)

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
