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

---

Working notes
-------------

*Use this section for learnings, discoveries, implementation details, and new work identified during the course of
completing active tasks.*
