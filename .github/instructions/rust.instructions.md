---
applyTo: "**/*.{rs,toml}"
---

Rust coding guidelines
======================

The Rust backend lives in `src-tauri/src/` and interfaces with the TypeScript frontend through the Tauri framework.

General style
-------------

- Place all `use` statements at the top of each file.
  - Avoid fully qualified paths in code.
  - Resolve naming conflicts with `as` (e.g., `use std::io::Error as IoError;`).
- Group imports in this order, with a blank line between groups:
  1. Standard library
  2. External crates
  3. Internal modules
- Apply `#[derive(Debug)]` to all structs and enums unless there is a specific reason not to.
- Apply `#[serde(rename_all = "camelCase")]` to structs serialized to or from JSON, especially Tauri commands and
  events.
- Do not use `unsafe`. This project should not require it.
- Use builders for complex object construction.
- Prefer iterators and adapters (`map`, `filter`, `collect`, etc.) over manual loops when it improves clarity.

Borrowing and ownership
-----------------------

Rust’s ownership model ensures memory safety and performance. Follow these practices:

- Prefer references (`&T`, `&mut T`) over owned values (`T`, `mut T`) when possible.
- Prefer `&str` over `String` and `&[T]` over `Vec<T>` when borrowing is sufficient.
- Avoid cloning unless the data must be duplicated.
- Use `Cow<T>` when cloning is only conditionally required.
- Use `Rc<T>` or `Arc<T>` for shared ownership.
- Use `RefCell<T>` or `Mutex<T>` for interior mutability when needed.

Error handling
--------------

- Propagate errors with `Result<T, E>`.
- Use the `thiserror` crate to define error types.
- Use the `?` operator to simplify propagation.
- Use combinators like `map_err()` and `and_then()` or control-flow constructs (`if let`, `let … else`) to keep error
  handling concise.
- Do not silently handle errors with inaction. If a function can fail, modify it to return a `Result`.
- Do not use `panic!()`, `unwrap()`, `expect()`, or similar, except in the cases below.

### When to panic ###

The `panic!()` macro and functions that call it (`unwrap()`, `expect()`, etc.) terminate the program abruptly. Use them
only when recovery is impossible or impractical. For example:

- Startup or shutdown code (e.g., inside `Drop`).
- Primitive threading operations (spawning/joining threads, acquiring locks, sending on channels).
- Logically unreachable states (`unreachable!()`).
- Unimplemented or placeholder functionality (`todo!()`, `unimplemented!()`).
- Assertions in test code.

Dependencies
------------

**Do not add new dependencies without approval.** Explain why new dependencies are needed.

When adding or modifying dependencies:

- Use `cargo` commands to manage crates. Avoid modifying `Cargo.toml` directly.
- Do not pin versions unless required; allow `cargo` to resolve the latest compatible version.
- Review existing dependencies before adding new ones to avoid duplication.
