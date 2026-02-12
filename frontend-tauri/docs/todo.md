# frontend-tauri TODO

Date initialized: 2026-02-12

Legend: `[ ]` pending, `[~]` in progress, `[x]` done

## Now

- [x] Skim root project architecture and build flow (`README`, `CMakeLists`, `Makefile`, `whisper.h`, `examples/cli`).
- [x] Draft high-level implementation plan in `docs/plan.md`.
- [x] Initialize `docs/memory.md` and frontend changelog.

## M0 — Baseline

- [x] Add `frontend-tauri` README with setup and run instructions.
- [x] Add Rust/Tauri task aliases for fmt/lint/test/dev.
- [x] Add minimal CI workflow for frontend checks.
- [x] Add smoke unit test (`app metadata` / `health`).
- [x] Run tests: fmt + lint + unit.
- [x] Prepare commit message and ask for approval before commit.

Notes:
- `make check` now runs successfully on Windows in this environment.

## M1 — Backend shell

- [x] Define typed Tauri command inputs/outputs.
- [x] Implement config model and persistence layer.
- [x] Implement model path validation command.
- [x] Implement audio file validation command.
- [x] Add unit tests for each command.
- [x] Add error mapping tests.
- [x] Run test suite.
- [ ] Prepare commit message and ask for approval before commit.

## M2 — MVP UI

- [x] Add model picker UI.
- [x] Add audio picker UI.
- [x] Add transcript output panel.
- [x] Add loading/success/error states.
- [x] Wire UI to backend commands.
- [x] Add component tests for core states.
- [x] Add integration test for end-to-end UI flow (mock backend).
- [x] Run test suite.
- [ ] Prepare commit message and ask for approval before commit.

Notes:
- M2 interaction layer expanded in `ui_state` with model/audio picker setters, start-button eligibility, transcript panel rendering text, and error banner text.
- Added `tauri_commands` wrappers and `mvp_binding` reducer/view-model to represent webview action wiring.

## M3 — whisper.cpp execution

- [ ] Implement `whisper-cli` invocation service.
- [ ] Implement argument builder with validation.
- [ ] Implement stdout/stderr parser into transcript model.
- [ ] Add cancellation/timeout handling.
- [ ] Add unit tests for arg builder and parser.
- [ ] Add mocked process integration tests.
- [ ] Add optional real sample smoke test.
- [ ] Run test suite.
- [ ] Prepare commit message and ask for approval before commit.

## M4 — Release readiness

- [ ] Add release checklist document.
- [ ] Add version bump procedure and script notes.
- [ ] Add packaging smoke checks.
- [ ] Add changelog update verification step.
- [ ] Run full verification.
- [ ] Prepare release commit message and ask for approval before commit.

