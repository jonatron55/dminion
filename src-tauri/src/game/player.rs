// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::game::{conditions::Condition, Action, Class, Stats};

/// A player instance in an encounter.
///
/// Created from a database [`PlayerRecord`] with rolled initiative. Tracks runtime state like current HP, conditions,
/// and action usage.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    /// Display name of the player.
    pub name: String,

    /// Character classes and levels.
    pub classes: Vec<Class>,

    /// Player base statistics.
    pub stats: Stats,

    /// Armor class.
    pub ac: u32,

    /// Any bonuses to initiative beyond the Dexterity modifier.
    pub initiative_bonus: u32,

    /// Path to small portrait image file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_portrait: Option<String>,

    /// Path to full portrait image file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_portrait: Option<String>,

    /// Rolled initiative for this encounter.
    pub initiative: u32,

    /// Tiebreaker value for initiative ties.
    pub tiebreaker: i32,

    /// Whether the player's standard action is available.
    pub action: bool,

    /// Whether the player's reaction is available.
    pub reaction: bool,

    /// Whether the player's bonus action is available.
    pub bonus_action: bool,

    /// Free-form notes about the player.
    pub notes: String,

    /// Active conditions affecting the player.
    pub conditions: Vec<Condition>,
}

impl Player {
    pub fn total_level(&self) -> u32 {
        self.classes.iter().map(|c| c.level).sum()
    }

    pub fn begin_turn(&mut self) {
        self.action = true;
        self.reaction = true;
        self.bonus_action = true;
        // self.conditions = self
        //     .conditions
        //     .into_iter()
        //     .filter_map(|c| c.begin_turn())
        //     .collect();
    }

    pub fn end_turn(&mut self) {
        // self.conditions = self
        //     .conditions
        //     .into_iter()
        //     .filter_map(|c| c.end_turn())
        //     .collect();
    }

    pub fn set_action(&mut self, action: Action, available: bool) -> Result<(), ()> {
        match action {
            Action::Standard => self.action = available,
            Action::Bonus => self.bonus_action = available,
            Action::Reaction => self.reaction = available,
            Action::Legendary { .. } => return Err(()),
        }

        Ok(())
    }
}
