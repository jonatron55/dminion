As a DM, I can use a dice calculator for complex rolls
======================================================

**Date completed:** 2026-01-23

Tasks completed
---------------

- [x] Design dice calculator UI component
  - [x] Create sidebar panel or dialog for dice calculator
  - [x] Add input field for dice expressions
  - [x] Design result display area
  - [x] Add roll history display
- [x] Integrate dice parser backend with UI
  - [x] Wire up input field to `roll` command
  - [x] Handle and display roll results
  - [x] Show breakdown of roll components (individual dice, modifiers)
  - [x] Display parse errors clearly when expression is invalid
- [x] Add dice calculator to sidebar or toolbar
  - [x] Add button/icon to open dice calculator
  - [x] Implement show/hide toggle
  - [x] Position calculator appropriately in layout
- [x] Add common dice presets
  - [x] Quick buttons for d20, d12, d10, d8, d6, d4
  - [x] Advantage/disadvantage buttons for d20 rolls
  - [x] Modifier input for quick adjustments
- [x] Test dice calculator with complex expressions
  - [x] Test multi-dice rolls (3d6+2d8+5)
  - [x] Test advantage/disadvantage syntax
  - [x] Verify all operators work correctly

Learnings
---------

### Calculator-style keypad layout ###

The grid-based keypad layout works well for dice calculators. Using CSS grid with `span` for larger buttons (d20, +,
enter) creates a familiar calculator feel while accommodating RPG-specific buttons like `adv`, `dis`, `kh`, `kl`.

### Token-aware backspace ###

Standard character-by-character backspace is frustrating for dice expressions. Implementing token-aware backspace that
removes entire tokens (`d20`, `adv`, `d100`) in one press provides a much better user experience.

### Expression re-roll pattern ###

Allowing empty submit to re-roll the last expression is a useful pattern. Users often want to repeat the same roll
(e.g., multiple attack rolls with the same modifier).

### Roll breakdown display ###

Showing individual dice results with keep/drop indicators helps users verify rolls and understand advantage/disadvantage
mechanics. The visual distinction between kept and dropped dice (strike-through for dropped) is immediately clear.

Reflections
-----------

The dice calculator came together quickly because the backend dice parser was already well-implemented. Having a
complete parsing and evaluation system in Rust meant the UI work was purely about presentation and interaction design.

The sidebar panel approach works well for tools that need to be accessible during encounters without blocking the main
view. This pattern could be extended to other quick-reference tools like condition lookups or rule references.

The keypad took some iteration to get the button layout right, but the result feels natural for tabletop use. The large
d20 button spanning multiple rows emphasizes its importance in D&D gameplay.
