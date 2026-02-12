# Rust + Tauri Best Practices (Context7 Notes)

Updated: 2026-02-12
Scope: practical guardrails for `frontend-tauri` development in this repo.

## Sources used

- Rust Book: `/rust-lang/book`
- Cargo docs: `/websites/doc_rust-lang_stable_cargo`
- Clippy docs: `/rust-lang/rust-clippy`
- Tauri v2 docs: `/websites/v2_tauri_app`

## Rust application structure

- Keep `main.rs` small and orchestration-only; put business logic in `lib.rs` modules so logic is testable.
- Use `Result` for expected runtime/user errors; reserve `panic!` for unrecoverable programming invariants.
- Keep user-facing error handling centralized to avoid inconsistent messages.
- Prefer focused unit tests in `src/*` plus integration tests for public API paths.

## Error handling conventions

- During quick prototyping, `unwrap`/`expect` can be temporary markers, but remove them in production command paths.
- Surface structured errors from command boundaries (code + message) for predictable frontend handling.
- For CLI/IO boundaries, convert low-level errors into domain-specific errors early.

## Cargo and reproducibility

- Commit `Cargo.lock` for deterministic builds and CI consistency.
- Use `cargo build --release` for production verification.
- Keep profile tuning in Cargo profiles rather than ad-hoc rustflags unless strictly needed.
- Prefer minimal dependency additions; review transitive impact when introducing UI/runtime crates.

## Clippy policy

- Keep `clippy` in CI and fail on warnings for stable quality gates.
- Avoid blanket enabling highly opinionated lint groups; adopt stricter groups incrementally.
- When a lint must be allowed, scope the allow narrowly and document rationale.

## Tauri v2 security and command design

- Apply least-privilege command exposure: only register/invoke commands needed by the UI.
- Use capability files to explicitly allow command access per window/scope.
- Keep frontend-originated input validation on Rust side even when UI validates first.
- Review `SecurityConfig` (CSP, headers, capabilities) as part of release hardening.
- If using `window.__TAURI__.core.invoke`, ensure config expectations are explicit; otherwise prefer API package imports.

## Tauri packaging/runtime practices

- Ensure `build.frontendDist` points to a real packaged asset directory.
- Keep a minimal fallback `dist/index.html` so release binaries always launch.
- Verify release startup on Windows after config/build-script changes.
- Keep icon/resources present when using `tauri-build` to avoid build-time/runtime drift.

## Recommended next hardening steps for this repo

1. Add explicit `capabilities/` files for current commands and default window only.
2. Add a release smoke checklist item for CSP/capability review before tagging.
3. Add one integration test path that exercises command error mapping from frontend input.
4. Decide on global Tauri API strategy (`window.__TAURI__` vs package import) and standardize.
5. Track security/dependency checks in release flow (`cargo audit` or equivalent policy).

## Quick PR checklist (Rust + Tauri)

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test`
- [ ] `cargo build --release`
- [ ] No unchecked `unwrap`/`expect` in runtime command paths
- [ ] Command input validated in Rust
- [ ] `Cargo.lock` updated and committed when dependencies changed
- [ ] `frontendDist` and startup assets verified in release build
- [ ] Capability/security config reviewed for new command surface
