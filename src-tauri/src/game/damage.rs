// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Damage {
    Damage { amount: u32 },
    HalfDamage { amount: u32 },
    DoubleDamage { amount: u32 },
    Kill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Healing {
    Heal { amount: u32 },
    SetHp { amount: u32 },
    SetTempHp { amount: u32 },
}
