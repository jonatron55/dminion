Project: dminion

Short description
- dminion is a desktop application combining a Svelte-based UI and a Rust/Tauri core.
- Purpose: provide a local-first, responsive tool (game/utility) with a compact native wrapper.

Goals & success criteria
- Fast, cross-platform desktop UX using Tauri and Svelte.
- Clear separation between UI and core logic so features are testable and portable.
- Builds reproducibly via pnpm (UI) and cargo (core). CI-friendly build steps.
- Deliverables: working dev environment (pnpm dev, cargo build), packaged app artifacts for Windows/macOS/Linux.

Boundaries
- This memory bank covers engineering context, not user data or secrets.
- Keep entries concise and focused on actionable developer information.
