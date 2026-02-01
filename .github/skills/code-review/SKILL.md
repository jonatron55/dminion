---
name: code-review
description: Instructions for reviewing changes and ensuring quality before completion. Use when asking for a review or before committing changes.
---

Code review instructions
========================

The code you are about to review has been written by a colleague and is ready for your feedback. You should be critical
and thorough in your review to ensure the highest quality of work. Follow these steps to conduct an effective code
review:

1. **Use git commands to examine the changes.**
   - The changes are not yet committed, so use `git diff`, `git status`, and related commands to see what has been
     modified.
   - Focus on the specific files and lines that have been changed. Don’t get distracted by unrelated pre-existing code.

2. **Check for correctness and functionality.**
   - The `active-tasks.md` **must** be included in the changes. Completed items should be marked as done and working
     notes should be updated.
   - Ensure that the changes align with the objectives and tasks outlined in `active-tasks.md`.

3. **Review code quality and style.**
   - Code should follow specific guidelines for certain languages:
     - [Markdown and documentation guidelines]
     - [Rust coding guidelines]
     - [SQL coding guidelines]
     - [TypeScript, Svelte, and SCSS coding guidelines]
   - Changes **should**:
     - Keep code units cohesive and minimally coupled.
     - Introduce new items at the narrowest scope possible.
     - Reuse existing functionality before adding new code.
     - Have clear, self-explanatory code with meaningful names.
     - Document public types and members with complete doc comments following [Markdown and documentation guidelines].
     - Update comments when the code they reference changes.
     - Follow existing style and conventions in the codebase.
   - Changes **shouldn’t**:
     - Refactor existing code unless specifically part of the task.
     - Reorganize existing declarations or rename them unnecessarily.
     - Add new dependencies or libraries without first seeking approval.
     - Add abstractions “just in case” they might be useful later.
     - Introduce new compiler warnings and linter issues.
     - Attempt to fix compiler warnings or linter issues in unmodified code.
     - Fix warnings by applying a global suppression.

4. **Review architecture and design.**
   - Look to the [Project brief] for high-level architectural guidelines.
   - Ensure that the changes fit well within the existing architecture and follow best practices.
   - Consider performance, scalability, and maintainability of the changes.
   - If the [Project brief] is deficient, consider improvements.

[Markdown and documentation guidelines]: /.github/instructions/markdown.instructions.md
[Project brief]: /docs/project-brief.md
[Rust coding guidelines]: /.github/instructions/rust.instructions.md
[SQL coding guidelines]: /.github/instructions/sql.instructions.md
[TypeScript, Svelte, and SCSS coding guidelines]: /.github/instructions/typescript.instructions.md
