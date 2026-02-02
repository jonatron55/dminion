// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! Monster CRUD operations for the campaign database.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{CampaignDb, DbError};

/// A monster template stored in the database.
///
/// Represents a library entry for spawning monsters into encounters.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MonsterRecord {
    /// Database ID.
    pub id: i64,

    /// Display name.
    pub name: String,

    /// Monster subtype (e.g., "Small Humanoid (Goblin)").
    pub subtype: String,

    /// Strength score.
    pub str: u32,

    /// Dexterity score.
    pub dex: u32,

    /// Constitution score.
    pub con: u32,

    /// Intelligence score.
    pub int: u32,

    /// Wisdom score.
    pub wis: u32,

    /// Charisma score.
    pub cha: u32,

    /// Challenge rating.
    pub cr: u32,

    /// Armor class.
    pub ac: u32,

    /// Initiative bonus.
    pub initiative_bonus: i32,

    /// Hit dice expression (e.g., "3d6+3").
    pub hit_dice: String,

    /// Number of legendary actions per round.
    pub legendary_actions: i32,

    /// Portrait base name (without path or extension).
    pub portrait: Option<String>,

    /// Notes about this monster.
    pub notes: Option<String>,
}

/// Data for creating or updating a monster.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonsterData {
    /// Display name.
    pub name: String,

    /// Monster subtype (e.g., "Small Humanoid (Goblin)").
    pub subtype: String,

    /// Strength score.
    pub str: u32,

    /// Dexterity score.
    pub dex: u32,

    /// Constitution score.
    pub con: u32,

    /// Intelligence score.
    pub int: u32,

    /// Wisdom score.
    pub wis: u32,

    /// Charisma score.
    pub cha: u32,

    /// Challenge rating.
    pub cr: u32,

    /// Armor class.
    pub ac: u32,

    /// Initiative bonus.
    pub initiative_bonus: u32,

    /// Hit dice expression (e.g., "3d6+3").
    pub hit_dice: String,

    /// Number of legendary actions per round.
    pub legendary_actions: u32,

    /// Portrait base name (without path or extension).
    pub portrait: Option<String>,

    /// Notes about this monster.
    pub notes: Option<String>,
}

impl CampaignDb {
    /// Inserts a new monster into the database.
    ///
    /// Returns the ID of the newly created monster.
    pub async fn insert_monster(&self, monster: &MonsterData) -> Result<i64, DbError> {
        let result = sqlx::query(
            r#"
            INSERT INTO Monster (
                name, subtype, str, dex, con, int, wis, cha,
                cr, ac, initiative_bonus, hit_dice,
                legendary_actions, portrait, notes
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&monster.name)
        .bind(&monster.subtype)
        .bind(monster.str)
        .bind(monster.dex)
        .bind(monster.con)
        .bind(monster.int)
        .bind(monster.wis)
        .bind(monster.cha)
        .bind(monster.cr)
        .bind(monster.ac)
        .bind(monster.initiative_bonus)
        .bind(&monster.hit_dice)
        .bind(monster.legendary_actions)
        .bind(&monster.portrait)
        .bind(&monster.notes)
        .execute(self.pool())
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Gets a monster by ID.
    ///
    /// Returns `DbError::MonsterNotFound` if the monster doesn't exist.
    pub async fn get_monster(&self, id: i64) -> Result<MonsterRecord, DbError> {
        sqlx::query_as::<_, MonsterRecord>(
            r#"
            SELECT
                id, name, subtype,
                str, dex, con, int, wis, cha,
                cr, ac, initiative_bonus, hit_dice,
                legendary_actions, portrait, notes
            FROM Monster
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool())
        .await?
        .ok_or(DbError::MonsterNotFound(id))
    }

    /// Lists all monsters in the database.
    ///
    /// Returns monsters sorted by name.
    pub async fn list_monsters(&self) -> Result<Vec<MonsterRecord>, DbError> {
        let monsters = sqlx::query_as::<_, MonsterRecord>(
            r#"
            SELECT
                id, name, subtype,
                str, dex, con, int, wis, cha,
                cr, ac, initiative_bonus, hit_dice,
                legendary_actions, portrait, notes
            FROM Monster
            ORDER BY name ASC
            "#,
        )
        .fetch_all(self.pool())
        .await?;

        Ok(monsters)
    }

    /// Updates an existing monster.
    ///
    /// Returns `DbError::MonsterNotFound` if the monster doesn't exist.
    pub async fn update_monster(&self, id: i64, monster: &MonsterData) -> Result<(), DbError> {
        let result = sqlx::query(
            r#"
            UPDATE Monster SET
                name = ?, subtype = ?,
                str = ?, dex = ?, con = ?, int = ?, wis = ?, cha = ?,
                cr = ?, ac = ?, initiative_bonus = ?, hit_dice = ?,
                legendary_actions = ?, portrait = ?, notes = ?
            WHERE id = ?
            "#,
        )
        .bind(&monster.name)
        .bind(&monster.subtype)
        .bind(monster.str)
        .bind(monster.dex)
        .bind(monster.con)
        .bind(monster.int)
        .bind(monster.wis)
        .bind(monster.cha)
        .bind(monster.cr)
        .bind(monster.ac)
        .bind(monster.initiative_bonus)
        .bind(&monster.hit_dice)
        .bind(monster.legendary_actions)
        .bind(&monster.portrait)
        .bind(&monster.notes)
        .bind(id)
        .execute(self.pool())
        .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::MonsterNotFound(id));
        }

        Ok(())
    }

    /// Deletes a monster by ID.
    ///
    /// Returns `DbError::MonsterNotFound` if the monster doesn't exist.
    pub async fn delete_monster(&self, id: i64) -> Result<(), DbError> {
        let result = sqlx::query("DELETE FROM Monster WHERE id = ?")
            .bind(id)
            .execute(self.pool())
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::MonsterNotFound(id));
        }

        Ok(())
    }
}
