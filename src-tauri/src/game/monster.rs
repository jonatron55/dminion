// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use serde::{Deserialize, Serialize};

use crate::{
    dice::DiceExpr,
    game::{conditions, time::Time, Action, Damage, Healing},
};

use super::{Condition, Stats};

#[derive(Debug, Clone)]
pub struct MonsterDef {
    pub name: String,
    pub subtype: String,
    pub stats: Stats,
    pub cr: u32,
    pub ac: u32,
    pub initiative_bonus: u32,
    pub hit_dice: DiceExpr,
    pub legendary_actions: u32,
    pub small_portrait: Option<String>,
    pub full_portrait: Option<String>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monster {
    pub name: String,
    pub subtype: String,
    pub stats: Stats,
    pub cr: u32,
    pub ac: u32,
    pub initiative_bonus: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_portrait: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_portrait: Option<String>,
    pub hit_dice: DiceExpr,
    pub initiative: u32,
    pub action: bool,
    pub hp: i32,
    pub temp_hp: i32,
    pub max_hp: i32,
    pub reaction: bool,
    pub bonus_action: bool,
    pub legendary_actions: Vec<bool>,
    pub legendary_action_count: u32,
    pub notes: String,
    pub tiebreaker: i32,
    pub conditions: Vec<Condition>,
    pub is_hostile: bool,
}

impl MonsterDef {}

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
