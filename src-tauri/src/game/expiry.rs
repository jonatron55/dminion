// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, time::Duration as StdDuration};

use crate::game::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Expiry {
    None,
    NextTurnStart,
    NextTurnEnd,
    Duration(Duration),
}

impl Ord for Expiry {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Expiry::None, Expiry::None) => Ordering::Equal,
            (Expiry::None, _) => Ordering::Greater,
            (_, Expiry::None) => Ordering::Less,
            (Expiry::Duration(a), Expiry::Duration(b)) => a.cmp(b),
            (Expiry::Duration(_), _) => Ordering::Greater,
            (_, Expiry::Duration(_)) => Ordering::Less,
            (Expiry::NextTurnStart, Expiry::NextTurnStart) => Ordering::Equal,
            (Expiry::NextTurnStart, Expiry::NextTurnEnd) => Ordering::Greater,
            (Expiry::NextTurnEnd, Expiry::NextTurnStart) => Ordering::Less,
            (Expiry::NextTurnEnd, Expiry::NextTurnEnd) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Expiry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TryInto<StdDuration> for Expiry {
    type Error = ();

    fn try_into(self) -> Result<StdDuration, Self::Error> {
        match self {
            Expiry::None => Err(()),
            Expiry::NextTurnStart => Err(()),
            Expiry::NextTurnEnd => Err(()),
            Expiry::Duration(duration) => Ok(StdDuration::from_secs(duration.total_secs() as u64)),
        }
    }
}
