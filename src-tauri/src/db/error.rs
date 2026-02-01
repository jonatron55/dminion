// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! Database-related error types.

use std::path::PathBuf;

use sqlx::{migrate::MigrateError, Error as SqlxError};
use thiserror::Error;

/// Errors that can occur during database operations.
#[derive(Debug, Error)]
pub enum DbError {
    /// SQLx database error.
    #[error("database error: {0}")]
    Sqlx(#[from] SqlxError),

    /// Failed to run migrations.
    #[error("migration error: {0}")]
    Migration(#[from] MigrateError),

    /// Monster not found by ID.
    #[error("monster not found: {0}")]
    MonsterNotFound(i64),

    /// Player not found by ID.
    #[error("player not found: {0}")]
    PlayerNotFound(i64),

    /// Encounter not found by ID.
    #[error("encounter not found: {0}")]
    EncounterNotFound(i64),

    /// Database file not found.
    #[error("database not found: {0}")]
    NotFound(PathBuf),
}
