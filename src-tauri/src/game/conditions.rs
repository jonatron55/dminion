// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

#![allow(unused)]

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

use crate::game::{
    time::{Duration, Time},
    Expiry, ParticipantId,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub name: String,
    pub start_time: Time,
    pub expiry: Expiry,
    #[serde(default)]
    pub instigator: Option<ParticipantId>,
}

pub const BLINDED: &'static str = "blinded";
pub const BLOODIED: &'static str = "bloodied";
pub const CHARMED: &'static str = "charmed";
pub const CONCENTRATING: &'static str = "concentrating";
pub const DEAD: &'static str = "dead";
pub const DEAFENED: &'static str = "deafened";
pub const FRIGHTENED: &'static str = "frightened";
pub const GRAPPLED: &'static str = "grappled";
pub const INCAPACITATED: &'static str = "incapacitated";
pub const INVISIBLE: &'static str = "invisible";
pub const MARKED: &'static str = "marked";
pub const PARALYZED: &'static str = "paralyzed";
pub const PETRIFIED: &'static str = "petrified";
pub const POISONED: &'static str = "poisoned";
pub const PRONE: &'static str = "prone";
pub const RESTRAINED: &'static str = "restrained";
pub const STUNNED: &'static str = "stunned";
pub const SURPRISED: &'static str = "surprised";
pub const UNCONSCIOUS: &'static str = "unconscious";

impl Condition {
    pub fn new(name: String, start_time: Time) -> Self {
        Self {
            name,
            start_time: start_time,
            expiry: Expiry::None,
            instigator: None,
        }
    }

    pub fn with_expiry(mut self, expiry: Expiry) -> Self {
        self.expiry = expiry;
        self
    }

    pub fn with_instigator(mut self, instigator: ParticipantId) -> Self {
        self.instigator = Some(instigator);
        self
    }

    pub fn blinded(start_time: Time) -> Self {
        Self::new(BLINDED.into(), start_time)
    }

    pub fn bloodied(start_time: Time) -> Self {
        Self::new(BLOODIED.into(), start_time)
    }

    pub fn charmed(start_time: Time) -> Self {
        Self::new(CHARMED.into(), start_time)
    }

    pub fn concentrating(start_time: Time) -> Self {
        Self::new(CONCENTRATING.into(), start_time)
    }

    pub fn dead(start_time: Time) -> Self {
        Self::new(DEAD.into(), start_time)
    }

    pub fn deafened(start_time: Time) -> Self {
        Self::new(DEAFENED.into(), start_time)
    }

    pub fn frightened(start_time: Time) -> Self {
        Self::new(FRIGHTENED.into(), start_time)
    }

    pub fn grappled(start_time: Time) -> Self {
        Self::new(GRAPPLED.into(), start_time)
    }

    pub fn incapacitated(start_time: Time) -> Self {
        Self::new(INCAPACITATED.into(), start_time)
    }

    pub fn invisible(start_time: Time) -> Self {
        Self::new(INVISIBLE.into(), start_time)
    }

    pub fn marked(start_time: Time) -> Self {
        Self::new(MARKED.into(), start_time)
    }

    pub fn paralyzed(start_time: Time) -> Self {
        Self::new(PARALYZED.into(), start_time)
    }

    pub fn petrified(start_time: Time) -> Self {
        Self::new(PETRIFIED.into(), start_time)
    }

    pub fn poisoned(start_time: Time) -> Self {
        Self::new(POISONED.into(), start_time)
    }

    pub fn prone(start_time: Time) -> Self {
        Self::new(PRONE.into(), start_time)
    }

    pub fn restrained(start_time: Time) -> Self {
        Self::new(RESTRAINED.into(), start_time)
    }

    pub fn stunned(start_time: Time) -> Self {
        Self::new(STUNNED.into(), start_time)
    }

    pub fn surprised(start_time: Time) -> Self {
        Self::new(SURPRISED.into(), start_time)
    }

    pub fn unconscious(start_time: Time) -> Self {
        Self::new(UNCONSCIOUS.into(), start_time)
    }

    pub fn expired_on_turn_start(&self, game_time: Time) -> bool {
        match self.expiry {
            Expiry::NextTurnStart => true,
            Expiry::Duration(duration) => game_time >= self.start_time + duration,
            _ => false,
        }
    }

    pub fn implications(&self) -> Vec<Condition> {
        match self.name.as_str() {
            GRAPPLED => vec![Self::restrained(self.start_time)],
            PARALYZED => vec![Self::incapacitated(self.start_time)],
            PETRIFIED => vec![
                Self::incapacitated(self.start_time),
                Self::unconscious(self.start_time),
            ],
            STUNNED => vec![Self::incapacitated(self.start_time)],
            UNCONSCIOUS => vec![
                Self::incapacitated(self.start_time),
                Self::prone(self.start_time),
            ],
            _ => vec![],
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.name)
    }
}

impl From<Duration> for Expiry {
    fn from(duration: Duration) -> Self {
        Expiry::Duration(duration)
    }
}
