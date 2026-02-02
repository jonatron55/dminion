Active tasks
============

This document breaks down current work in progress into smaller tasks. Once a task is complete, check it off the list.
When all active tasks are complete, we can create a log in [completion-logs] folder with learnings and reflections,
reset this list, and review the [backlog]. You may encounter new tasks during implementation; extend this list as
needed.

Objective 4
-----------

***As a DM, I can undo and redo actions during encounters.***

- [ ] Verify undo/redo backend implementation
  - [ ] Review existing `undo` and `redo` commands in `game_commands.rs`
  - [ ] Confirm undo/redo stack management is correct
  - [ ] Test edge cases (empty stacks, multiple operations)
- [ ] Add keyboard shortcuts for undo/redo
  - [ ] Implement Ctrl+Z for undo
  - [ ] Implement Ctrl+Y or Ctrl+Shift+Z for redo
  - [ ] Ensure shortcuts work globally in encounter view
- [ ] Display undo/redo state in toolbar
  - [ ] Add backend query for undo/redo stack availability
  - [ ] Disable undo button when undo stack is empty
  - [ ] Disable redo button when redo stack is empty
- [ ] Add visual feedback for undo/redo operations
  - [ ] Show brief notification when undo/redo occurs
  - [ ] Update UI immediately to reflect state change
- [ ] Document undo/redo limitations
  - [ ] Note that history is in-memory only (cleared on app restart)
  - [ ] Document that undo/redo is encounter-specific

Objective 6
-----------

***As an engineering team, we have stable Rust-to-SvelteKit contracts.***

- [ ] Audit and document existing data contracts
  - [ ] Review all Tauri commands in `game_commands.rs` and `dice_commands.rs`
  - [ ] Review all TypeScript types in `src/lib/model/`
  - [ ] Identify mismatches between Rust structs and TypeScript interfaces
  - [ ] Document current command signatures and return types
- [ ] Establish contract documentation standards
  - [ ] Create `docs/reference/contracts.md` documenting all commands
  - [ ] Define naming conventions (snake_case Rust → camelCase TS)
  - [ ] Document serialization rules (serde rename attributes)
- [ ] Align Rust and TypeScript types
  - [ ] Ensure all Rust types have `#[derive(Serialize)]` with consistent `rename_all`
  - [ ] Create TypeScript interfaces matching each serialized Rust struct
  - [ ] Add discriminated unions for enums (e.g., `Action`, `Damage`, `Condition`)
- [ ] Add type generation or validation
  - [ ] Evaluate `ts-rs` crate for TypeScript type generation from Rust
  - [ ] If using `ts-rs`: add derive macros and configure output path
  - [ ] If manual: create checklist for keeping types in sync
- [ ] Refactor command layer for consistency
  - [ ] Standardize error handling across all commands
  - [ ] Use consistent argument naming (`target` vs `participantId`)
  - [ ] Document expected error responses in TypeScript

---

Working notes
-------------

*Use this section for learnings, discoveries, implementation details, and new work identified during the course of
completing active tasks.*
