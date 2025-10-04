Getting started
===============

1. Read the [project brief] to understand what we're building and why.
2. Review the [process document] to learn how work flows through the system.
3. Check [active tasks] to see what's currently in progress.
4. Skim the [backlog] to understand priorities and upcoming work.

**Ensure you have read and understood the above documents before proceeding.**

Working on tasks
================

1. Pick an unchecked task from [active tasks]. If your instructions seem to refer to something not in the document, ask
   for clarification.
2. Complete the work according to [coding standards](#coding-standards).
3. Check off the task.
4. Add discoveries or context to "Working notes" in [active tasks].

Coding standards
----------------

You are surgically precise and pragmatic. You write clean, idiomatic code that focuses on the task at hand with the
simplest and smallest changes possible. Your goal is a minimal diff that does not alter unrelated code or add
unnecessary complexity.

### DO ###

- Keep code units cohesive and minimally coupled.
- Introduce new items at the narrowest scope possible.
- Search for existing functionality before adding new code.
- Write clear, self-explanatory code with meaningful names.
- Document public types and members with complete doc comments following [style guidelines].
- Update comments and documentation when the code they reference changes.
- Follow existing style and conventions in the codebase.
- Fix compiler warnings and linter issues in the areas you touch.

### DO NOT ###

- Refactor existing code unless specifically instructed.
- Reorganize existing declarations or rename them unnecessarily.
- Add new dependencies or libraries without first seeking approval.
- Add abstractions "just in case" they might be useful later.
- Attempt to fix compiler warnings or linter issues with code you did not modify.
- Fix warnings by applying a global suppression.
- Attempt to change file encodings, line endings, or formatting applied by your editor.

Maintaining documentation
=========================

**Critical**: Keep documentation current as you work. You should imagine that you might at any point need to hand off
work to someone new who has no context nor explanation apart from the documents mentioned here.

- Update [active tasks] immediately when tasks are completed or new details emerge.
- Follow the same standards you see in existing documents (formatting, level of detail, tone).
- Ask questions if anything is unclear; better to clarify than guess.

What's expected
===============

- **Surgically precise changes**: Each change should be as minimal as possible to achieve the goal.
- **High cohesion, low coupling**: New code should be focused and encapsulated.
- **Documentation is not optional**: It's how the team stays synchronized and how you hold yourself accountable.
- **Be consistent**: Match the conventions you see in existing code. Match the brevity you see in existing documents.
- **Reference, don't repeat**: Reuse existing functionality and link to other documents rather than duplicating
  material.
- **Keep it tactical**: Focus on what to do, not extensive explanation.

[active tasks]: /docs/process/active-tasks.md
[backlog]: /docs/process/backlog.md
[process document]: /docs/process.md
[project brief]: /docs/project-brief.md
[style guidelines]: /.github/instructions/md.instructions.md
