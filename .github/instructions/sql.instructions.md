---
applyTo: "**/*.sql"
---

SQL coding guidelines
=====================

SQL files in this project target **SQLite** and must follow SQLite’s dialect and constraints. Queries and schemas map
directly to Rust types through `sqlx`, so consistency and predictability are essential.

Style and formatting
--------------------

- Use **UPPERCASE** for SQL keywords (`SELECT`, `FROM`, `WHERE`, `JOIN`, `INSERT`, `UPDATE`, `DELETE`).
- Use **lowercase** for SQL functions (`count()`, `max()`, `lower()`).
- Indent query clauses on separate lines for readability:

  ```sql
  SELECT id, first_name
      FROM User
      WHERE created_at > ?
      ORDER BY created_at DESC;
  ```

- End statements with a semicolon.
- Avoid trailing commas in column lists.

Naming conventions
------------------

Tables map to Rust structs via `sqlx`. Use names that align with Rust conventions:

- Use **PascalCase** for table names (`UserAccount`, `GameSession`).
- Use **singular** table names (`User`, `Order`).
- Use **snake_case** for column names (`first_name`, `created_at`).
- Use `<entity>_id` for foreign keys (`user_id`, `order_id`).
- Avoid Rust reserved words in table or column names (`type`, `match`, `self`, etc.).

Schema design
-------------

- Define primary keys as:

  ```sql
  id INTEGER PRIMARY KEY
  ```

  SQLite automatically treats this as an alias for `ROWID`; no `AUTO_INCREMENT` needed.

- Use `NOT NULL` unless nullability is explicitly required.
- Avoid `DEFAULT` values unless there is a strong, documented reason. Prefer handling defaults in application code.
- Use appropriate SQLite types:
  - `TEXT` for strings
  - `INTEGER` for whole numbers and booleans
  - `REAL` for floating‑point values
  - `BLOB` for binary data

Foreign keys and relationships
------------------------------

- Use `<entity>_id` for foreign key columns.
- Explicitly declare foreign keys:

  ```sql
  FOREIGN KEY (user_id) REFERENCES User(id)
  ```

- Consider cascading behavior carefully. Use `ON DELETE CASCADE` or `ON UPDATE CASCADE` only when the relationship truly
  requires it.
- Avoid circular foreign key dependencies.

Query guidelines
----------------

- Prefer explicit `JOIN` syntax over implicit joins.
- Always specify the columns you need; avoid `SELECT *` in production queries.
- Use parameter placeholders (`?`) for values bound by `sqlx`.
- Use `LIMIT` and `ORDER BY` when returning multiple rows to ensure deterministic results.
- When filtering by text, use `LIKE` with care — SQLite’s default collation is case‑sensitive unless configured otherwise.

Migrations
----------

- Write migrations as **idempotent** as possible. Use `IF NOT EXISTS` when creating tables or indexes:

  ```sql
  CREATE TABLE IF NOT EXISTS User (
      id INTEGER PRIMARY KEY,
      ...
  );
  ```

- When altering schema, prefer additive changes (adding columns) over destructive ones.
- Document breaking schema changes in the migration file header.

Indexing
--------

- Create indexes only when needed for performance.
- Name indexes using the pattern:

  ```
  idx_<table>_<column>
  ```

  Example: `idx_User_created_at`

- Avoid indexing boolean or low‑cardinality columns.

Data integrity
--------------

- Use `CHECK` constraints when they improve correctness:

  ```sql
  CHECK (status IN ('pending', 'active', 'disabled'))
  ```

- Avoid storing derived or redundant data unless necessary for performance.

Comments and documentation
--------------------------

- Use `--` for inline comments.
- Add a short comment block at the top of each migration describing:
  - Purpose of the change
  - Impact on existing data
  - Any required application‑level changes
