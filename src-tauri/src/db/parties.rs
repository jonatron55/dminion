// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! Party CRUD operations for the campaign database.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{CampaignDb, DbError};

/// A party (group of players) stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PartyRecord {
    /// Database ID.
    pub id: i64,

    /// Party name.
    pub name: String,
}

/// Data for creating or updating a party.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartyData {
    /// Party name.
    pub name: String,
}

impl CampaignDb {
    /// Inserts a new party into the database.
    ///
    /// Returns the ID of the newly created party.
    pub async fn insert_party(&self, party: &PartyData) -> Result<i64, DbError> {
        let result = sqlx::query(
            r#"
            INSERT INTO Party (name)
            VALUES (?)
            "#,
        )
        .bind(&party.name)
        .execute(self.pool())
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Gets a party by ID.
    ///
    /// Returns `DbError::NotFound` if the party doesn't exist.
    pub async fn get_party(&self, id: i64) -> Result<PartyRecord, DbError> {
        sqlx::query_as::<_, PartyRecord>(
            r#"
            SELECT id, name
            FROM Party
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool())
        .await?
        .ok_or(DbError::NotFound(format!("Party {}", id).into()))
    }

    /// Lists all parties in the database.
    ///
    /// Returns parties sorted by name.
    pub async fn list_parties(&self) -> Result<Vec<PartyRecord>, DbError> {
        let parties = sqlx::query_as::<_, PartyRecord>(
            r#"
            SELECT id, name
            FROM Party
            ORDER BY name ASC
            "#,
        )
        .fetch_all(self.pool())
        .await?;

        Ok(parties)
    }

    /// Updates an existing party.
    ///
    /// Returns `DbError::NotFound` if the party doesn't exist.
    pub async fn update_party(&self, id: i64, party: &PartyData) -> Result<(), DbError> {
        let result = sqlx::query(
            r#"
            UPDATE Party SET
                name = ?
            WHERE id = ?
            "#,
        )
        .bind(&party.name)
        .bind(id)
        .execute(self.pool())
        .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!("Party {}", id).into()));
        }

        Ok(())
    }

    /// Deletes a party by ID.
    ///
    /// Returns `DbError::NotFound` if the party doesn't exist.
    /// Sets party_id to NULL for all players in this party (ON DELETE SET NULL).
    pub async fn delete_party(&self, id: i64) -> Result<(), DbError> {
        let result = sqlx::query("DELETE FROM Party WHERE id = ?")
            .bind(id)
            .execute(self.pool())
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!("Party {}", id).into()));
        }

        Ok(())
    }
}
