---
applyTo: "**/*.md"
---

Documentation style guide
=========================

Audience
--------

Your readers are **software developers** with technical background. However, don’t assume they have any context on this
specific project.

Writing style
-------------

- **Be concise:** short sentences, minimal words.
- **Use plain language:** prefer simple terms (“use” over “utilize”).
- **Write in active voice:** “Run the script” not “The script should be run.”
- **Use an imperative mood:** give direct instructions.
- **Use Oxford commas** in lists.
- **Use sentence case** for headings and lists.

Structure
---------

- Put the **most important action first**.
- Use **headings, lists, and code blocks** for scannability.
- Provide a **minimal working example** before advanced options.
- Link out to details instead of overloading the main document.

Formatting
----------

- Use **sentence case** in all headings. Capitalize only the first word and proper nouns.
- Use **setext style** (`===` or `---`) for level 1 and 2 headings.
- Use **ATX closed style** (`### Heading ###`) for levels 3 and deeper.
- Ensure there are blank lines *before* and *after*:
  - Headings
  - Tables
  - Lists
  - Code blocks
- Use **reference-style** links instead of inline URLs wherever possible.

  **Example:**

  ```markdown
  See the [installation guide].

  ...

  [installation guide]: https://example.com/install
  ```

Diagrams
--------

- Use diagrams when you need to clarify complex concepts or workflows.
- Use Mermaid syntax for diagrams in a code block with `mermaid` specified.
- Don’t apply color to Mermaid diagrams. It will not adapt to different themes.
- Don’t use ASCII art or other text-based diagrams.
- Use external SVG images when Mermaid is insufficient.

Consistency
-----------

- Use the same term for the same concept throughout.
- Keep formatting uniform (headings, code snippets, lists).

Accessibility and inclusivity
-----------------------------

- Use neutral, inclusive language.
- Avoid idioms, jokes, or culture-specific references.
- Write descriptive link text (*“View API guide”* not *“click here”*).
- Provide alternate text for images.
- Don’t use hexadecimal colors or CSS styling in Markdown documents. Use only plain text and ordinary Markdown
  formatting.

Troubleshooting
---------------

- Anticipate common errors and show quick fixes.
- Keep error explanations short and actionable.

Do’s
----

- Use short, descriptive headings.
- Provide copy-pasteable code snippets.
- Explain why only when it helps decision-making.

Don’ts
------

- Don’t write long introductions or summaries.
- Don’t repeat the same instruction in multiple ways.
- Don’t use filler phrases (“It is important to note that”).
- Use HTML tags instead of Markdown syntax.

Example rewrite
---------------

Wordy (avoid):
> In order to successfully configure the environment, you must first ensure that all required dependencies have been
> installed.

Concise (preferred):
> Install dependencies before configuring the environment.
