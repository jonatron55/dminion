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
| As a DM, I can manage a monster library with custom entries. | Create, edit, and organize monsters with full stat blocks and abilities using in-app forms. | now | high |
| As a DM, I can run encounters with initiative, health, and condition tracking. | Track turn order, HP changes, status effects, and lair actions with full DM override capability. | now | high |
| As a DM, I can undo and redo actions during encounters. | Maintain in-memory history for encounter changes with clear undo/redo controls. | now | high |
| As a DM, I can save and restore encounter states. | Create savepoints for encounters separately from the main campaign database. | now | high |
| As an engineering team, we have a SQLite data model supporting campaigns. | One database per campaign with tables for monsters, encounters, items, and players. | now | high |
| As an engineering team, we have stable Rust-to-SvelteKit contracts. | Define and document data structures passed between backend and frontend. | now | high |
| As a DM, I can create and manage campaigns. | Initialize new campaign databases and switch between existing campaigns. | now | high |
| As a DM, I can display maps to players on a separate window. | Show configured maps with basic display controls in a player-facing window. | later | med |
| As a DM, I can highlight and mark areas on maps. | Add visual indicators and markers to maps visible in the player window. | later | med |
| As a DM, I can manage a trade interface with pricing. | Track buy/sell transactions with item lists, quantities, and price adjustments. | later | med |
| As a DM, I can organize items in a library. | Create and categorize items with properties, descriptions, and pricing. | later | med |
| As a DM, I can track player information in my campaign. | Store player characters with basic details for reference during play. | later | med |
| As a DM, I can use a dice calculator for complex rolls. | Execute multi-part dice expressions with modifiers in a sidebar utility. | later | low |
| As a DM, I can generate random names. | Quick-generate names for NPCs, monsters, and locations. | later | low |
| As a DM, I can customize themes and display settings. | Configure appearance and multi-monitor setup preferences. | later | low |
| As an engineering team, we can handle database schema evolution. | Implement migration strategy for safe schema changes across versions. | later | med |

Doing
-----

See [active tasks] for a detailed breakdown of current work in progress. Current high-level
objectives are:

- As an engineering team, we have a clearly documented understanding of the project's goals and high-level design
  decisions so that we can break down work into key deliverables.
- As an engineering team, we have an adaptable and agile process for managing work so that we can execute efficiently.

Done
----

| Objective | Link to completion log |
| --------- | ---------------------- |
|           |                        |
|           |                        |
|           |                        |

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
