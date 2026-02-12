# frontend-tauri Release Checklist

Date: 2026-02-12

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

## 6) Commit + release hygiene

- [ ] Prepare Conventional Commit message(s).
- [ ] Ask user for approval before every commit.
- [ ] Commit docs/version/changelog updates.
- [ ] Tag release (if requested by user workflow).
