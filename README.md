<img src="src/images/icon.svg" alt="Dungeon Minion Icon" style="width: 128px; height: 128px; margin: 24px auto; display: block;"/>

Dungeon Minion
==============

Dungeon Minion is a tabletop RPG companion app designed to:
- Track encounters and initiative.
- Present maps and images.
- Track trade items in transactions.
- Generate character names.
- Simulate dice rolls.

Getting started
---------------

This project uses a web UI built with [Node.js] tooling ([pnpm], [Vite], [Svelte]) and a [Rust]/[Tauri] backend. The instructions below target Windows (PowerShell).

### Prerequisites ###
- [Node.js] (LTS) and npm (bundled with Node)
- [pnpm] (recommended package manager)
- [Rust] (rustup + cargo)
- [Tauri] CLI (optional, for running/packaging via Tauri)
- Platform build tools (Windows: [Visual Studio] with C++ build tools / MSVC)

### Installation ###

Once you have the [prerequisites](#prerequisites) installed, you can set up the project by following these steps:

```powershell
# Use Corepack to manage pnpm
corepack enable
corepack prepare pnpm@latest --activate

# Verify pnpm is available
pnpm --version

# Update Rust
rustup update

# Install project dependencies
pnpm install

# Build backend (a clean build will take a few minutes)
cargo build --manifest-path=.\src-tauri\Cargo.toml
```

### Running the app for development ###

To build the backend, start a dev server, and launch the app all in one step, run:

```powershell
pnpm tauri dev
```

To instead run the server in a separate terminal, use:

```powershell
pnpm dev
```

To launch the app, use:

```powershell
cargo run --manifest-path=.\src-tauri\Cargo.toml
```

[Node.js]: https://nodejs.org/
[pnpm]: https://pnpm.io/
[Rust]: https://www.rust-lang.org/
[Svelte]: https://svelte.dev/
[Tauri]: https://tauri.app/
[Visual Studio]: https://visualstudio.microsoft.com/
[Vite]: https://vitejs.dev/
