---
applyTo: "**/*.{ts,svelte,scss,json}"
---

TypeScript, Svelte, and SCSS coding guidelines
==============================================

This project uses TypeScript for frontend logic, Svelte for UI components, and SCSS for styling. It interfaces with the
Rust backend through Tauri and uses `pnpm` for package management. **Do not** use `npm` or `yarn`.

General rules
-------------

- Use `<script lang="ts">` in Svelte components and `<style lang="scss">` for styles.
  Do not use plain JavaScript or plain CSS.
- Use Svelte’s `$:` syntax for reactive statements and declarations.
- Do not prefix interfaces with `I`. Use descriptive names.
- Avoid explicit or implicit `any`. Use `unknown` when the type is not known.
- Keep component styles focused on layout and positioning.
  Use classes and variables from the **styling system** for appearance.
- Avoid inline styles. Place all styling in the component’s `<style>` block.
- Do **not** use external CSS frameworks (Bootstrap, Tailwind, etc.).
- Do **not** hard‑code colors or sizes. Use variables or classes from the styling system.

Styling system
--------------

A comprehensive styling system lives in `src/styles/`. You do not need to read the entire system, but you must understand the variables and classes listed here.

The system supports light/dark modes and multiple color schemes. Use **semantic tokens** whenever possible so UI adapts automatically.

Element selectors come first, then classes. Buttons, inputs, and other controls are styled through selectors and classes rather than custom components.

### Classes ###

| Class name          | Applies to                                                | Description                                                                                   |
| ------------------- | --------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `panel`             | Any block element                                         | High‑level UI panel with padding, border, and optional caption bar.                           |
| `card`              | Any block element                                         | Card‑like container with padding and border.                                                  |
| `caption`           | First block element inside a `panel` or `card`            | Optional caption bar for panels or cards.                                                     |
| `content`           | Main block element inside a `panel` or `card`             | Content area with padding.                                                                    |
| `dim`               | Any text element                                          | Less prominent or subdued text.                                                               |
| `badge`             | Inline elements                                           | Badge or tag‑like element visually distinct from surrounding text.                            |
| `secondary`         | Panels, cards, controls                                   | Secondary or less prominent control style.                                                    |
| `ok`                | Panels, cards, controls                                   | Indicates a positive or successful action.                                                    |
| `caution`           | Panels, cards, controls                                   | Indicates a cautionary action or warning.                                                     |
| `danger`            | Panels, cards, controls                                   | Indicates a destructive action or error.                                                      |
| `toolbar`           | Most controls                                             | Styles a control as a toolbar button; often used in `caption` bars.                           |
| `toggle-switch`     | `<input type="checkbox">`                                 | Styles a checkbox as a toggle switch.                                                         |
| `buttons-container` | Block element containing dialog buttons                   | Ensures correct Yes/No/Cancel ordering per platform conventions.                              |

### Sizing and spacing ###

- The UI uses monospaced fonts and rem‑based sizing. When adjusting font size, use whole‑rem values (`2rem`, `3rem`).
- For margins, paddings, and gaps, use `--horizontal-gap` and `--vertical-gap`. For larger spacing, use multiples of
  these variables or rem increments of `0.5rem`.
- Use `--ui-hydrate-time` and `--ui-transition-time` for animation durations.

### Color variables ###

The palette is large and generated automatically. Use these guidelines instead of browsing the full palette.

Prefer **semantic colors**:

- **Text:** `--primary-foreground`, `--intense-foreground`, `--dim-foreground`
- **Backgrounds:** `--content-background`, `--card-background`, `--page-background`
- **Selection:** `--selection-background`, `--dim-selection-background`
- **Chrome (noninteractive borders and dividers):** `--chrome-foreground`, `--chrome-high`, `--chrome`, `--chrome-low`,
  `--chrome-background`, `--chrome-shadow`, `--caption-foreground`
- **Links:** `--link(-hover|-active|-disabled)?`
- **Labels:** `--label(-hover|-active|-disabled)?`
- **Controls:**
  - Primary: `--control(-hover|-active|-disabled)?-(foreground|background)`
  - Secondary: `--secondary-control(-hover|-active|-disabled)?-(foreground|background)`
- **Affirmative:** `--ok(-hover|-active|-disabled)?-(foreground|background)`
- **Cautionary/warnings:** `--caution(-hover|-active|-disabled)?-(foreground|background)`
- **Dangerous/errors:** `--danger(-hover|-active|-disabled)?-(foreground|background)`
- **Inputs:** `--input(-hover|-active)?-(border|background)`

If no semantic color fits, use palette tokens:

`--<intensity>-<hue>-<shade>`

- **Intensity:** `muted`, `base`, `intense`
- **Hue:**
  - Indexed: `color-[1-6]`
  - Temperature: `cold`, `cool`, `coolish`, `warmish`, `warm`, `hot`
  - Named: `red`, `green`, `blue`, `cyan`, `purple`, `yellow`
- **Shade:**
  - Semantic: `low-background`, `medium-background`, `high-background`,
    `low-midground`, `medium-midground`, `high-midground`,
    `low-foreground`, `medium-foreground`, `high-foreground`
  - Literal: `dim-low`, `dim-medium`, `dim-high`, `medium-low`, `medium`,
    `medium-high`, `bright-low`, `bright-medium`, `bright-high`

Use temperature and literal shades carefully—they do not adapt to theme changes.

Dependencies
------------

**Do not add new dependencies without approval.** Explain why new dependencies are needed.

When adding or modifying dependencies:

- Use `pnpm` to manage packages; avoid editing `package.json` directly.
- Do not pin versions unless required. Let `pnpm` resolve the latest compatible version.
- Review existing dependencies before adding new ones to avoid duplication.
