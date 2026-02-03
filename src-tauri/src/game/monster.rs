// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::game::{conditions, time::Time, Action, Damage, Healing};

use super::{Condition, Stats};

/// A monster instance in an encounter.
///
/// Created from a database [`MonsterRecord`] with rolled HP and initiative. Tracks runtime state like current HP,
/// conditions, and action usage.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Monster {
    /// Display name of the monster, including a possible instance number.
    pub name: String,

    /// Monster size and type, e.g. "Large beast".
    pub subtype: String,

    /// Monster base statistics.
    pub stats: Stats,

    /// Challenge rating index.
    ///
    /// Maps as follows:
    /// - 0 => 0
    /// - 1 => ⅛
    /// - 2 => ¼
    /// - 3 => ½
    /// - 4 => 1
    /// - etc.
    pub cr: u32,

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

    /// Current hit points.
    pub hp: i32,

    /// Current temporary hit points.
    pub temp_hp: i32,

    /// Maximum hit points.
    pub max_hp: i32,

    /// Whether the monster's standard action is available.
    pub action: bool,

    /// Whether the monster's reaction is available.
    pub reaction: bool,

    /// Whether the monster's bonus action is available.
    pub bonus_action: bool,

    /// Availability of legendary actions.
    pub legendary_actions: Vec<bool>,

    /// Number of legendary actions the monster can take per round.
    pub legendary_action_count: u32,

    /// Free-form notes about the monster.
    pub notes: String,

    /// Active conditions affecting the monster.
    pub conditions: Vec<Condition>,

    /// Whether the monster should be counted towards difficulty calculations.
    pub is_hostile: bool,
}

impl Monster {
    pub fn begin_turn(&mut self) {
        self.action = true;
        self.reaction = true;
        self.bonus_action = true;
        self.legendary_actions.fill(true);
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

    pub fn has_condition(&self, condition_name: &str) -> bool {
        self.conditions.iter().any(|c| c.name == condition_name)
    }

    pub fn set_action(&mut self, action: Action, available: bool) -> Result<(), ()> {
        match action {
            Action::Standard => self.action = available,
            Action::Bonus => self.bonus_action = available,
            Action::Reaction => self.reaction = available,
            Action::Legendary { index } => self.legendary_actions[index] = available,
        }

        Ok(())
    }

    pub fn damage(&mut self, time: Time, damage: Damage) {
        let damage_amount = match damage {
            Damage::Damage { amount } => amount as i32,
            Damage::HalfDamage { amount } => (amount / 2) as i32,
            Damage::DoubleDamage { amount } => (amount * 2) as i32,
            Damage::Kill => {
                self.hp = 0;
                self.temp_hp = 0;
                if !self.has_condition(conditions::BLOODIED) {
                    self.conditions.push(Condition::bloodied(time));
                }
                if !self.has_condition(conditions::DEAD) {
                    self.conditions.push(Condition::dead(time));
                }
                return;
            }
        };

        let remaining = if self.temp_hp > 0 {
            let temp_damage = damage_amount.min(self.temp_hp);
            self.temp_hp -= temp_damage;
            damage_amount - temp_damage
        } else {
            damage_amount
        };

        self.hp = self.hp.saturating_sub(remaining);

        if self.hp <= self.max_hp / 2 && !self.has_condition(conditions::BLOODIED) {
            self.conditions.push(Condition::bloodied(time));
        }

        if self.hp <= 0 && !self.has_condition(conditions::DEAD) {
            self.conditions.push(Condition::dead(time));
        }
    }

    pub fn heal(&mut self, healing: Healing) {
        match healing {
            Healing::Heal { amount } => {
                self.hp = (self.hp.max(0) + amount as i32).min(self.max_hp);
            }
            Healing::SetHp { amount } => {
                self.hp = amount as i32;
            }
            Healing::SetTempHp { amount } => {
                self.temp_hp = amount as i32;
            }
        };

        if self.hp > 0 {
            self.conditions.retain(|c| c.name != conditions::DEAD);
        }

        if self.hp > self.max_hp / 2 {
            self.conditions.retain(|c| c.name != conditions::BLOODIED);
        }
    }
}
