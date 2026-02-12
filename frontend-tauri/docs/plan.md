# frontend-tauri Plan

Date: 2026-02-12
Scope: Build a Rust/Tauri frontend for `whisper.cpp` with clear milestones, small steps, and tests for each step.

## Project context (from root skim)

- Core inference and public API are exposed via `include/whisper.h` and built from CMake at repo root.
- Standard workflows use CMake + generated binaries (e.g. `whisper-cli`) and model files under `models/`.
- The current `frontend-tauri` crate is minimal (`tauri` dependency + `src/main.rs` hello-world).

## Guardrails

1. **No commits without confirmation**: before every commit, explicitly ask for approval.
2. **Commit format**: use Conventional Commits (e.g. `feat(ui): add model selection panel`).
3. **Semantic Versioning**: maintain `MAJOR.MINOR.PATCH` for frontend releases.
	- `MAJOR`: breaking UX/API/workflow changes.
	- `MINOR`: new backward-compatible features.
	- `PATCH`: fixes/internal improvements.
4. **Changelog discipline**: update `frontend-tauri/changelog.md` on every user-visible change.

## Milestones and step-by-step execution

### M0 — Baseline and reproducible dev setup

1. Add frontend architecture doc section (runtime model, data flow, boundaries).
2. Add local setup instructions for Windows/Linux/macOS and model prerequisites.
3. Add scripts/tasks to run: format, lint, test, dev.
4. Add CI skeleton for frontend checks only.

**Tests for M0**
- Run formatting check.
- Run lint check.
- Run unit test command (even if smoke test only).
- Run CI workflow locally or via dry-run tooling where available.

### M1 — Backend shell (Tauri commands + service layer)

1. Create typed command contract for:
	- app health/version
	- system capability check
	- model discovery/validation
2. Implement service modules with explicit error types.
3. Add config/state management (paths, selected model, preferences).
4. Keep whisper execution integration mocked/stubbed initially.

**Tests for M1**
- Unit tests for command handlers.
- Unit tests for config persistence and validation.
- Unit tests for error mapping.
- Contract tests for command input/output serialization.

### M2 — Minimal usable UI (MVP)

1. Build single-page MVP with only:
	- model path selection
	- audio file selection
	- start transcription button
	- transcript output panel
2. Add deterministic loading/error/success states.
3. Wire UI to backend command contract.

**Tests for M2**
- Component tests for each UI state.
- Integration test for “select inputs → run → render transcript”.
- Regression test for empty/invalid input handling.

### M3 — Real whisper.cpp execution bridge

1. Choose integration path:
	- invoke `whisper-cli` process (first implementation), or
	- direct FFI binding (later optimization milestone).
2. Implement process execution with cancellation and timeout.
3. Parse output into structured transcript model.
4. Surface robust runtime errors (missing model, unsupported audio, execution failure).

**Tests for M3**
- Unit tests for argument construction.
- Unit tests for output parsing.
- Integration tests with mocked process I/O.
- Optional end-to-end test with sample audio (feature-flagged in CI).

### M4 — Quality and release readiness

1. Add accessibility and keyboard checks for core workflow.
2. Add performance guardrails (cold start + transcription timing baselines).
3. Package app for target OSes.
4. Prepare release checklist and version bump workflow.

**Tests for M4**
- End-to-end happy path test.
- Smoke tests on packaged artifact.
- Release checklist verification test (scripted where possible).

## Definition of done (per step)

Each step is complete only when all are true:

1. Code implemented.
2. Tests for that step written and passing.
3. Docs updated (`plan.md`/`todo.md`/`memory.md`/`changelog.md` when relevant).
4. Proposed commit message prepared.
5. User confirms before commit.

## Initial commit message patterns

- `chore(frontend): initialize tauri project guardrails and docs`
- `feat(frontend): add transcription MVP flow`
- `test(frontend): add command and UI integration tests`
- `fix(frontend): handle whisper execution timeout and parse errors`

## Versioning flow

1. Determine change type (`MAJOR`/`MINOR`/`PATCH`).
2. Update version in `frontend-tauri/Cargo.toml`.
3. Add changelog entry under matching version/date.
4. Run full frontend test suite.
5. Ask user for commit approval.

