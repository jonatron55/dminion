use std::vec;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::dice::DiceExpr;

use super::{conditions, Condition, Stats};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterDef {
    pub name: String,
    pub subtype: String,
    pub stats: Stats,
    pub cr: u32,
    pub ac: u32,
    pub initiative_bonus: i32,
    pub legendary_actions: u32,
    pub portrait: String,
    pub notes: String,
    pub hit_dice: DiceExpr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monster {
    pub def: MonsterDef,
    pub name: String,
    pub initiative: i32,
    pub action: bool,
    pub hp: i32,
    pub reaction: bool,
    pub bonus_action: bool,
    pub legendary_actions: u32,
    pub notes: String,
    pub tiebreaker: i32,
    pub conditions: Vec<Condition>,
}

impl MonsterDef {
    pub fn spawn<TRng: Rng>(&self, rng: &mut TRng) -> Monster {
        Monster {
            name: self.name.clone(),
            def: self.clone(),
            hp: self.hit_dice.roll(rng).unwrap().value,
            notes: String::new(),
            action: true,
            reaction: true,
            bonus_action: true,
            legendary_actions: self.legendary_actions,
            initiative: 0,
            tiebreaker: ((self.stats.dex & 0xFF) << 24 | (rng.gen::<u32>() & 0x00FF_FFFF)) as i32,
            conditions: vec![],
        }
    }
}

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
}
