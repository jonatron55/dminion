// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! Player CRUD operations for the campaign database.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{CampaignDb, DbError};

/// A player character stored in the database.
///
/// Represents a persistent character that can be added to encounters.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRecord {
    /// Database ID.
    pub id: i64,

    /// Party ID (nullable).
    pub party_id: Option<i64>,

    /// Display name.
    pub name: String,

    /// Strength score.
    pub str: i32,

    /// Dexterity score.
    pub dex: i32,

    /// Constitution score.
    pub con: i32,

    /// Intelligence score.
    pub int: i32,

    /// Wisdom score.
    pub wis: i32,

    /// Charisma score.
    pub cha: i32,

    /// Armor class.
    pub ac: i32,

    /// Initiative bonus.
    pub initiative_bonus: i32,

    /// Portrait base name (without path or extension).
    pub portrait: Option<String>,

    /// Notes about this player.
    pub notes: Option<String>,
}

/// A player class entry (for multiclass support).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PlayerClassRecord {
    /// Database ID.
    pub id: i64,

    /// Player ID this class belongs to.
    pub player_id: i64,

    /// Class name.
    pub name: String,

    /// Level in this class.
    pub level: i32,
}

/// Data for creating or updating a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerData {
    /// Party ID (nullable).
    pub party_id: Option<i64>,

    /// Display name.
    pub name: String,

    /// Strength score.
    pub str: i32,

    /// Dexterity score.
    pub dex: i32,

    /// Constitution score.
    pub con: i32,

    /// Intelligence score.
    pub int: i32,

    /// Wisdom score.
    pub wis: i32,

    /// Charisma score.
    pub cha: i32,

    /// Armor class.
    pub ac: i32,

    /// Initiative bonus.
    pub initiative_bonus: i32,

    /// Portrait base name (without path or extension).
    pub portrait: Option<String>,

    /// Notes about this player.
    pub notes: Option<String>,
}

/// Data for creating or updating a player class.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerClassData {
    /// Class name.
    pub name: String,

    /// Level in this class.
    pub level: i32,
}

