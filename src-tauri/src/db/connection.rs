// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! Database connection management for campaigns.

use std::path::Path;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::SqlitePool;

use super::DbError;

/// Database connection for a campaign.
///
/// Wraps a `SqlitePool` with campaign-specific configuration.
#[derive(Debug, Clone)]
pub struct CampaignDb {
    pool: SqlitePool,
}

impl CampaignDb {
    /// Opens or creates a campaign database at the given path.
    ///
    /// Uses WAL mode for better concurrent read performance and
    /// creates the database file if it doesn't exist.
    pub async fn open(path: &Path) -> Result<Self, DbError> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        Ok(Self { pool })
    }

    /// Opens an existing campaign database. Returns error if file doesn't exist.
    pub async fn open_existing(path: &Path) -> Result<Self, DbError> {
        if !path.exists() {
            return Err(DbError::NotFound(path.to_path_buf()));
        }

        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(false)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        Ok(Self { pool })
    }

    /// Returns a reference to the underlying connection pool.
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Runs the embedded migrations to ensure schema is up to date.
    pub async fn migrate(&self) -> Result<(), DbError> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    /// Closes the database connection pool.
    pub async fn close(&self) {
        self.pool.close().await;
    }
}
