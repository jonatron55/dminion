// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::dice::{roll, DiceExpr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub str: u32,
    pub dex: u32,
    pub con: u32,
    pub int: u32,
    pub wis: u32,
    pub cha: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
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
