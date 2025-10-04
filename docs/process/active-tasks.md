Active tasks
============

This document breaks down current work in progress into smaller tasks. Once a task is complete, check it off the list.
When all active tasks are complete, we can create a log in [completion-logs] folder with learnings and reflections,
reset this list, and review the [backlog].

Objective 3
-----------

***As a DM, I can run encounters with initiative, health, and condition tracking.***

- [ ] Review and polish existing encounter UI components
  - [ ] Test ParticipantRow display with various participant states
  - [ ] Verify condition display shows all relevant information
  - [ ] Ensure HP bars and stats render correctly
  - [ ] Test damage and heal dialogs for edge cases
- [ ] Integrate undo/redo controls into encounter toolbar
  - [ ] Add undo button with keyboard shortcut
  - [ ] Add redo button with keyboard shortcut
  - [ ] Display undo/redo state (enabled/disabled based on stack)
- [ ] Add next turn controls to encounter toolbar
  - [ ] Implement "Next Turn" button
  - [ ] Update active participant highlighting
  - [ ] Advance round counter when initiative cycles
- [ ] Implement encounter time tracking display
  - [ ] Show current round number
  - [ ] Show current turn time
  - [ ] Display game elapsed time
- [ ] Add lair action support to UI
  - [ ] Display lair participants in initiative order
  - [ ] Show lair action indicators
- [ ] Test full encounter workflow end-to-end
  - [ ] Create encounter, add participants, run through multiple rounds
  - [ ] Verify all HP changes, conditions, and state updates work correctly
  - [ ] Test undo/redo through complex state changes

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
- [ ] Add visual feedback for undo/redo operations
  - [ ] Show brief notification when undo/redo occurs
  - [ ] Update UI immediately to reflect state change
- [ ] Document undo/redo limitations
  - [ ] Note that history is in-memory only (cleared on app restart)
  - [ ] Document that undo/redo is encounter-specific

Objective 5
-----------

***As a DM, I can use a dice calculator for complex rolls.***

- [ ] Design dice calculator UI component
  - [ ] Create sidebar panel or dialog for dice calculator
  - [ ] Add input field for dice expressions
  - [ ] Design result display area
  - [ ] Add roll history display
- [ ] Integrate dice parser backend with UI
  - [ ] Wire up input field to `roll` command
  - [ ] Handle and display roll results
  - [ ] Show breakdown of roll components (individual dice, modifiers)
  - [ ] Display parse errors clearly when expression is invalid
- [ ] Add dice calculator to sidebar or toolbar
  - [ ] Add button/icon to open dice calculator
  - [ ] Implement show/hide toggle
  - [ ] Position calculator appropriately in layout
- [ ] Add common dice presets
  - [ ] Quick buttons for d20, d12, d10, d8, d6, d4
  - [ ] Advantage/disadvantage buttons for d20 rolls
  - [ ] Modifier input for quick adjustments
- [ ] Test dice calculator with complex expressions
  - [ ] Test multi-dice rolls (3d6+2d8+5)
  - [ ] Test advantage/disadvantage syntax
  - [ ] Verify all operators work correctly

---

Working notes
-------------

*Use this section for learnings, discoveries, implementation details, and new work identified during the course of
completing active tasks.*

[backlog]: /docs/process/backlog.md
[completion-logs]: /docs/process/completion-logs.md
