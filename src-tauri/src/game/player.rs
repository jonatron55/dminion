use std::path::PathBuf;

use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{Class, Stats};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerDef {
    pub name: String,
    pub classes: Vec<Class>,
    pub stats: Stats,
    pub ac: u32,
    pub initiative_bonus: i32,
    pub portrait: PathBuf,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub def: PlayerDef,
    pub initiative: i32,
    pub action: bool,
    pub reaction: bool,
    pub bonus_action: bool,
    pub notes: String,
    pub tiebreaker: i32,
}

impl PlayerDef {
    pub fn spawn<TRng: Rng>(&self, rng: &mut TRng) -> Player {
        Player {
            def: self.clone(),
            notes: String::new(),
            action: true,
            reaction: true,
            bonus_action: true,
            initiative: 0,
            tiebreaker: ((self.stats.dex & 0xFF) << 24 | (rng.gen::<u32>() & 0x00FF_FFFF)) as i32,
        }
    }

    pub fn total_level(&self) -> u32 {
        self.classes.iter().map(|c| c.level).sum()
    }
}
