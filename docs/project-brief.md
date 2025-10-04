Dungeon Minion project brief
===========================

Summary
-------

Dungeon Minion is a desktop assistant for dungeon masters running Dungeons & Dragons 5e. The app runs locally, targets
power users, and is strictly a DM tool—players do not interact with it directly. The DM manages the game without
requiring player accounts, character sheet access, or any shared system that might impose trust overhead. Documentation
and plans will evolve iteratively for collaborative development and future open sourcing.

Core objectives
---------------

- Deliver a private, interactive DM window with complete game details alongside a player-facing window that mirrors
  the DM view with only player-appropriate information.
- Keep the entire experience offline first with local storage only.
- Prioritize the encounter perspective before other views.
- Enable power-user workflows with minimal friction.

Perspectives
------------

- Map: Display configured maps to players, highlight areas, add markers, and pursue routefinding as a stretch goal.
- Encounter: Track initiative, conditions, health, and lair actions; support undo, redo, and state saves.
- Trade: Manage buy and sell flows with totals, discounts, and curated item lists.
- Library: Organize campaign data, including maps, monsters, encounters, items, and player information.

Sidebar utilities
-----------------

- Dice calculator for complex rolls.
- Name generator for NPCs, monsters, and locations.
- Settings for global themes, displays, and data management.

Architecture decisions
----------------------

- Use Tauri with SvelteKit for cross-platform delivery.
- Maintain multiple windows in one process, do not overcomplicate window management with multiple apps.
- Implement the shared data model in Rust with `sqlx` for SQLite access.
- Favor forms-on-tables UI patterns for data entry.

Data management
---------------

- Store campaign data in SQLite with one database file per campaign.
- Persist encounters, monsters, items, players, and related assets inside the campaign database.
- Support data entry primarily through in-app forms over the data model.
- Bulk operations are a non-goal; users can leverage external SQLite tools when needed.

Encounter workflow focus
------------------------

- Build the monster database backend to support end-to-end encounter management.
- Support DM overrides for any creature or condition attribute, including hit points, initiative, timers, and stats.
- Limit undo and redo to the encounter perspective and keep the history in-memory only.
- Provide encounter savepoints stored separately from the main database.

Testing approach
----------------

- Rely on manual testing during early development.
- Add Rust unit tests where practical; automation and CI are out of scope for now.

Key decisions
-------------

- **SQLite for storage**: Portable, zero-config, and suitable for local-first desktop apps.
- **One database per campaign**: Natural isolation boundary that maps to DM workflow.
- **Forms-first data entry**: Matches power-user expectations; bulk operations handled externally when needed.

Stretch goals backlog
---------------------

- Import SRD content when creating a database once sourcing is solved.
- Add map routefinding assistance.
- Integrate AI image generation for NPC and monster art.
