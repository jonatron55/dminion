Development process establishment
==================================

**Date completed:** October 4, 2025

Objective
---------

***As an engineering team, we have an adaptable and agile process for managing work so that we can execute
efficiently.***

Tasks completed
---------------

- [X] Create a process document
  - [X] Define how items move from backlog → active tasks → done
  - [X] Establish criteria for moving backlog items to "now" readiness
  - [X] Document when and how to use completion logs
  - [X] Define lightweight review cadence for backlog prioritization
- [X] Break down project brief into manageable backlog items
- [X] Understand scope of work already done
  - [X] Audit Rust backend (src-tauri/src) and document implemented features
  - [X] Review SvelteKit frontend (src/) and map components to perspectives
  - [X] Document current database schema and data model (if implemented)
  - [X] Map existing functionality to backlog items (not started, partial, complete)

Learnings
---------

### Process design

- **Three-document workflow**: Backlog (high-level goals) → Active tasks (detailed breakdown) → Completion logs
  (learnings) creates clear separation of concerns without complexity.
- **Readiness/priority dimensions**: Using both "readiness" (now/later) and "priority" (high/med/low) helps distinguish
  between "we could do this now" and "we should do this now."
- **Instructions as prompts**: Storing process instructions in `.github/prompts/` makes them accessible to both humans
  and AI collaborators, enabling consistent workflow execution.

### Existing codebase audit

- **Substantial foundation exists**: The app has a working Tauri scaffold, complete dice parser, functional encounter
  UI with mock data, and comprehensive theming system.
- **Database schema exists but unused**: SQLite schema imported from another project exists in `game/schema.sql` but
  hasn't been adapted or connected yet.
- **Mock data enables iteration**: Using in-memory mock data in `lib.rs` allows UI development to proceed independently
  of database implementation.
- **Clean architecture**: Clear separation between Rust backend (game logic, commands) and SvelteKit frontend (views,
  components) with Tauri events for reactivity.

### Partially complete features

Three features have backend implementations but need UI work:

- Encounter tracking (UI functional but needs polish)
- Undo/redo (backend complete, needs keyboard shortcuts and visual feedback)
- Dice calculator (parser complete, needs UI component)

Reflections
-----------

- **Lightweight is better**: The initial impulse might be to create detailed task tracking, but keeping backlog items
  high-level and only breaking down active work reduces overhead while maintaining clarity.
- **Documentation audit was valuable**: Understanding what already exists before planning new work prevents duplicate
  effort and reveals opportunities to build on existing foundations.
- **Working notes are essential**: Capturing discoveries during work (like the audit findings) preserves context that
  would otherwise be lost. This paid immediate dividends when breaking down partially complete work.
- **Process as enabler**: A good process should feel like it's helping you work, not creating busywork. The test is
  whether you can hand off to someone new with confidence—we achieved that.
