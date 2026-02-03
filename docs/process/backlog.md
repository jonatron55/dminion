Backlog
=======

This document is a running list of high-level goals. Items here should be clear enough that they can be broken down into
smaller tasks when the time comes; however, this breakdown should *not* occur here. Instead, keep this list focused and
tactical.

It is normal and expected for items to move in and out of this list as development progresses and new information comes
to light. This list is not a commitment of work to be done, but rather a prioritized set of possibilities. It should be
reviewed and updated regularly.

To do
-----

These are high-level goals from perspective of a stakeholder. They should be of the form:

> As a (stakeholder), I want (outcome) so that (reason).

The stakeholder may be the DM, a player, us as an engineering team, or other personas we identify. The outcome should be
something we can deliver with a clear definition of done. The reason should explain why this outcome is valuable.

| Objective | Description | Readiness | Priority |
| --------- | ----------- | --------- | -------- |
| **8.** As a user, my preferences are saved between sessions. | Persist UI and game settings (theme, font, rules version, monster HP mode) to disk and restore on app launch. Backend foundation exists in `preferences.rs`. | now | high |
| **9.** As a DM, I can manage a player and monster library with custom entries. | Create, edit, and organize players and monsters with full stat blocks and abilities using in-app forms. | now | high |
| **10.** As a DM, I can manage an encounter library with custom entries. | Create, edit, and organize encounter configurations using in-app forms. | now | high |
| **11.** As a DM, I can save and restore encounter states. | Create savepoints for encounters separately from the main campaign database. | now | high |
| **12.** As a DM, I can create and manage campaigns. | Initialize new campaign databases and switch between existing campaigns. | now | high |
| **13.** As a DM, I can test full encounter workflows end-to-end. | Comprehensive testing: create encounters, add participants, run multiple rounds with damage/healing/conditions, verify undo/redo through complex state changes. | now | med |
| **14.** As a DM, I can display maps to players on a separate window. | Show configured maps with basic display controls in a player-facing window. | later | med |
| **15.** As a DM, I can highlight and mark areas on maps. | Add visual indicators and markers to maps visible in the player window. | later | med |
| **16.** As a DM, I can manage a trade interface with pricing. | Track buy/sell transactions with item lists, quantities, and price adjustments. | later | med |
| **17.** As a DM, I can organize items in a library. | Create and categorize items with properties, descriptions, and pricing. | later | med |
| **18.** As a DM, I can track player information in my campaign. | Store player characters with basic details for reference during play. | later | med |
| **19.** As a DM, I can generate random names. | Quick-generate names for NPCs, monsters, and locations. | later | low |
| **20.** As a DM, I can customize themes and display settings. | Configure appearance and multi-monitor setup preferences. | later | low |
| **21.** As an engineering team, we can handle database schema evolution. | Implement migration strategy for safe schema changes across versions. | later | med |

### Bugs ###

- Title bar does not resize elegantly when window is smaller than the toolbar's minimum width. The toolbar needs to be
  more responsive and clip when overflowing instead of attempting to wrap. In any case, the window decorations should
  never clip and should always be fully visible.

Doing
-----

See [active tasks] for a detailed breakdown of current work in progress. Current high-level
objectives are:

- As a DM, I can undo and redo actions during encounters.
- As an engineering team, we have stable Rust-to-SvelteKit contracts.

Done
----

| Objective | Link to completion log |
| --------- | ---------------------- |
| As a DM, I can run encounters with initiative, health, and condition tracking. | [2026-02-02](./completion-logs/2026-02-02-encounter-tracking.md) |
| As an engineering team, Views route commands through ViewModels. | [2026-02-02](./completion-logs/2026-02-02-viewmodel-routing.md) |
| As an engineering team, we have a SQLite data model supporting campaigns. | [2026-02-02](./completion-logs/2026-02-02-sqlite-data-model.md) |
| As a DM, I can use a dice calculator for complex rolls. | [2026-01-23](./completion-logs/2026-01-23-dice-calculator.md) |
| As an engineering team, we have a clearly documented understanding of the project's goals and high-level design decisions so that we can break down work into key deliverables. | [2025-10-04](./completion-logs/2025-10-04-project-documentation.md) |
| As an engineering team, we have an adaptable and agile process for managing work so that we can execute efficiently. | [2025-10-04](./completion-logs/2025-10-04-development-process.md) |

Parking lot
-----------

*Use this section for ideas that are not yet fully formed, but that you want to capture for future consideration.*

- SRD import when creating a new database.
  - Where to obtain structured SRD data? Only certain versions of SRD are CC-BY.
- Map routefinding assistance.
  - How to structure map data for this? Maps are at the moment just images.
- AI image generation for NPC and monster art.
  - Need to validate feasibility and quality. Is this really a useful feature or is easier to just use these tools
    directly?

[active tasks]: /docs/process/active-tasks.md
