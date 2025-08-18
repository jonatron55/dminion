Last updated: 2025-08-17

What works
- Repository structure is present and build tasks are wired (`pnpm` for UI, `cargo` for core).
- `README.md` now contains a Getting Started guide with Corepack-based `pnpm` instructions and reference links.

What remains / TODO
- Add curated design decisions to `system-patterns.md` when changes occur.
- Add CI configuration that builds UI and core and runs smoke tests (suggest GitHub Actions matrix for platforms).

Known issues / risks
- No runtime/build issues currently recorded; monitor CI for platform-specific linker/tooling failures (Windows/MSVC, OpenSSL etc.).

Suggested next steps (short checklist)
- [ ] Add `CONTRIBUTING.md` with quick-start steps and preferred local tasks.
- [ ] Create a small GitHub Actions workflow that runs `pnpm build` and `cargo build`.
- [ ] Document minimum Node version required for Corepack in `README.md`.