impl CampaignDb {
    /// Inserts a new player into the database.
    ///
    /// Returns the ID of the newly created player.
    pub async fn insert_player(&self, player: &PlayerData) -> Result<i64, DbError> {
        let result = sqlx::query(
            r#"
            INSERT INTO Player (
                party_id, name, str, dex, con, int, wis, cha,
                ac, initiative_bonus, portrait, notes
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(player.party_id)
        .bind(&player.name)
        .bind(player.str)
        .bind(player.dex)
        .bind(player.con)
        .bind(player.int)
        .bind(player.wis)
        .bind(player.cha)
        .bind(player.ac)
        .bind(player.initiative_bonus)
        .bind(&player.portrait)
        .bind(&player.notes)
        .execute(self.pool())
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Gets a player by ID.
    ///
    /// Returns `DbError::PlayerNotFound` if the player doesn't exist.
    pub async fn get_player(&self, id: i64) -> Result<PlayerRecord, DbError> {
        sqlx::query_as::<_, PlayerRecord>(
            r#"
            SELECT
                id, party_id, name,
                str, dex, con, int, wis, cha,
                ac, initiative_bonus, portrait, notes
            FROM Player
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool())
        .await?
        .ok_or(DbError::PlayerNotFound(id))
    }

    /// Lists all players in the database.
    ///
    /// Returns players sorted by name.
    pub async fn list_players(&self) -> Result<Vec<PlayerRecord>, DbError> {
        let players = sqlx::query_as::<_, PlayerRecord>(
            r#"
            SELECT
                id, party_id, name,
                str, dex, con, int, wis, cha,
                ac, initiative_bonus, portrait, notes
            FROM Player
            ORDER BY name ASC
            "#,
        )
        .fetch_all(self.pool())
        .await?;

        Ok(players)
    }

    /// Lists all players in a specific party.
    ///
    /// Returns players sorted by name.
    pub async fn list_players_in_party(&self, party_id: i64) -> Result<Vec<PlayerRecord>, DbError> {
        let players = sqlx::query_as::<_, PlayerRecord>(
            r#"
            SELECT
                id, party_id, name,
                str, dex, con, int, wis, cha,
                ac, initiative_bonus, portrait, notes
            FROM Player
            WHERE party_id = ?
            ORDER BY name ASC
            "#,
        )
        .bind(party_id)
        .fetch_all(self.pool())
        .await?;

        Ok(players)
    }

    /// Updates an existing player.
    ///
    /// Returns `DbError::PlayerNotFound` if the player doesn't exist.
    pub async fn update_player(&self, id: i64, player: &PlayerData) -> Result<(), DbError> {
        let result = sqlx::query(
            r#"
            UPDATE Player SET
                party_id = ?, name = ?,
                str = ?, dex = ?, con = ?, int = ?, wis = ?, cha = ?,
                ac = ?, initiative_bonus = ?, portrait = ?, notes = ?
            WHERE id = ?
            "#,
        )
        .bind(player.party_id)
        .bind(&player.name)
        .bind(player.str)
        .bind(player.dex)
        .bind(player.con)
        .bind(player.int)
        .bind(player.wis)
        .bind(player.cha)
        .bind(player.ac)
        .bind(player.initiative_bonus)
        .bind(&player.portrait)
        .bind(&player.notes)
        .bind(id)
        .execute(self.pool())
        .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::PlayerNotFound(id));
        }

        Ok(())
    }

    /// Deletes a player by ID.
    ///
    /// Returns `DbError::PlayerNotFound` if the player doesn't exist.
    /// Cascades to delete all associated PlayerClass entries.
    pub async fn delete_player(&self, id: i64) -> Result<(), DbError> {
        let result = sqlx::query("DELETE FROM Player WHERE id = ?")
            .bind(id)
            .execute(self.pool())
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::PlayerNotFound(id));
        }

        Ok(())
    }

    /// Adds a class to a player.
    ///
    /// Returns the ID of the newly created class entry.
    pub async fn add_player_class(
        &self,
        player_id: i64,
        class: &PlayerClassData,
    ) -> Result<i64, DbError> {
        let result = sqlx::query(
            r#"
            INSERT INTO PlayerClass (player_id, name, level)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(player_id)
        .bind(&class.name)
        .bind(class.level)
        .execute(self.pool())
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Lists all classes for a player.
    ///
    /// Returns classes sorted by name.
    pub async fn list_player_classes(
        &self,
        player_id: i64,
    ) -> Result<Vec<PlayerClassRecord>, DbError> {
        let classes = sqlx::query_as::<_, PlayerClassRecord>(
            r#"
            SELECT id, player_id, name, level
            FROM PlayerClass
            WHERE player_id = ?
            ORDER BY name ASC
            "#,
        )
        .bind(player_id)
        .fetch_all(self.pool())
        .await?;

        Ok(classes)
    }

    /// Updates a player class entry.
    pub async fn update_player_class(
        &self,
        id: i64,
        class: &PlayerClassData,
    ) -> Result<(), DbError> {
        let result = sqlx::query(
            r#"
            UPDATE PlayerClass SET
                name = ?, level = ?
            WHERE id = ?
            "#,
        )
        .bind(&class.name)
        .bind(class.level)
        .bind(id)
        .execute(self.pool())
        .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!("PlayerClass {}", id).into()));
        }

        Ok(())
    }

    /// Deletes a player class entry.
    pub async fn delete_player_class(&self, id: i64) -> Result<(), DbError> {
        let result = sqlx::query("DELETE FROM PlayerClass WHERE id = ?")
            .bind(id)
            .execute(self.pool())
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!("PlayerClass {}", id).into()));
        }

        Ok(())
    }
}
