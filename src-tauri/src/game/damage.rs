// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Represents damage that can be applied to a participant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Damage {
    /// Reduce HP by `amount`.
    Damage { amount: u32 },

    /// Reduce HP by half `amount` (rounded down).
    HalfDamage { amount: u32 },

    /// Reduce HP by double `amount`.
    DoubleDamage { amount: u32 },

    /// Instantly set HP to 0.
    Kill,
}

/// Represents healing that can be applied to a participant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Healing {
    /// Increase HP by `amount`.
    Heal { amount: u32 },

    /// Set HP to `amount`.
    SetHp { amount: u32 },

    /// Set temporary HP to `amount`.
    SetTempHp { amount: u32 },
}
