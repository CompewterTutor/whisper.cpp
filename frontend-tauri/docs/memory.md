# frontend-tauri Memory

## 2026-02-12

### What was learned

- Root project builds via CMake and ships reusable C API in `include/whisper.h`.
- Typical inference workflow relies on model files under `models/` and executable flows like `whisper-cli`.
- `frontend-tauri` currently starts as a minimal Rust crate with Tauri dependency and no implemented frontend architecture yet.

### Constraints to preserve

- Keep implementation incremental and test-first.
- Keep steps small and verifiable.
- Always ask user before committing.
- Use proper Conventional Commit messages.
- Follow Semantic Versioning for frontend releases.
- Maintain `frontend-tauri/changelog.md` as source of release notes.

### Current progress

- Completed: repository skim and frontend planning initialization.
- Completed: M0 baseline implementation (README, task aliases, CI workflow, smoke test).
- Completed: M1 backend shell implementation:
	- typed contracts in `src/contracts.rs`
	- error mapping in `src/errors.rs`
	- config persistence in `src/config.rs`
	- model/audio validators in `src/commands.rs`
	- module export in `src/lib.rs`
- In progress: M2 MVP UI bootstrap (backend-first):
	- run command contract and placeholder transcript flow in `src/commands.rs`
	- transcription run request/response types in `src/contracts.rs`
	- UI workflow state model with loading/success/error transitions in `src/ui_state.rs`
	- interaction helpers for model/audio pickers, start availability, transcript panel, and error banner in `src/ui_state.rs`
	- integration sequence test for select inputs -> run -> render transcript
	- tauri command wrappers in `src/tauri_commands.rs`
	- webview reducer/view-model binding in `src/mvp_binding.rs`
- In progress: M3 whisper execution bridge (initial service slice):
	- `execution` module with CLI runner abstraction and process runner
	- whisper-cli arg builder and stdout parser
	- mocked runner integration tests
	- parser integration used in `run_transcription_mvp`
- Validation completed via Cargo commands:
	- `cargo fmt --check`
	- `cargo clippy --all-targets --all-features -- -D warnings`
	- `cargo test`
- Validation also completed via Makefile alias:
	- `make check`
- Latest test count: 28 passing tests (27 lib + 1 main).
- Created/updated planning docs:
	- `frontend-tauri/docs/plan.md`
	- `frontend-tauri/docs/todo.md`
	- `frontend-tauri/docs/memory.md`
	- `frontend-tauri/changelog.md`

### Environment note

- `make` is available in the current shell (`GNU Make 4.4.1` on Windows32).

### Next immediate action

- Add cancellation/timeout support to `execution` runner and expose timeout-aware command path.

