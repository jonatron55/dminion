use std::{path::PathBuf, vec};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::dice::DiceExpr;

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
    pub legendary_actions: u32,
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
}
