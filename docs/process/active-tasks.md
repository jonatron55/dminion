Active tasks
============

This document breaks down current work in progress into smaller tasks. Once a task is complete, check it off the list.
When all active tasks are complete, we can create a log in [completion-logs] folder with learnings and reflections,
reset this list, and review the [backlog]. You may encounter new tasks during implementation; extend this list as
needed.

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

- [ ] Implement campaign folder and settings management
  - [ ] Create `config/paths.rs` for `ProjectDirs` integration and campaign folder resolution
  - [ ] Create `config/campaign_settings.rs` for per-campaign `settings.toml` (name, schema version, created date,
        theme, rules version)
  - [ ] Create `config/app_settings.rs` for app-wide `settings.toml` in `preference_dir`
  - [ ] Implement campaign folder creation with subfolders (`saves/`, `portraits/`, `maps/`)
  - [ ] Implement folder naming (snake-case, 16-char limit, numeric suffix for disambiguation)
- [ ] Define campaign database schema
  - [ ] Create `schema.sql` with all core tables (extend existing draft in `src-tauri/src/game/schema.sql`)
  - [ ] Add `Maps` table for map metadata (image path references external file)
  - [ ] Add `Items` table for item library entries
  - [ ] Update `portrait` field to store base name only (resolution handled by service)
  - [ ] Document table relationships and constraints with `ON DELETE` behavior
- [ ] Integrate SQLite with Tauri app
  - [ ] Add `sqlx` crate with SQLite feature to `Cargo.toml`
  - [ ] Create `db/connection.rs` for `SqlitePool` management per campaign
  - [ ] Implement campaign open/close lifecycle (load settings, open DB, release on close)
  - [ ] Add database path resolution using campaign folder path
- [ ] Implement portrait resolution service
  - [ ] Create `services/portrait.rs` with prioritized search (campaign → app → placeholder)
  - [ ] Support `.small` and `.full` size variants
  - [ ] Handle missing files and broken symlinks gracefully
- [ ] Implement encounter savepoint service
  - [ ] Create `services/savepoint.rs` for TOML serialization of complete encounter state
  - [ ] Implement periodic savepoint writes to `saves/turn-NNN.toml`
  - [ ] Implement savepoint restoration on app startup if encounter was in progress
- [ ] Implement core CRUD operations for monsters
  - [ ] Create `db/monsters.rs` module with query functions
  - [ ] Implement `insert_monster`, `get_monster`, `list_monsters`, `update_monster`, `delete_monster`
  - [ ] Add proper error handling and return types
- [ ] Add database migration support
  - [ ] Create `migrations/` folder structure
  - [ ] Read schema version from `settings.toml`, run migrations, update version
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

### Objective 5 baseline ###

Existing draft schema at `src-tauri/src/game/schema.sql` provides a starting point with:

- `Players`, `Classes`, `Parties` tables for player/party management
- `Monsters` table with full stat blocks
- `Encounters`, `EncounterMonsters` tables for encounter composition
- `TradeItems` table for trade system

Needs additions: `Maps` table (metadata only, images stored externally), `Items` library table, and proper foreign key
constraints with `ON DELETE` behavior. Portrait fields should store base name only.

### Architecture decisions for Objective 5 ###

**Campaign metadata**: Stored in `settings.toml` per campaign folder, not in the database. This keeps DB files portable
and allows campaign inspection without opening the database.

**Single schema version**: One version in `settings.toml` covers both folder layout and database schema. On startup:
read version → run migrations → update version.

**Savepoints**: Stored as complete encounter state in `saves/turn-NNN.toml` files, not in the database. High-frequency
writes that are discarded after encounter ends. Full snapshots enable recovery if app closes unexpectedly.

**Portrait resolution**: Database stores only the base name (e.g., `goblin`). A service resolves the actual file using
prioritized search:

1. Campaign `portraits/` folder: exact match for requested size (`.small` or `.full`)
2. Campaign `portraits/` folder: alternate size as fallback
3. App installation `portraits/` folder: same search order
4. Built-in placeholder image if no match found

**One campaign at a time**: Only one campaign is open per app instance. Simplifies state management.

**Undo/redo**: Full encounter snapshots in memory. Acceptable for typical encounter sizes and aligns with savepoint
strategy.

**Backend layering**:

- `commands/` — Thin Tauri command handlers (validate, delegate, emit)
- `domain/` — Pure types, no I/O (encounter participants, conditions, damage)
- `services/` — Business logic (encounter state, undo/redo, portrait resolution, savepoints)
- `db/` — Data access layer (sqlx queries)
- `config/` — Settings and path resolution

### Settings file formats ###

**Campaign settings** (`<campaign>/settings.toml`):

```toml
# Campaign metadata
name = "Dragon Heist"
created = 2026-01-15T10:30:00Z
schema_version = "1.0.0"  # Semver for folder + DB schema

# Rules configuration
[rules]
version = "5.1"           # "5.1" or "5.2"
monster_hp = "rolled"     # "rolled" or "fixed"
# Future: variant rules, homebrew toggles

# UI preferences (per-campaign for different "feels")
[ui]
theme = "arcane"
theme_mode = "dusk"
font_size = "medium"
```

**App-wide settings** (`<preference_dir>/settings.toml`):

