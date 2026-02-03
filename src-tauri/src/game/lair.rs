// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// A lair instance in an encounter.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Lair {
    /// Display name of the lair.
    pub name: String,

    /// Free-form notes about the lair.
    pub notes: String,

    /// Whether the lair's standard action is available.
    pub action: bool,

    /// Path to small portrait image file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_portrait: Option<String>,

    /// Path to full portrait image file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_portrait: Option<String>,
}

impl Lair {
    pub fn set_action(&mut self, action: super::Action, available: bool) -> Result<(), ()> {
        match action {
            super::Action::Standard => self.action = available,
            super::Action::Bonus => return Err(()),
            super::Action::Reaction => return Err(()),
            super::Action::Legendary { .. } => return Err(()),
        }

        Ok(())
    }
}
