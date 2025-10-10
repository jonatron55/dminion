---
applyTo: "**/*.{ts,svelte,scss}"
---

TypeScript, Svelte, and SCSS coding guidelines
==============================================

This project uses TypeScript for frontend code and Svelte for UI components. It uses the Tauri framework to interface
with the Rust backend and `pnpm` for package management (do **not** attempt to use `npm` or `yarn`).

General rules
-------------

- Use `<script lang="ts">` in Svelte components to enable TypeScript and `<style lang="scss">` for SCSS styles. Do not
  use plain JavaScript or CSS.
- Use `$:` for reactive statements and declarations in Svelte.
- Do not add `I` prefixes to interface names. Use a descriptive name instead.
- Avoid explicit or implicit `any` types. Prefer `unknown` if the type is not known.
- Keep styles within Svelte components focused on layout and positioning. Use classes and variables from the
  [Styling system](#styling-system) defined below to define appearance.
- Avoid inline styles. Define styles in the component's `<style>` block instead.
- Do **not** attempt to use external CSS frameworks (e.g., Bootstrap, Tailwind).
- Do **not** use explicit color values or hard-coded sizes in styles. Themes and sizing can change, so always use
  variables or classes from the styling system.

Styling system
--------------

A comprehensive styling system is defined in `src/styles/`. You do not need to examine the entire system, but you should
be familiar with the variables and classes discussed below.

The styling system can be configured for light and dark modes, and supports different color schemes. You should use
semantic tokens whenever possible, as these will adapt to the current theme and color scheme.

In general, this system relies on element selectors first, then classes. Controls such as buttons and inputs do not have
special components, but are styled using element selectors and classes.

### Classes ###

| Class name          | Applies to                                                | Description                                                                                   |
| ------------------- | --------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `panel`             | Any block element                                         | A high-level UI panel with padding, a border, and optional caption bar.                       |
| `card`              | Any block element                                         | A card-like container with padding and a border.                                              |
| `caption`           | The first block element contained in a `panel` or `card`  | A caption bar for panels or sections.                                                         |
| `content`           | The second block element contained in a `panel` or `card` | A content area with padding used within a `panel` or `card`.                                  |
| `dim`               | Any text element                                          | Dimmed or less prominent text.                                                                |
| `badge`             | Inline elements                                           | A badge or tag-like element that's visually distinct from surrounding text.                   |
| `secondary`         | Panels, cards, control elements                           | A secondary or less prominent control style.                                                  |
| `ok`                | Panels, cards, control elements                           | A control indicating a positive or successful action.                                         |
| `caution`           | Panels, cards, control elements                           | A control indicating a cautionary action or warning.                                          |
| `danger`            | Panels, cards, control elements                           | A control indicating a deleterious action or error.                                           |
| `toolbar`           | Most control elements                                     | Styles a control as a toolbar button. May be used for controls in a `caption`.                |
| `toggle-switch`     | `<input type="checkbox">` elements                        | Styles a checkbox as a toggle switch.                                                         |
| `buttons-container` | Block element containing dialog buttons                   | Used to correctly order Yes/No/Cancel (or similar) buttons according to platform conventions. |

### Sizing and spacing ###

This project uses monospaced fonts throughout and rem-based sizing. Font sizes are not typically changed, and when they
are it is only by an integral amount (e.g., `2rem`, `3rem`).

For most margins, paddings, and gaps, use the `--horizontal-gap` and `--vertical-gap` variables. If larger gaps are
needed, use either multiples of these variables (e.g., `2 * var(--horizontal-gap)`) or rem values in increments of
`0.5rem`.

### Color variables ###

Do not attempt to read the entire color palette. It is generated from a palette generator and is quite large. Rely on
these guidelines for selecting colors.

Semantic colors are defined for common UI elements, these should be used in most cases (unless a class can be used
instead):

- `--primary-foreground`, `--intense-foreground`, `--dim-foreground`
- `--content-background`, `--card-background`, `--page-background`
- `--selection-background`, `--dim-selection-background`
- `--chrome-foreground`, `--chrome-high`, `--chrome`, `--chrome-low`, `--chrome-background`, `--chrome-shadow`,
  `--caption-foreground`
- `--link(-hover|-active|-disabled)?`
- `--label(-hover|-active|-disabled)?`
- `--control(-hover|-active|-disabled)?-(foreground|background)`
- `--secondary-control(-hover|-active|-disabled)?-(foreground|background)`
- `--ok(-hover|-active|-disabled)?-(foreground|background)`
- `--caution(-hover|-active|-disabled)?-(foreground|background)`
- `--danger(-hover|-active|-disabled)?-(foreground|background)`
- `--input(-hover|-active)?-(border|background)`

If no semantic color is appropriate, a color may be selected from the palette using the form
`--<intensity>-<hue>-<shade>` where:

- `intensity` is one of: `muted`, `base`, or `intense`.
- `hue` may be:
  - A color index: `color-[1-6]`. Where `1` is the typical foreground color, `6` the typical background color, and
    others are various accent colors.
  - A color temperature: `cold`, `cool`, `coolish`, `warmish`, `warm`, `hot`.
  - A primary color name: `red`, `green`, `blue`, `cyan`, `purple`, `yellow`. These are not exact matches to standard
    colors, but instead the closest match in the palette.
- `shade` may be:
  - A semantic shade: `low-background`, `medium-background`, `high-background`, `low-midground`, `medium-midground`,
    `high-midground`, `low-foreground`, `medium-foreground`, `high-foreground`.
  - A literal shade: `dim-low`, `dim-medium`, `dim-high`, `medium-low`, `medium`, `medium-high`, `bright-low`,
    `bright-medium`, `bright-high`. Use these carefully, as they do not change with light and dark modes.

Dependencies
------------

First, **do not add new dependencies without first asking for approval**. Explain why new dependencies are needed.

Follow these guidelines when adding or modifying dependencies:

- Use `pnpm` commands to manage packages. Avoid modifying `package.json` directly.
- Do not specify package versions unless necessary. Use the latest version.
- Review existing dependencies before adding new ones. Avoid duplicating functionality.
