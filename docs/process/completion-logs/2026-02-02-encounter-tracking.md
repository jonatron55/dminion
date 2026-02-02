As a DM, I can run encounters with initiative, health, and condition tracking
===========================================================================

**Date completed:** 2026-02-02

Tasks completed
---------------

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
- [x] Add lair action support to UI
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

Learnings
---------

### Timer reactivity in Svelte ###

Simple function calls in Svelte templates don't create reactive dependencies. For the encounter timer to update when `game.round` changes, we needed to use a reactive statement:

```svelte
$: timeStr = (() => {
    const time = game.time();
    // ... formatting logic
})()
```

This creates a reactive dependency on `game.round` (via `game.time()`) so the timer updates automatically.

### Temporary HP damage ordering ###

D&D 5e rules specify that damage is applied to temporary HP first, then to regular HP. The initial implementation applied damage directly to HP. The fix:

1. Calculate total damage based on damage type
2. Apply to temp HP first if available
3. Apply remaining damage to regular HP
4. Handle `Kill` damage type specially to clear both

This makes the system follow RAW correctly.

### Inverted checkbox semantics ###

The action checkboxes were confusing because the backend stores `action: bool` as "action available" (true = can use, false = used), but users expect checkboxes to show "checked when used". The solution was to invert the binding:

```svelte
checked={!monster.action}
on:change={(e) => monster.action = !e.currentTarget.checked}
```

This maintains the backend semantics while providing intuitive UI behavior.

### Full action state management ###

Moving from simple booleans to a typed `Action` enum enabled proper handling of legendary actions:

```rust
pub enum Action {
    Standard,
    Bonus,
    Reaction,
    Legendary { index: usize },
}
```

Combined with `Vec<bool>` for tracking multiple legendary action slots, this provides complete action economy tracking for complex monsters.

### Dual rules support strategy ###

Supporting both SRD 5.1 (2014) and SRD 5.2 (2024) difficulty calculations required:

- Separate threshold data structures (5.1 has Deadly, 5.2 has High)
- Conditional encounter multiplier logic (5.1 uses it, 5.2 doesn't)
- Type guards in TypeScript to safely access version-specific fields
- Visual gauge that adapts to show correct threshold markers

This pattern will apply to other rules differences (e.g., monster stat blocks, condition mechanics).

### Difficulty gauge as feedback tool ###

The difficulty gauge provides immediate visual feedback for encounter balance during design. Showing both raw stats (XP, CR) and computed values (adjusted XP, difficulty rating) helps DMs understand why an encounter is rated a certain way.

### Component-driven encounter UI ###

Breaking the encounter page into focused components (ParticipantRow, DifficultyGauge, EncounterToolbar, condition dialogs) made the UI manageable. Each component handles one concern and communicates through ViewModel methods.

Reflections
-----------

### Objective scope creep ###

This objective grew from "basic encounter tracking" to include undo/redo, lair actions, and difficulty gauges. While all valuable features, this made the objective take longer than initially estimated. Better scoping or breaking into sub-objectives would help.

### Bug-fix driven development ###

Several tasks (temp HP, inverted checkboxes, timer reactivity) were discovered bugs that became formal tasks. This reactive approach worked well—we shipped working features first, then polished based on actual use.

### UI/UX iteration value ###

Testing the encounter UI with real scenarios revealed subtle issues (checkbox semantics, timer not updating) that weren't obvious from code inspection. Regular manual testing during implementation was valuable.

### Backend-first helps ###

Having the action state management in Rust before wiring up the UI made the TypeScript integration straightforward. The contract was clear, and type safety caught mismatches immediately.

### Lair actions as first-class ###

Treating lair actions as full `Participant` types (not a special case) simplified the initiative system and made the UI consistent. Lair participants show up in the order naturally and get their turn like any other participant.

### Keyboard shortcuts matter ###

Adding Ctrl+Z/Ctrl+Y for undo/redo significantly improved workflow during testing. DMs will use these constantly during encounters. Similar shortcuts for common actions (damage, heal, next turn) would be valuable.

New work identified
-------------------

### End-to-end encounter testing ###

Comprehensive workflow testing remains incomplete:

- Create encounter from scratch
- Add mix of monsters, players, lair
- Run through 5+ rounds with damage, healing, conditions
- Verify undo/redo through complex state changes
- Test edge cases (negative HP, killing blows, condition expiry)

This should be added to the backlog as a quality objective once campaign/monster management UI is ready.

### Lair action UI completion ###

While lair participants display correctly, there's no specific UI for managing lair abilities or triggering lair events. This is low priority since basic lair initiative tracking works.

### Encounter persistence ###

Encounters currently exist only in memory. Once campaign database is wired up, need to:

- Save encounter configurations to DB
- Load encounters from DB
- Auto-save encounter state via savepoints (backend exists, needs UI)

This is covered by other backlog items about campaign management and savepoint restoration.
