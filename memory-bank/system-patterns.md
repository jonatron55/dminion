Architecture overview
- Two-tier app: Svelte UI (renderer) + Rust/Tauri core (native shell & features).
- Communication: Tauri commands/events or message channels; small surface area of native APIs.

Design patterns & conventions
- MVVM-ish structure in `lib/model` and `lib/viewmodel` for UI state.
- Keep UI-only logic in `src/` and platform-specific code in `src-tauri/`.
- Assets: place images and fonts in `images/`, `fonts/` and reference via build pipeline.

Coding & review norms
- Prefer small, focused PRs that update memory-bank when architectural choices are made.
- Update `active-context.md` with the current focus when starting a new task/feature.
- Note breaking decisions (public APIs, data formats) in this file with date and author.

Testing
- Keep unit-level logic in TypeScript or Rust with clear test files where appropriate.
- Prefer small smoke tests for UI routes and a fast verification step after builds.
