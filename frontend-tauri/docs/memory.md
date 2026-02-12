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
- Validation completed via Cargo commands:
	- `cargo fmt --check`
	- `cargo clippy --all-targets --all-features -- -D warnings`
	- `cargo test`
- Validation also completed via Makefile alias:
	- `make check`
- Created/updated planning docs:
	- `frontend-tauri/docs/plan.md`
	- `frontend-tauri/docs/todo.md`
	- `frontend-tauri/docs/memory.md`
	- `frontend-tauri/changelog.md`

### Environment note

- `make` is available in the current shell (`GNU Make 4.4.1` on Windows32).

### Next immediate action

- Prepare the M1 commit message proposal and ask user approval before committing.