```toml
# Recently opened campaigns (most recent first)
recent_campaigns = [
  "C:/Users/dm/AppData/Local/dungeon-minion/dragon-heist",
  "C:/Users/dm/AppData/Local/dungeon-minion/inn-at-the-crossroads",
]

# Window state persistence
[window.main]
x = 100
y = 100
width = 1280
height = 720
maximized = false

[window.player]
x = 1400
y = 100
width = 800
height = 600
maximized = false

# Savepoint behavior
[savepoints]
trigger = "turn"          # "turn" or "round"
max_count = 50            # Older savepoints pruned when exceeded

# Future: storage paths, backup settings, etc.
```

### Schema versioning ###

Use semver (`major.minor.patch`) for schema versions:

- **Major**: Breaking changes requiring data migration (e.g., table restructure)
- **Minor**: Additive changes (new tables, new nullable columns)
- **Patch**: Non-structural fixes (index changes, constraint tweaks)

Use `semver` crate in Rust for parsing and comparison. Migration runner checks current version against target and runs
appropriate migrations in order.

### Savepoint strategy ###

- **Trigger**: Configurable in app settings (`turn` or `round`)
- **Limit**: `max_count` setting; prune oldest when exceeded
- **Format**: Complete encounter state as TOML (not deltas)
- **Naming**: `turn-NNN.toml` where NNN is zero-padded turn number
- **Restoration**: On app startup, check for savepoints; prompt user to restore if found

### Error handling ###

Use `thiserror` for component-focused error types:

```rust
// config/error.rs
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read settings: {0}")]
    Read(#[from] std::io::Error),
    #[error("invalid settings format: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("campaign folder already exists: {0}")]
    FolderExists(PathBuf),
}

// db/error.rs
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("monster not found: {0}")]
    MonsterNotFound(i64),
}

// services/error.rs
#[derive(Debug, thiserror::Error)]
pub enum EncounterError {
    #[error("participant not found: {0}")]
    ParticipantNotFound(usize),
    #[error("invalid action for participant type")]
    InvalidAction,
}
```

Commands convert these to `String` for Tauri's error handling, or use a unified `AppError` if needed.

### Template databases ###

Deferred to a later iteration. For now, campaigns start with an empty database created from `schema.sql`. When ready,
template databases (`srd-5.1.sqlite3`, `srd-5.2.1.sqlite3`) will be created by converting the official SRD documents,
which are licensed CC-BY.

### Fixes applied for Objective 3 (objectives 2 and 3) ###

**Timer reactivity issue**: The timer in `EncounterToolbar.svelte` was not reactive. Changed from a function call
`time()` to a reactive statement `$: timeStr = (() => {...})()` so it updates when `game.round` changes.

**Damage ignoring temporary HP**: The `damage()` function in `monster.rs` was directly subtracting from HP without
checking temp HP first. Updated to:

1. Calculate total damage amount based on damage type
2. Apply damage to temp HP first if available
3. Apply remaining damage to regular HP
4. Properly handle the `Kill` damage type to clear both HP and temp HP

**Action checkbox logic inverted**: The checkboxes were bound directly to the action boolean, but they should show as
checked when the action has been used (i.e., when `action` is `false`). Changed from `bind:checked={monster.action}` to:

- `checked={!monster.action}` to invert the display
- `on:change={(e) => monster.action = !e.currentTarget.checked}` to invert the value on change
- Applied to Monster, Player, and Lair action/bonus/reaction checkboxes

**Action checkbox binding to Rust**: Added full backend binding for action state management:

1. Created `Action` enum in `participant.rs` (moved from `damage.rs` as it's participant-related)
   - Variants: `Standard`, `Bonus`, `Reaction`, `Legendary { index: usize }`
2. Added `legendary_actions: Vec<bool>` to Monster to track individual legendary action usage
3. Added `set_action(action: Action, available: bool) -> Result<(), ()>` methods to Monster, Player, Lair, and
   Participant
   - Returns `Err(())` for invalid action types (e.g., Legendary for Players/Lairs)
4. Created `set_action` Tauri command in `game_commands.rs` with proper error handling
5. Updated TypeScript `Action` type as discriminated union: `{ type: "standard" }`, `{ type: "bonus" }`,
   `{ type: "reaction" }`, `{ type: "legendary", index: number }`
6. Updated MonsterViewModel, PlayerViewModel, and LairViewModel:
   - Changed setters to use Action objects (e.g., `{ type: "standard" }`)
   - Added `setLegendaryAction(index: number, value: boolean)` to MonsterViewModel
   - Changed `legendaryActions` getter to return `boolean[]` array
7. Updated ParticipantRow to iterate over legendary actions array with proper inverted checkbox logic

Action state now properly syncs between UI and Rust backend through undo/redo system with type-safe error handling.

All three fixes tested and working. Actions now properly reset at the beginning of a participant's turn via the existing
`begin_turn()` implementation.

### Difficulty gauge SRD 5.1 vs 5.2 implementation ###

`DifficultyGauge.svelte` supports both SRD 5.1 and SRD 5.2 difficulty calculations, switching based on
`currentRulesVersion`.

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

The component uses type guards (`is51Thresholds`) to safely access the different threshold structures and renders
appropriate labels/markers for each rules version.

[backlog]: /docs/process/backlog.md
[completion-logs]: /docs/process/completion-logs.md
