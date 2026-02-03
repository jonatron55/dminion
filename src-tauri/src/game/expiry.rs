// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, time::Duration as StdDuration};
use ts_rs::TS;

use crate::game::time::Duration;

/// Options for when a condition or effect expires.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Expiry {
    /// Does not expire.
    None,

    /// Expires at the start of the instigator's next turn.
    NextTurnStart,

    /// Expires at the end of the instigator's next turn.
    NextTurnEnd,

    /// Expires after a certain number of rounds.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expiry_serialization() {
        let expiry_none = Expiry::None;
        let expiry_next_turn_start = Expiry::NextTurnStart;
        let expiry_next_turn_end = Expiry::NextTurnEnd;
        let expiry_duration = Expiry::Duration(Duration::from_secs(30));

        let json_none = serde_json::to_string(&expiry_none).unwrap();
        let json_next_turn_start = serde_json::to_string(&expiry_next_turn_start).unwrap();
        let json_next_turn_end = serde_json::to_string(&expiry_next_turn_end).unwrap();
        let json_duration = serde_json::to_string(&expiry_duration).unwrap();

        assert_eq!(json_none, r#"{"type":"none"}"#);
        assert_eq!(json_next_turn_start, r#"{"type":"nextTurnStart"}"#);
        assert_eq!(json_next_turn_end, r#"{"type":"nextTurnEnd"}"#);
        assert_eq!(json_duration, r#"{"type":"duration","rounds":5}"#);
    }
}
