# Careless

A Tauri-based desktop frontend for `whisper.cpp` speech recognition.

## Tech Stack

- **Backend**: Rust + Tauri 2.x
- **Frontend**: SvelteKit 5 + TypeScript + Vite

## Prerequisites

- Rust toolchain (stable)
- Node.js 18+ and npm
- Tauri system prerequisites: <https://v2.tauri.app/start/prerequisites/>

## Local Development

From `frontend-tauri/`:

```bash
# Install dependencies
make web-install   # or: npm install

# Development (starts Vite dev server + Tauri)
make dev           # or: cargo tauri dev

# Build frontend only
make web-build     # or: npm run build

# Rust quality checks
make check         # fmt + lint + test

# Release verification
make verify-release
```

## Project Structure

```
frontend-tauri/
├── src/           # Rust backend (Tauri commands, config, execution)
├── web/           # SvelteKit frontend
│   ├── src/
│   │   ├── routes/    # SvelteKit routes/pages
│   │   └── lib/       # Shared components and utilities
│   └── build/         # Production build output
├── docs/          # Project documentation
└── dist/          # Legacy monolithic HTML (to be removed)
```

## Documentation

- `docs/plan.md` - Product vision and feature roadmap
- `docs/todo.md` - Current task tracking
- `docs/memory.md` - Development notes and decisions
- `changelog.md` - User-visible changes

## Workflow

- Use Conventional Commits for all commit messages
- Follow Semantic Versioning for releases
- Keep `changelog.md` updated for user-visible changes
- Always ask for approval before committing
