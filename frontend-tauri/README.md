# frontend-tauri

Rust/Tauri frontend workspace for `whisper.cpp`.

## Current status

This frontend is in bootstrap phase.
Current milestone: M0 (baseline setup, quality gates, and CI checks).

## Prerequisites

- Rust toolchain (stable)
- Cargo
- Tauri system prerequisites for your OS

For Tauri requirements, follow: <https://v2.tauri.app/start/prerequisites/>

## Local development

From `frontend-tauri/`:

- `make fmt` - format check
- `make lint` - clippy warnings as errors
- `make test` - run unit tests
- `make check` - run all local quality checks
- `make dev` - run app locally

Direct cargo equivalents:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `cargo run`

## Workflow constraints

- Always ask for approval before committing.
- Use Conventional Commits for all commit messages.
- Follow Semantic Versioning (`MAJOR.MINOR.PATCH`) for frontend releases.
- Keep `changelog.md` updated for user-visible changes.
