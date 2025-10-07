use serde::{Deserialize, Serialize};

use crate::game::{conditions::Condition, Action, Class, Stats};

#[derive(Debug, Clone)]
pub struct PlayerDef {
    pub name: String,
    pub classes: Vec<Class>,
    pub stats: Stats,
    pub ac: u32,
    pub initiative_bonus: u32,
    pub small_portrait: Option<String>,
    pub full_portrait: Option<String>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub name: String,
    pub classes: Vec<Class>,
    pub stats: Stats,
    pub ac: u32,
    pub initiative_bonus: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_portrait: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_portrait: Option<String>,
    pub initiative: u32,
    pub action: bool,
    pub reaction: bool,
    pub bonus_action: bool,
    pub notes: String,
    pub tiebreaker: i32,
    pub conditions: Vec<Condition>,
}

impl PlayerDef {
    pub fn total_level(&self) -> u32 {
        self.classes.iter().map(|c| c.level).sum()
    }
}

impl Player {
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

    pub fn end_turn(&mut self) {
        // self.conditions = self
        //     .conditions
        //     .into_iter()
        //     .filter_map(|c| c.end_turn())
        //     .collect();
    }

    pub fn set_action(&mut self, action: Action, available: bool) -> Result<(), ()> {
        match action {
            Action::Standard => self.action = available,
            Action::Bonus => self.bonus_action = available,
            Action::Reaction => self.reaction = available,
            Action::Legendary { .. } => return Err(()),
        }

        Ok(())
    }
}
