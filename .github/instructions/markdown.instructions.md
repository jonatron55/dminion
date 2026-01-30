---
applyTo: "**/*.md"
---

Documentation style guide
=========================

Concise, effective documentation is essential for this project. Apply these guidelines to all markdown files, including
planning and process documents, design documents, and code comments.

Audience
--------

Write for **software developers** with technical background but **no assumed project context**.

Writing style
-------------

- **Be concise:** short sentences, minimal words.
- **Use plain language:** prefer simple terms (“use” over “utilize”).
- **Write in active voice:** “Run the script”, not “The script should be run”.
- **Use imperative mood:** give direct instructions.
- **Use Oxford commas** in lists.
- **Use sentence case** for all headings and list items.
- **Avoid filler phrases:** remove “It is important to note that,” “In order to,” etc.

Structure
---------

- Lead with the **most important action**.
- Use **headings, lists, and code blocks** for scannability.
- Provide a **minimal working example** before advanced options.
- Link to deeper detail instead of expanding inline.
- Keep sections short and focused.

Formatting
----------

- Use **sentence case** for all headings; capitalize only the first word and proper nouns.
- Use **setext headings** (`===`, `---`) for levels 1–2.
- Use **ATX closed headings** (`### Heading ###`) for level 3+.
- Add a blank line **before and after**:
  - Headings
  - Lists
  - Tables
  - Code blocks
- Use **reference-style links** instead of inline URLs.

  **Example:**

  ```markdown
  See the [installation guide].

  <!-- Document continues... -->

  [installation guide]: https://example.com/install
  ```

- Always specify a **language tag** for fenced code blocks.
- Keep code examples **copy‑pasteable**; avoid ellipses in runnable snippets.

Diagrams
--------

- Use diagrams to clarify complex concepts or workflows.
- Use **Mermaid** fenced blocks (` ```mermaid `).
- Don’t use color in Mermaid diagrams.
- Don’t use ASCII art or text‑based diagrams.
- Use external **SVG** files when Mermaid is insufficient.
- Provide short descriptive alt text for all images.

Consistency
-----------

- Use the same term for the same concept throughout.
- Keep formatting uniform across headings, lists, and code blocks.
- Follow the same structure for similar document types.

Accessibility and inclusivity
-----------------------------

- Use neutral, inclusive language.
- Avoid idioms, jokes, or culture‑specific references.
- Use descriptive link text (“View API guide,” not “click here”).
- Provide alt text for images.
- Don’t use inline CSS, HTML styling, or hex colors.

Troubleshooting sections
------------------------

- Anticipate common errors and provide **short, actionable fixes**.
- Keep explanations focused on what the reader must do.

Do’s
----

- Use short, descriptive headings.
- Provide minimal working examples.
- Explain “why” only when it supports a decision.

Don’ts
------

- Don’t write long introductions or summaries.
- Don’t repeat instructions in multiple ways.
- Don’t use HTML tags instead of Markdown.
- Don’t include filler or conversational language.

Example rewrite
---------------

Wordy (avoid):

> In order to successfully configure the environment, you must first ensure that all required dependencies have been
> installed.

Concise (preferred):

> Install dependencies before configuring the environment.
