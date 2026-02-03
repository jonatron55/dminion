***As an engineering team, we have stable Rust-to-SvelteKit contracts.***

**Date completed:** February 2, 2026

Tasks completed
---------------

- [x] Audit and document existing data contracts
  - [x] Review all Tauri commands in `game_commands.rs` and `dice_commands.rs`
  - [x] Review all TypeScript types in `src/lib/model/`
  - [x] Identify mismatches between Rust structs and TypeScript interfaces
  - [x] Document current command signatures and return types
- [x] Establish contract documentation standards
  - [x] Create `docs/reference/contracts.md` documenting all commands
  - [x] Define naming conventions (snake_case Rust → camelCase TS)
  - [x] Document serialization rules (serde rename attributes)
- [x] Align Rust and TypeScript types
  - [x] Fix `Player.classes` field name mismatch (`class` → `name`)
  - [x] Add `isHostile` field to TypeScript `Monster` interface
  - [x] Verify all other types match exactly
- [x] Add type generation or validation
  - [x] Added `ts-rs` crate with `serde-compat` and `chrono-impl` features
  - [x] Added `#[derive(TS)]` to all 16 serialized types
  - [x] Types generated to `src/lib/model/gen/` directory
  - [x] Manual type files replaced with re-exports from generated types
  - [x] Helper functions preserved in re-export files
- [x] Refactor command layer for consistency
  - [x] Standardize error handling across all commands (already done via tryInvoke)
  - [x] Review argument naming consistency across commands
  - [x] Document expected error responses in TypeScript

Learnings
---------

### TypeScript Type Generation with ts-rs

- **Setup**: Add `ts-rs = { version = "10", features = ["serde-compat", "chrono-impl"] }` to Cargo.toml
- **Derive macros**: All serialized types need `#[derive(Serialize, Deserialize, TS)]`
- **Export attributes**: Use `#[ts(export, export_to = "../src/lib/model/gen/")]` on each type
- **Generation command**: `cd src-tauri && cargo test export_bindings` generates TypeScript files
- **Non-serializable fields**: Use `#[ts(skip)]` for fields that don't serialize (e.g., `DiceExpr`)

### Type Contract Alignment

Discovered two mismatches during audit:

1. `Player.classes` used `class` field name instead of `name` - fixed in TypeScript
2. `Monster.isHostile` was missing from TypeScript interface - added

### Serialization Patterns

- **Discriminated unions**: Use `#[serde(tag = "type")]` on enums to create TypeScript union types
- **camelCase conversion**: Use `#[serde(rename_all = "camelCase")]` on structs for field names
- **Tuple variant flattening**: Serde flattens `Duration(Duration)` to same level as tagged enum
- **Test verification**: Created `test_expiry_serialization` to verify Rust→JSON→TypeScript alignment

### Re-export Pattern

Preserved backwards compatibility while using generated types:

- Manual type files (`Participant.ts`, `Stats.ts`, etc.) re-export from `gen/`
- Helper functions (`crValue`, `conditionNames`, `modifer`) stay in manual files
- Components import from manual files, which pull from generated types underneath

### Architecture Benefits

- Single source of truth for types (Rust structs)
- Compile-time verification of type generation
- No manual synchronization needed between Rust and TypeScript
- Caught mismatches that were working at runtime but violated contracts

Reflections
-----------

### What Worked Well

**ts-rs integration**: After initial setup challenges, ts-rs provides exactly what we needed - automatic type generation that respects serde attributes and creates proper discriminated unions.

**Comprehensive audit**: Taking time to document all commands and types in contracts.md (even though later removed) surfaced the mismatches that needed fixing. The documentation exercise was valuable even if the artifact didn't survive.

**Testing serialization**: Writing explicit tests for serialization behavior (like `test_expiry_serialization`) gives confidence that the Rust→JSON→TypeScript pipeline works correctly.

### Process Observations

**Documentation evolution**: Started with comprehensive contracts.md, then realized it was redundant with generated types. Consolidated guidelines into project-brief.md instead. This shows healthy iteration on what documentation actually needs to be maintained manually.

**Incremental verification**: Building UI after each change caught issues immediately. The fix-test-fix cycle kept problems small and manageable.

### For Next Time

**Generate early**: For future objectives involving types, add ts-rs derives from the start rather than maintaining manual types first. The generation process is straightforward once the initial setup is done.

**Test generation in CI**: Consider adding `cargo test export_bindings` to CI to catch when types aren't regenerated after Rust changes. For now, the command is documented in project-brief.md as part of the workflow.
