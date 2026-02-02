As an engineering team, we have a SQLite data model supporting campaigns
========================================================================

**Date completed:** 2026-02-02

Tasks completed
---------------

- [x] Implement campaign folder and settings management
  - [x] Create `config/paths.rs` for `ProjectDirs` integration and campaign folder resolution
  - [x] Create `config/campaign_settings.rs` for per-campaign `settings.toml` (name, schema version, created date, theme, rules version)
  - [x] Create `config/app_settings.rs` for app-wide `settings.toml` in `preference_dir`
  - [x] Implement campaign folder creation with subfolders (`saves/`, `portraits/`, `maps/`)
  - [x] Implement folder naming (kebab-case, 16-char limit including suffix, numeric disambiguation)
- [x] Define campaign database schema
  - [x] Create initial migration with core tables in `migrations/` folder
  - [x] Add `Maps` table for map metadata (image path references external file)
  - [x] Add `Items` table for item library entries
  - [x] Update `portrait` field to store base name only (resolution handled by service)
  - [x] Document table relationships and constraints with `ON DELETE` behavior
- [x] Integrate SQLite with Tauri app
  - [x] Add `sqlx` crate with SQLite feature to `Cargo.toml`
  - [x] Create `db/connection.rs` for `SqlitePool` management per campaign
  - [x] Create `db/error.rs` for `DbError` enum with `thiserror`
  - [x] Implement campaign open/close lifecycle (load settings, open DB, release on close)
  - [x] Add campaign commands (`create_campaign`, `open_campaign`, `close_campaign`, `list_campaigns`)
- [x] Implement portrait resolution service
  - [x] Create `services/portrait.rs` with prioritized search (campaign → app → placeholder)
  - [x] Support `.small` and `.full` size variants
  - [x] Handle missing files and broken symlinks gracefully
- [x] Implement encounter savepoint service
  - [x] Create `services/savepoint.rs` for TOML serialization of complete encounter state
  - [x] Implement periodic savepoint writes to `saves/turn-NNN.toml`
  - [x] Implement savepoint restoration on app startup if encounter was in progress
- [x] Implement core CRUD operations for monsters
  - [x] Create `db/monsters.rs` module with query functions
  - [x] Implement `insert_monster`, `get_monster`, `list_monsters`, `update_monster`, `delete_monster`
  - [x] Add proper error handling and return types
- [x] Implement core CRUD operations for players
  - [x] Create `db/players.rs` module with query functions
  - [x] Implement `insert_player`, `get_player`, `list_players`, `update_player`, `delete_player`
  - [x] Add proper error handling and return types
  - [x] Implement player class operations (`add_player_class`, `list_player_classes`, `update_player_class`, `delete_player_class`)
  - [x] Add `list_players_in_party` query function
- [x] Implement core CRUD operations for parties
  - [x] Create `db/parties.rs` module with query functions
  - [x] Implement `insert_party`, `get_party`, `list_parties`, `update_party`, `delete_party`
  - [x] Add proper error handling and return types
- [x] Add database migration support
  - [x] Create `migrations/` folder structure with initial schema
  - [x] Use sqlx built-in migration system (runs automatically on campaign open)
  - [x] Schema version in `settings.toml` for documentation; sqlx tracks versions internally

Learnings
---------

### Campaign-first architecture ###

The campaign-as-folder design creates a clean separation of concerns. Each campaign is a self-contained unit with:
- `settings.toml` for metadata and preferences
- `db.sqlite3` for structured data
- `saves/`, `portraits/`, `maps/` subfolders for assets

This makes campaigns portable (copy folder = copy campaign) and simplifies backup/restore.

### One schema version for everything ###

Using a single semver version in `settings.toml` to cover both folder structure and database schema simplifies versioning. On startup: read version → run migrations → update version. This avoids the complexity of tracking multiple version numbers.

### Base name portrait strategy ###

Storing only the portrait base name (e.g., `goblin`) in the database rather than full paths provides flexibility:
1. Portrait resolution happens at runtime via service
2. Supports fallback search order (campaign → app → placeholder)
3. Supports size variants (`.small`, `.full`) with automatic alternate size fallback
4. Simplifies database schema and queries

### Complete state savepoints ###

Storing complete encounter state (not deltas) in TOML files makes savepoints simple and reliable:
- Easy to serialize/deserialize with serde
- Human-readable for debugging
- Recovery from crashes is straightforward
- Prune old savepoints when count exceeds limit
- No complex delta reconstruction logic needed

### i64 for database, i32 for domain ###

The convention of using i64 for database IDs (SQLite's native INTEGER type) and i32 for gameplay values (stats, AC, initiative) provides a good balance:
- i64 primary/foreign keys accommodate large databases and prevent overflow
- i32 gameplay values save memory and match expected ranges
- sqlx handles the conversion transparently
- Clear semantic distinction between identity and quantity

### Multiclass support from the start ###

The `PlayerClass` junction table enables multiclass characters without schema changes later. Storing level per class rather than trying to encode multiclass in a single field makes queries simple and supports unlimited combinations.

### Folder name normalization ###

Kebab-case normalization with 16-character limit and numeric disambiguation creates predictable, filesystem-safe folder names:
- "Dragon Heist" → `dragon-heist`
- "My Super Long Campaign Name" → `my-super-long-c`
- Collision: `dragon-heist` → `dragon-heist-2`

This avoids issues with spaces, special characters, and long paths on Windows.

### Type consolidation patterns ###

Having separate types for database records vs. insert/update data is useful:
- `MonsterRecord` includes `id: i64` from database
- `MonsterData` omits id, used for both insert and update
- Reduces duplication while maintaining type safety
- Clear at call site whether you're working with persisted vs. transient data

### Integer type alignment ###

Aligning all game domain integers to i32 eliminates casting noise throughout the codebase. The brief refactor to change `u32 → i32` for stats, initiative, AC, etc. made the code cleaner and more consistent with database types.

### Field naming conventions ###

Using `str` instead of `str_` works fine in Rust when used as a field name (the language distinguishes field context from type context). This keeps database field names clean and matches SQL column names exactly.

Reflections
-----------

### Well-scoped objective ###

Objective 5 was well-defined with clear deliverables. The architecture decisions documented in working notes provided a good foundation for implementation. Breaking it into config → database → services → CRUD operations created a logical sequence.

### Test-driven approach ###

Writing unit tests for config, portrait service, and savepoint service before integrating them into commands caught issues early and provided confidence in the implementation. All 11 tests passed on first run after integration.

### Pattern consistency ###

Following the same pattern for monsters, players, and parties (separate Record/Data types, similar method names, consistent error handling) made the CRUD implementations straightforward. The third module (parties) took less than 5 minutes to write because the pattern was established.

### Incremental compilation ###

Running `cargo check` after each major addition (config module, db module, services module) caught issues incrementally rather than facing a pile of errors at the end. The warnings about unused code were expected and acceptable during implementation.

### Working notes as implementation guide ###

The detailed architecture decisions in working notes (portrait resolution strategy, savepoint format, error handling approach) served as an effective implementation guide. Having these decisions documented before writing code reduced the number of mid-implementation pivots.

### Database-first schema design ###

Creating the SQL migration file first, then implementing the CRUD operations to match the schema, felt more natural than the reverse. The database schema acted as the contract that Rust types implemented.

New work identified
-------------------

None. The objective is complete and provides a solid foundation for campaign data management. Next work will involve:
- Connecting these CRUD operations to Tauri commands for frontend use (Objective 6 scope)
- Building UI for campaign, monster, and player management
- Implementing encounter persistence/restore using savepoints
