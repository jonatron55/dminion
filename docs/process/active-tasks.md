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
- [x] Integrate undo/redo controls into encounter toolbar
  - [x] Add undo button with keyboard shortcut
  - [x] Add redo button with keyboard shortcut
  - [x] Implement keyboard shortcuts (Ctrl+Z for undo, Ctrl+Y/Ctrl+Shift+Z for redo)
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
- [x] Add encounter difficulty gauge
  - [x] Create DifficultyGauge Svelte component
    - [x] Visual bar with threshold markers (Trivial, Easy, Medium, Hard, Deadly)
    - [x] Fill indicator showing current adjusted XP position (clips at deadly)
    - [x] XP labels at threshold boundaries
  - [x] Add summary stats display below gauge
    - [x] Total XP (sum of monster XP)
    - [x] Player count
    - [x] Monster count
    - [x] Adjusted XP (with encounter multiplier)
    - [x] XP per player
    - [x] Average CR
  - [x] Implement frontend difficulty calculations
    - [x] Add XP_PER_CR and DIFFICULTIES_PER_LEVEL lookup tables
    - [x] Calculate party thresholds from player levels
    - [x] Calculate encounter multiplier based on monster count
    - [x] Compute adjusted XP and current difficulty rating
  - [x] Integrate gauge into encounter page footer
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

Objective 5
-----------

***As an engineering team, we have a SQLite data model supporting campaigns.***

- [ ] Define campaign database schema
  - [ ] Create `schema.sql` with all core tables (extend existing draft in `src-tauri/src/game/schema.sql`)
  - [ ] Add `Campaign` table with name, creation date, and metadata
  - [ ] Add `Maps` table for map images and metadata
  - [ ] Add `Items` table for item library entries
  - [ ] Add `EncounterSnapshots` table for savepoint storage
  - [ ] Document table relationships and constraints
- [ ] Integrate SQLite with Tauri app
  - [ ] Add `sqlx` crate with SQLite feature to `Cargo.toml`
  - [ ] Create database connection management in a new `db.rs` module
  - [ ] Implement campaign file creation (one `.db` file per campaign)
  - [ ] Add database path resolution using Tauri's app data directory
- [ ] Implement core CRUD operations for monsters
  - [ ] Create `db/monsters.rs` module with query functions
  - [ ] Implement `insert_monster`, `get_monster`, `list_monsters`, `update_monster`, `delete_monster`
  - [ ] Add proper error handling and return types
- [ ] Implement core CRUD operations for encounters
  - [ ] Create `db/encounters.rs` module with query functions
  - [ ] Implement encounter persistence including participant snapshots
  - [ ] Support encounter savepoint creation and restoration
- [ ] Add database migration support
  - [ ] Create `migrations/` folder structure
  - [ ] Implement version tracking table
  - [ ] Add migration runner for schema updates

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

### Objective 5 baseline

Existing draft schema at `src-tauri/src/game/schema.sql` provides a starting point with:

- `Players`, `Classes`, `Parties` tables for player/party management
- `Monsters` table with full stat blocks
- `Encounters`, `EncounterMonsters` tables for encounter composition
- `TradeItems` table for trade system

Needs additions: `Campaign` metadata table, `Maps` table, `Items` library table, `EncounterSnapshots` for savepoints, and proper foreign key constraints with `ON DELETE` behavior.

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

### Difficulty gauge SRD 5.1 vs 5.2 implementation

`DifficultyGauge.svelte` supports both SRD 5.1 and SRD 5.2 difficulty calculations, switching based on `currentRulesVersion`.

**SRD 5.1 (2014 rules)**:

- Thresholds: Easy, Medium, Hard, Deadly (per-player XP budgets summed for party)
- Ratings: Trivial (below Easy), Easy, Medium, Hard, Deadly
- Uses *adjusted XP* with encounter multiplier based on monster count (1×–4× multiplier)
- Gauge fills based on adjusted XP relative to deadly threshold

**SRD 5.2 (2024 rules)**:

- Thresholds: Low, Moderate, High (simplified budgets)
- Ratings: Trivial (below Low), Low, Moderate, High
- Uses *total XP* directly without encounter multiplier
- Gauge fills based on total XP relative to high threshold

The component uses type guards (`is51Thresholds`) to safely access the different threshold structures and renders appropriate labels/markers for each rules version.

[backlog]: /docs/process/backlog.md
[completion-logs]: /docs/process/completion-logs.md
