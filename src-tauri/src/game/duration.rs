use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, time::Duration as StdDuration};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Duration {
    Indefinite,
    StartOfTurn,
    EndOfTurn,
    Seconds(u32),
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Duration::Indefinite, Duration::Indefinite) => Ordering::Equal,
            (Duration::Indefinite, _) => Ordering::Greater,
            (_, Duration::Indefinite) => Ordering::Less,
            (Duration::Seconds(a), Duration::Seconds(b)) => a.cmp(b),
            (Duration::Seconds(_), _) => Ordering::Greater,
            (_, Duration::Seconds(_)) => Ordering::Less,
            (Duration::StartOfTurn, Duration::StartOfTurn) => Ordering::Equal,
            (Duration::StartOfTurn, Duration::EndOfTurn) => Ordering::Greater,
            (Duration::EndOfTurn, Duration::StartOfTurn) => Ordering::Less,
            (Duration::EndOfTurn, Duration::EndOfTurn) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<StdDuration> for Duration {
    fn into(self) -> StdDuration {
        match self {
            Duration::Indefinite => StdDuration::MAX,
            Duration::StartOfTurn => StdDuration::from_secs(6),
            Duration::EndOfTurn => StdDuration::from_secs(6),
            Duration::Seconds(s) => StdDuration::from_secs(s as u64),
        }
    }
}
