Objective completion instructions
==================================

Follow these steps when all tasks for an objective are complete:

1. **Create a completion log file for each completed objective.**
   - Navigate to `docs/process/completion-logs/`.
   - Create a new file named `YYYY-MM-DD-objective-name.md` (e.g., `2025-10-04-project-documentation.md`).
   - Use lowercase with hyphens for the objective name portion.
   - If multiple objectives completed on the same day, append `-01`, `-02`, etc. to distinguish them.

2. **Document what was completed in the log.**
   - Start with a heading using the full objective statement.
   - Include a "Tasks completed" section with the checkbox list from `active-tasks.md`.
   - Mark all checkboxes as complete to show the final state.
   - Add a "Date completed" field at the top for quick reference.

3. **Capture learnings and reflections.**
   - Add a "Learnings" section documenting key discoveries or insights.
   - Include technical decisions made, approaches that worked well, or challenges encountered.
   - Copy relevant content from the "Working notes" section of `active-tasks.md`.
   - Add a "Reflections" section for process observations, what to do differently, or improvements.
   - Keep both sections concise but substantive; future you will thank you.

4. **Handle new work identified during completion.**
   - Review working notes and reflections for newly identified work.
   - For well-defined new objectives:
     - Add them to the "To do" table in `backlog.md`.
     - Include objective statement, description, readiness, and priority.
   - For ideas needing more thought:
     - Add them to the "Parking lot" section in `backlog.md`.
     - Include open questions or concerns that need resolution.

5. **Update the "Done" section in `backlog.md`.**
   - Add a new row to the "Done" table for each completed objective.
   - Copy the objective statement to the first column.
   - Link to the completion log file in the second column using relative path.
   - Example: `[2025-10-04](./completion-logs/2025-10-04-project-documentation.md)`

6. **Reset the "Doing" section in `backlog.md`.**
   - Remove completed objective(s) from the "Doing" bullet list.
   - If no objectives remain active, leave the section with just the reference link.
   - Update the introductory sentence if needed to reflect current state.

7. **Reset `active-tasks.md` for new work.**
   - Remove all completed objective sections and their tasks.
   - Clear the "Working notes" section (content should now be in completion logs).
   - Keep the document structure and instructions at the top.
   - Leave the file ready for the next objective breakdown.

8. **Review and reflect on the backlog.**
   - After completion, take a moment to review the "To do" section.
   - Consider if priorities or readiness have changed based on what was learned.
   - Update the backlog table if needed to reflect new understanding.
   - This is a natural checkpoint to reassess direction.
