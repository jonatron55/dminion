// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use rand::Rng;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::dice::{roll, DiceExpr};

/// Represents a set of character ability scores.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Stats {
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
}

/// Represents a character class and its level.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Class {
    /// Name of the class.
    pub name: String,

    /// Level in the class.
    pub level: u32,
}

impl Stats {
    pub fn roll<TRng: Rng>(rng: &mut TRng) -> Self {
        let dice = DiceExpr::parse("4d6kh3").unwrap();

        Stats {
            str: dice.roll(rng).unwrap().value as u32,
            dex: dice.roll(rng).unwrap().value as u32,
            con: dice.roll(rng).unwrap().value as u32,
            int: dice.roll(rng).unwrap().value as u32,
            wis: dice.roll(rng).unwrap().value as u32,
            cha: dice.roll(rng).unwrap().value as u32,
        }
    }
}

pub fn modifier(score: u32) -> i32 {
    (score as i32 - 10) / 2
}
