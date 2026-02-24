# frontend-tauri Release Checklist

Date: 2026-02-12
Last updated: 2026-02-24

## Scope

Use this checklist before any `frontend-tauri` release tag or announcement.

## 1) Version + changelog preparation

- [ ] Determine release type (`major` / `minor` / `patch`) from completed changes.
- [ ] Update `frontend-tauri/Cargo.toml` version.
- [ ] Add new dated section to `frontend-tauri/changelog.md`.
- [ ] Move relevant items from `Unreleased` into the new version section.

## 2) Local quality gates

- [ ] `make fmt`
- [ ] `make lint`
- [ ] `make test` (runs `cargo test`)
- [ ] `make verify-release` for combined check + optional smoke flow

## 3) Optional real execution smoke

- [ ] Set environment variables:
  - `WHISPER_CLI_PATH`
  - `WHISPER_MODEL_PATH`
  - `WHISPER_AUDIO_PATH`
- [ ] Run:
  - `make smoke-real`
- [ ] Confirm command executes and produces non-empty output.

## 4) CI checks

- [ ] Confirm `.github/workflows/frontend-tauri-ci.yml` is green for branch/PR.
- [ ] Verify lint + tests passed in CI logs.

## 5) Packaging readiness (when packaging is enabled)

- [ ] Verify target platform prerequisites for Tauri.
- [ ] Build package artifacts for target OS(es).
- [ ] Run startup smoke check on produced artifacts.

## 6) P4: Settings and hardening verification

- [ ] **Settings persistence**: Change each setting, restart app, verify values persist.
  - [ ] Theme (system/light/dark)
  - [ ] Default output directory
  - [ ] Default model directory
  - [ ] Diagnostics toggle
  - [ ] Default threads
  - [ ] Default timeout
  - [ ] Start in background
  - [ ] Launch on login
- [ ] **Keyboard accessibility**:
  - [ ] Tab through all interactive elements
  - [ ] Ctrl+R triggers transcription
  - [ ] Focus indicators visible on all controls
  - [ ] Accesskey shortcuts work (Alt+letter on Windows)
- [ ] **Security config review**:
  - [ ] `capabilities/default.json` has explicit permissions
  - [ ] No overly permissive wildcards in capabilities
  - [ ] Clipboard permissions scoped to text only
  - [ ] Global shortcut permissions present
  - [ ] Autostart permissions present

## 7) Packaged artifact smoke test

- [ ] **Startup verification**:
  - [ ] App launches from packaged executable
  - [ ] Window displays correctly
  - [ ] Tray icon appears (if applicable)
- [ ] **Transcription smoke**:
  - [ ] Model picker opens and selects valid model
  - [ ] Audio picker opens and selects valid audio file
  - [ ] Run transcription produces output
  - [ ] Export to TXT/SRT/VTT/JSON works
  - [ ] Open output folder opens correct directory
- [ ] **Settings verification**:
  - [ ] Settings load on startup
  - [ ] Settings save and persist across restarts

## 8) Commit + release hygiene

- [ ] Prepare Conventional Commit message(s).
- [ ] Ask user for approval before every commit.
- [ ] Commit docs/version/changelog updates.
- [ ] Tag release (if requested by user workflow).
