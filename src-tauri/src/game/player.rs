use std::path::PathBuf;

use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{conditions::Condition, Class, Stats};

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
    pub name: String,
    pub classes: Vec<Class>,
    pub stats: Stats,
    pub ac: u32,
    pub initiative_bonus: i32,
    pub portrait: PathBuf,
    pub initiative: i32,
    pub action: bool,
    pub reaction: bool,
    pub bonus_action: bool,
    pub notes: String,
    pub tiebreaker: i32,
    pub conditions: Vec<Condition>,
}

impl PlayerDef {
    pub fn spawn<TRng: Rng>(&self, rng: &mut TRng) -> Player {
        Player {
            name: self.name.clone(),
            classes: self.classes.clone(),
            stats: self.stats.clone(),
            ac: self.ac,
            initiative_bonus: self.initiative_bonus,
            portrait: self.portrait.clone(),
            initiative: 0,
            action: true,
            reaction: true,
            bonus_action: true,
            notes: self.notes.clone(),
            tiebreaker: ((self.stats.dex & 0xFF) << 24 | (rng.gen::<u32>() & 0x00FF_FFFF)) as i32,
            conditions: vec![],
        }
    }

    pub fn total_level(&self) -> u32 {
        self.classes.iter().map(|c| c.level).sum()
    }
}
