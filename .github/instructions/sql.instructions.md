---
applyTo: "**/*.sql"
---

SQL coding guidelines
=====================

- SQL files in this project are used with SQLite databases and follow that dialect.
- Use **UPPERCASE** for SQL keywords (e.g., `SELECT`, `FROM`, `WHERE`).
- Use **PascalCase** for table names (e.g., `UserAccounts`, `GameSessions`).
- Use **snake_case** for field names (e.g., `first_name`, `created_at`).
- Use **plural** names for tables (e.g., `Users`, `Orders`).
- Define primary keys as `id INTEGER PRIMARY KEY`. Do not use `AUTO_INCREMENT`—it is not needed in SQLite.
- Use `<entity>_id` for foreign key fields (e.g., `user_id`, `order_id`).
- Consider cascading behavior for foreign keys carefully.
- Use `NOT NULL` in most fields unless null values are definitely needed.
- Use `TEXT` for string fields, `INTEGER` for whole numbers, `REAL` for floating-point numbers, and `BLOB` for binary
  data.
