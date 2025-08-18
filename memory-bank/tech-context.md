Tech stack
- UI: Svelte (SvelteKit/Vite-like structure), TypeScript, pnpm package manager.
- Native/Backend: Rust + Tauri (`src-tauri/`), Cargo for builds.
- Tooling: pnpm scripts, vite, tauri.conf.json, cargo build tasks.

Repository layout (important dirs)
- `src/` — Svelte source: components, routes, styles, lib, images.
- `src-tauri/` — Rust integration, Tauri configuration, Cargo.toml.
- `build/` & `static/` — compiled front-end artifacts and static assets.
- `memory-bank/` — (this directory) persistent project context for Copilot/maintainers.

Local dev commands (observed in workspace)
- UI dev: `pnpm dev` (task: `ui:dev`).
- UI build: `pnpm build` (task: `ui:build`).
- Core debug build: `cargo build --manifest-path=./src-tauri/Cargo.toml` (task: `core:buildDebug`).

Environments & constraints
- Primary tested OS: Windows (developer environment shown). Target: cross-platform via Tauri.
- Avoid storing secrets in memory bank. Document environment variables and build expectations here only.
