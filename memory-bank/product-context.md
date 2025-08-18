Users & problems
- Primary users: repository maintainers and contributors working on the desktop app.
- Problems solved: give maintainers a fast onboarding reference, design intent, and decisions so contributors can make consistent changes.

Key features (high level)
- Local-first UI implemented with Svelte (+Vite). Live development with `pnpm dev`.
- Native shell and capabilities provided by a Rust/Tauri backend (`src-tauri/`).
- Asset pipeline and build output live under `build/`, `static/`, and `src-tauri/target/`.

Measures of success
- New contributors can start the app locally within 30 minutes following README instructions.
- CI builds succeed for UI and core; packaging pipeline produces installable artifacts.
- Low friction for adding new UI pages or native commands.
