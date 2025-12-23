// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Sub},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Time {
    round: u32,
    initiative: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Duration {
    rounds: i32,
}

impl Time {
    pub fn new(round: u32, initiative: u32) -> Self {
        Time { round, initiative }
    }

    pub fn total_secs(&self) -> u32 {
        self.round * 6
    }

    pub fn mins(&self) -> u32 {
        self.total_secs() / 60
    }

    pub fn secs(&self) -> u32 {
        self.total_secs() % 60
    }
}

impl Duration {
    pub fn _from_rounds(rounds: i32) -> Self {
        Duration { rounds }
    }

    pub fn from_secs(seconds: i32) -> Self {
        Duration {
            rounds: seconds / 6,
        }
    }

    pub fn total_secs(&self) -> i32 {
        self.rounds * 6
    }

    pub fn mins(&self) -> i32 {
        self.total_secs() / 60
    }

    pub fn secs(&self) -> i32 {
        (self.total_secs() % 60).abs()
    }
}

impl Add<Duration> for Time {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Time {
            round: (self.round as i32 + rhs.rounds) as u32,
            initiative: self.initiative,
        }
    }
}

impl Sub<Time> for Time {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        Duration {
            rounds: self.round as i32 - rhs.round as i32,
        }
    }
}

impl Sub<Duration> for Time {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        Time {
            round: i32::min(self.round as i32 - rhs.rounds, 0) as u32,
            initiative: self.initiative,
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}:{:02}", self.mins(), self.secs())
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}:{:02}", self.mins(), self.secs())
    }
}
