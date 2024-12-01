use chrono::{DateTime, Duration, Utc};
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};

use super::{Participant, XP_PER_CR};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub participants: Vec<Participant>,
    pub round: u32,
    pub turn: u32,
    pub game_started: DateTime<Utc>,
    pub turn_started: DateTime<Utc>,

    #[serde(skip)]
    pub rng: ThreadRng,
}

impl Game {
    pub fn new(mut participants: Vec<Participant>) -> Self {
        participants.sort();
        Self {
            participants,
            round: 0,
            turn: 0,
            game_started: Utc::now(),
            turn_started: Utc::now(),
            rng: rand::thread_rng(),
        }
    }

    pub fn spawn(&mut self, participant: Participant) {
        self.participants.push(participant);
        self.participants.sort();
    }

    // pub fn realtime_duration(&self) -> Duration {
    //     Utc::now().signed_duration_since(self.game_started)
    // }

    // pub fn game_duration(&self) -> Duration {
    //     Duration::seconds(self.round as i64 * 6)
    // }

    // pub fn turn_duration(&self) -> Duration {
    //     Utc::now().signed_duration_since(self.turn_started)
    // }

    // pub fn player_count(&self) -> usize {
    //     self.participants
    //         .iter()
    //         .filter(|p| matches!(p, Participant::Player(_)))
    //         .count()
    // }

    // pub fn monster_count(&self) -> usize {
    //     self.participants
    //         .iter()
    //         .filter(|p| matches!(p, Participant::Monster(_)))
    //         .count()
    // }

    // pub fn average_player_level(&self) -> f64 {
    //     let player_count = self.player_count();
    //     if player_count == 0 {
    //         return 0.0;
    //     }

    //     self.participants
    //         .iter()
    //         .map(|c| match c {
    //             Participant::Player(p) => p.def.total_level(),
    //             _ => 0,
    //         })
    //         .sum::<u32>() as f64
    //         / player_count as f64
    // }

    // pub fn average_monster_challenge(&self) -> f64 {
    //     let monster_count = self.monster_count();
    //     if monster_count == 0 {
    //         return 0.0;
    //     }

    //     self.participants
    //         .iter()
    //         .map(|c| match c {
    //             Participant::Monster(m) => m.def.cr,
    //             _ => 0,
    //         })
    //         .sum::<u32>() as f64
    //         / monster_count as f64
    // }

    // pub fn total_xp(&self) -> u32 {
    //     self.participants
    //         .iter()
    //         .map(|c| match c {
    //             Participant::Monster(m) => XP_PER_CR[(m.def.cr as usize).max(XP_PER_CR.len() - 1)],
    //             _ => 0,
    //         })
    //         .sum()
    // }
}
