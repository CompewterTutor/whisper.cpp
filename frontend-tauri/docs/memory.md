# Careless Memory

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
