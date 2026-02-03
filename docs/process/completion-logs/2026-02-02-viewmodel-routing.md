As an engineering team, Views route commands through ViewModels
====================================================================

**Date completed:** February 2, 2026

Tasks completed
---------------

- [x] Add game control methods to GameViewModel
  - [x] Add `nextTurn()` method to GameViewModel
  - [x] Add `undo()` method to GameViewModel
  - [x] Add `redo()` method to GameViewModel
  - [x] Add `canUndo` property to GameViewModel (deferred)
  - [x] Add `canRedo` property to GameViewModel (deferred)
- [x] Refactor EncounterToolbar to use GameViewModel
  - [x] Replace `gameCommands.nextTurn()` with `game.nextTurn()`
  - [x] Replace `gameCommands.undo()` with `game.undo()`
  - [x] Replace `gameCommands.redo()` with `game.redo()`
  - [x] Disable undo button when `!game.canUndo` (deferred)
  - [x] Disable redo button when `!game.canRedo` (deferred)
- [x] Add condition methods to ParticipantViewModel or ConditionDialogViewModel
  - [x] Decide whether `addConditions()` belongs on ParticipantViewModel or ConditionDialogViewModel
  - [x] Implement `addConditions(conditions: Condition[])` method
  - [x] Update method to call `gameCommands.addConditions()` internally
- [x] Refactor ConditionDialog to use ViewModel method
  - [x] Replace `gameCommands.addConditions()` with ViewModel method call
  - [x] Verify error handling is consistent with other dialogs
- [x] Create DiceViewModel for dice rolling
  - [x] Create `src/lib/viewmodel/DiceViewModel.ts`
  - [x] Add `roll(expr: string)` method returning `Promise<Roll>`
  - [x] Add error handling consistent with other ViewModels
  - [x] Add `history` property to track roll results
- [x] Refactor DicePanel to use DiceViewModel
  - [x] Instantiate DiceViewModel in DicePanel
  - [x] Replace `diceCommands.roll()` with `diceViewModel.roll()`
  - [x] Move history state into DiceViewModel if appropriate
- [x] Verify error handling consistency
  - [x] Ensure all ViewModel methods catch and handle errors appropriately
  - [x] Confirm error messages reach MessageBox or user-facing UI
  - [x] Test error scenarios for each refactored component

Learnings
---------

### Component architecture decisions

**Added `addConditions()` to ParticipantViewModel base class** rather than ConditionDialogViewModel. Since conditions apply to participants (not to the dialog), this keeps the API surface focused. All participant types can have conditions except Lair, which is validated in the backend.

**Created DiceViewModel as standalone ViewModel** rather than integrating into GameViewModel. Dice rolling is independent of game state and used in multiple contexts, so separation maintains cohesion.

**Kept error handling centralized in Commands.ts** rather than duplicating it in each ViewModel. The `tryInvoke()` function provides consistent MessageBox error display for all command failures. ViewModels simply call through to commands and let errors propagate naturally.

### Svelte reactivity with ViewModels

**Class instance properties aren't reactive by default.** Initial implementation had DicePanel directly referencing `diceViewModel.history`, but Svelte doesn't track changes to object properties. Solution: maintain a local reactive variable (`let history: HistoryItem[]`) and explicitly update it after ViewModel operations (`history = diceViewModel.history`).

**Immutable array updates aren't sufficient alone.** The ViewModel uses `this.history = [...this.history, newItem]` for immutability, but the component still needs to explicitly assign the updated reference to trigger reactivity.

### Deferred work rationale

**`canUndo` and `canRedo` properties require backend support.** The backend currently returns errors when stacks are empty but doesn't expose stack availability as queryable state. Adding these properties would require new Tauri commands like `can_undo()` and `can_redo()`. Current implementation is functional—buttons just remain enabled and show error dialogs if clicked when unavailable. This is acceptable UX for now and can be improved when working on Objective 4 (undo/redo UI enhancements).

Reflections
-----------

### What worked well

**Audit-first approach prevented scope creep.** Starting with a comprehensive audit of which components needed refactoring prevented over-engineering and kept focus on actual violations of the pattern.

**Existing MonsterViewModel provided clear precedent.** DamageDialog and HealDialog already demonstrated the correct pattern, making it straightforward to apply the same approach elsewhere.

**Small, focused ViewModels are easier to reason about.** Each ViewModel has a clear responsibility: GameViewModel manages game state queries and mutations, ParticipantViewModel handles participant operations, DiceViewModel manages roll history.

### What to improve

**Document Svelte reactivity patterns for ViewModels.** The reactivity issue with DicePanel could have been avoided with upfront documentation about how to properly integrate class instances with Svelte's reactivity system. Should add this to coding standards or create a ViewModel guide.

**Consider using Svelte stores for ViewModel state.** An alternative approach would be to use Svelte stores within ViewModels for reactive properties. This would eliminate the need for manual updates like `history = diceViewModel.history`. Trade-off: adds dependency on Svelte-specific APIs within ViewModels, reducing testability.

**Test ViewModels independently.** Now that command invocation is centralized in ViewModels, we should add unit tests for ViewModel methods to verify they call the correct commands with correct arguments. This would catch regressions during refactoring.

New work identified
-------------------

Added to backlog:

- None at this time

Deferred for future consideration:

- Add `can_undo()` and `can_redo()` Tauri commands to support button disable states (relates to Objective 4)
- Create coding standards documentation for integrating ViewModels with Svelte reactivity
- Consider using Svelte stores for ViewModel reactive state
- Add unit tests for ViewModel methods
