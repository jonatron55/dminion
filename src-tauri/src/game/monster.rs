use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::dice::roll;

use super::Stats;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterDef {
    pub name: String,
    pub subtype: String,
    pub stats: Stats,
    pub cr: u32,
    pub ac: u32,
    pub initiative_bonus: i32,
    pub hit_dice: String,
    pub legendary_actions: u32,
    pub portrait: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monster {
    pub def: MonsterDef,
    pub initiative: i32,
    pub action: bool,
    pub hp: i32,
    pub reaction: bool,
    pub bonus_action: bool,
    pub legendary_actions: u32,
    pub notes: String,
    pub tiebreaker: i32,
}


impl MonsterDef {
    pub fn spawn<TRng: Rng>(&self, rng: &mut TRng) -> Monster {
        Monster {
            def: self.clone(),
            hp: roll(&self.hit_dice, rng).unwrap(),
            notes: String::new(),
            action: true,
            reaction: true,
            bonus_action: true,
            legendary_actions: self.legendary_actions,
            initiative: 0,
            tiebreaker: ((self.stats.dex & 0xFF) << 24 | (rng.gen::<u32>() & 0x00FF_FFFF)) as i32,
        }
    }
}
