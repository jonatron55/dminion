// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lair {
    pub name: String,
    pub notes: String,
    pub action: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_portrait: Option<String>,
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
