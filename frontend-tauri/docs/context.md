# Careless Project Context

Use this prompt at the start of a new chat session to get up to speed on the Careless frontend project.

---

## Prompt for New Sessions

```
Read the following files to get context on the Careless Tauri frontend project:

1. frontend-tauri/README.md - Project overview and dev commands
2. frontend-tauri/docs/todo.md - Current task list and progress
3. frontend-tauri/docs/memory.md - Development notes and decisions
4. frontend-tauri/docs/plan.md - Product vision and feature roadmap
5. frontend-tauri/changelog.md - Recent changes

Then check the task list in the current session and continue with the next pending item.
```

## Quick Reference

### Tech Stack
- **Backend**: Rust + Tauri 2.x
- **Frontend**: SvelteKit 5 + TypeScript + Vite
- **Parent Project**: whisper.cpp (C/C++ ASR engine)

### Key Commands
```bash
make dev           # Start Tauri with hot reload
make check         # Rust fmt + lint + test
make web-build     # Build frontend only
```

### Current Phase
R1 - SvelteKit Refactor: Breaking up monolithic HTML into modular components.

### Project Structure
```
frontend-tauri/
├── src/           # Rust backend
├── web/           # SvelteKit frontend
│   ├── src/routes/    # Pages
│   └── src/lib/       # Components & utilities
├── docs/          # Documentation
└── dist/          # Legacy HTML (to be removed)
```

### Workflow
- Conventional Commits for all messages
- Semantic Versioning for releases
- Always ask before committing
- Update changelog.md for user-visible changes
