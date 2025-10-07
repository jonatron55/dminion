Active tasks
============

This document breaks down current work in progress into smaller tasks. Once a task is complete, check it off the list.
When all active tasks are complete, we can create a log in [completion-logs] folder with learnings and reflections,
reset this list, and review the [backlog]. You may encounter new tasks during implementation; extend this list as needed.

Objective 3
-----------

***As a DM, I can run encounters with initiative, health, and condition tracking.***

- [x] Review and polish existing encounter UI components
  - [x] Test ParticipantRow display with various participant states
  - [x] Verify condition display shows all relevant information
  - [x] Ensure HP bars and stats render correctly
  - [x] Test damage and heal dialogs for edge cases
  - [x] Fix damage to properly handle temporary HP (subtract from temp HP first)
  - [x] Fix action checkboxes to show checked when action is used (inverted logic)
- [ ] Integrate undo/redo controls into encounter toolbar
  - [x] Add undo button with keyboard shortcut
  - [x] Add redo button with keyboard shortcut
  - [x] Implement keyboard shortcuts (Ctrl+Z for undo, Ctrl+Y/Ctrl+Shift+Z for redo)
  - [ ] Display undo/redo state (enabled/disabled based on stack)
- [x] Add next turn controls to encounter toolbar
  - [x] Implement "Next Turn" button
  - [x] Update active participant highlighting
  - [x] Advance round counter when initiative cycles
- [x] Implement encounter time tracking display
  - [x] Show current round number
  - [x] Show current turn time
  - [x] Display game elapsed time
  - [x] Fix timer reactivity to update when round changes
- [ ] Add lair action support to UI
  - [x] Display lair participants in initiative order
  - [x] Show lair action indicators
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

### Fixes applied for Objective 3 (objectives 2 and 3)

**Timer reactivity issue**: The timer in `EncounterToolbar.svelte` was not reactive. Changed from a function call `time()` to a reactive statement `$: timeStr = (() => {...})()` so it updates when `game.round` changes.

**Damage ignoring temporary HP**: The `damage()` function in `monster.rs` was directly subtracting from HP without checking temp HP first. Updated to:
1. Calculate total damage amount based on damage type
2. Apply damage to temp HP first if available
3. Apply remaining damage to regular HP
4. Properly handle the `Kill` damage type to clear both HP and temp HP

**Action checkbox logic inverted**: The checkboxes were bound directly to the action boolean, but they should show as checked when the action has been used (i.e., when `action` is `false`). Changed from `bind:checked={monster.action}` to:
- `checked={!monster.action}` to invert the display
- `on:change={(e) => monster.action = !e.currentTarget.checked}` to invert the value on change
- Applied to Monster, Player, and Lair action/bonus/reaction checkboxes

**Action checkbox binding to Rust**: Added full backend binding for action state management:
1. Created `Action` enum in `participant.rs` (moved from `damage.rs` as it's participant-related)
   - Variants: `Standard`, `Bonus`, `Reaction`, `Legendary { index: usize }`
2. Added `legendary_actions: Vec<bool>` to Monster to track individual legendary action usage
3. Added `set_action(action: Action, available: bool) -> Result<(), ()>` methods to Monster, Player, Lair, and Participant
   - Returns `Err(())` for invalid action types (e.g., Legendary for Players/Lairs)
4. Created `set_action` Tauri command in `game_commands.rs` with proper error handling
5. Updated TypeScript `Action` type as discriminated union: `{ type: "standard" }`, `{ type: "bonus" }`, `{ type: "reaction" }`, `{ type: "legendary", index: number }`
6. Updated MonsterViewModel, PlayerViewModel, and LairViewModel:
   - Changed setters to use Action objects (e.g., `{ type: "standard" }`)
   - Added `setLegendaryAction(index: number, value: boolean)` to MonsterViewModel
   - Changed `legendaryActions` getter to return `boolean[]` array
7. Updated ParticipantRow to iterate over legendary actions array with proper inverted checkbox logic

Action state now properly syncs between UI and Rust backend through undo/redo system with type-safe error handling.

All three fixes tested and working. Actions now properly reset at the beginning of a participant's turn via the existing `begin_turn()` implementation.

[backlog]: /docs/process/backlog.md
[completion-logs]: /docs/process/completion-logs.md
