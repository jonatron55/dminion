Last updated: 2025-08-17

Current focus
- Maintain developer onboarding and documentation; specifically updated `README.md` with a Getting Started guide that uses Corepack for `pnpm`, PowerShell examples, and a References section.

Workspace quick facts (captured)
- Branch: main
- Running tasks observed: `ui:dev` (pnpm dev) and `core:buildDebug` (cargo build) in the environment snapshot.
- Primary files/folders: `src/`, `src-tauri/`, `build/`, `static/`.

Recent changes
- `README.md` updated: added Getting Started, switched pnpm install instructions to use Corepack (`corepack enable` + `corepack prepare pnpm@latest --activate`), added reference-style links.

Next actions
- Keep this file updated when shifting to a new active task (e.g., feature X, bug Y).
- Add an onboarding checklist or `CONTRIBUTING.md` so new contributors can start within 30 minutes.
- Consider adding CI steps for UI + core builds and smoke tests.

Assumptions
- Developer uses Windows and PowerShell by default, but changes should remain cross-platform where possible.
