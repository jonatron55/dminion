---
applyTo: "**/*.{rs,toml}"
---

Rust coding guidelines
======================

The Rust backend exists under `src-tauri/src/`. It uses the Tauri framework to interface with the TypeScript frontend.

General style
-------------

- Import items with `use` statements at the top of each file. Avoid fully qualified names in code. If a name conflicts,
  use `as` to rename it on import (e.g., `use std::io::Error as IoError;`).
- Group `use` statements by standard library imports, external crate imports, and internal module imports, with a blank
  line between each group.
- Use `#[serde(rename_all = "camelCase")]` on structs serialized to or from JSON, particularly Tauri commands and
  events sent to the frontend.
- `unsafe` should not be required in this project. **Do not** use `unsafe` in code you write.
- Use builders for complex object creation.
- Use iterators and iterator adapters (e.g., `map`, `filter`, `collect`) instead of loops when possible.

Borrowing and ownership
-----------------------

Rust's ownership and borrowing model provides performant code while ensuring memory safety.

- Prefer references (`&T`, `&mut T`) over ownership (`T`, `mut T`) when possible.
- Prefer `&str` instead of `String` and `&[T]` instead of `Vec<T>` when possible.
- Avoid cloning data unless a copy is actually needed.
- Use `Cow<T>` in situations where cloning is only sometimes needed.
- Use `Rc<T>` or `Arc<T>` for shared ownership when necessary.
- Use `RefCell<T>` or `Mutex<T>` for interior mutability when needed.

Error handling
--------------

Propagate errors using `Result<T, E>`. Use the `thiserror` crate to define error types and allow many potential errors
to be handled with the `?` operator. Use functions like `map_err()` and `and_then()` or constructions like `if let ...`
and `let ... else` to simplify error handling.

Avoid silently handling errors with inaction. If a function's contract is not met, modify the function to return a
`Result`. Do not use `panic!()`, `unwrap()`, or `expect()` outside the situations described below.

### When to panic ###

The `panic!()` macro and related `unwrap()` and `expect()` functions terminate the program immediately. Use them only
when recovery is impossible or impractical.

Appropriate uses:

- Code that runs at program startup or shutdown, such as in `Drop`.
- Primitive threading operations, such as spawning or joining threads, acquiring locks, or sending on channels.
- Situations that should be logically unreachable at runtime (use the `unreachable!()` macro).
- Incomplete or unimplemented functionality (use the `todo!()` or `unimplemented!()` macros).
- Test code, where panicking indicates a failed test.

Dependencies
------------

**Do not add new dependencies without approval.** Explain why new dependencies are needed.

When adding or modifying dependencies:

- Use `cargo` commands to manage crates. Avoid modifying `Cargo.toml` directly.
- Do not specify crate versions unless necessary. Use the latest version.
- Review existing dependencies before adding new ones. Avoid duplicating functionality.
