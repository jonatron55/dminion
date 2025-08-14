use std::{collections::HashMap, vec};

use chrono::{DateTime, Duration, Utc};
use rand::{rngs::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};

use super::{Participant, XP_PER_CR};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub participants: HashMap<u32, Participant>,
    pub order: Vec<u32>,
    pub round: u32,
    pub turn: u32,
    pub game_started: DateTime<Utc>,
    pub turn_started: DateTime<Utc>,

    #[serde(skip, default = "rand::rngs::StdRng::from_entropy")]
    pub rng: StdRng,

    next_id: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            order: vec![],
            participants: HashMap::new(),
            next_id: 1,
            round: 0,
            turn: 0,
            game_started: Utc::now(),
            turn_started: Utc::now(),
            rng: StdRng::from_entropy(),
        }
    }

    // pub fn with_participants(self, participants: Vec<Participant>) -> Self {
    //     let participant_count = participants.len();
    //     Self {
    //         order: participants
    //             .iter()
    //             .enumerate()
    //             .map(|(id, _)| (id + 1) as u32)
    //             .collect(),
    //         next_id: participants.len() as u32 + 1,
    //         participants: participants
    //             .into_iter()
    //             .enumerate()
    //             .map(|(i, p)| ((i + 1) as u32, p))
    //             .collect(),
    //         round: self.round,
    //         turn: self.turn.min(participant_count as u32 - 1),
    //         game_started: self.game_started,
    //         turn_started: self.turn_started,
    //         rng: self.rng,
    //     }
    // }

    pub fn spawn(&mut self, participant: Participant) -> u32 {
        self.participants.insert(self.next_id, participant);
        self.order.push(self.next_id);
        self.order.sort_by(|a, b| {
            self.participants
                .get(a)
                .unwrap()
                .partial_cmp(self.participants.get(b).unwrap())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        self.next_id += 1;
        self.next_id - 1
    }

    pub fn despawn(&mut self, id: u32) {
        self.participants.remove(&id);
        self.order.retain(|&x| x != id);
    }

    pub fn begin_play(&mut self) {
        self.turn = 0;
        self.round = 1;
        self.game_started = Utc::now();
        self.turn_started = Utc::now();
    }

    pub fn next_turn(&mut self) {
        if self.order.is_empty() {
            return;
        }

        if let Some(participant) = self.participants.get_mut(&self.order[self.turn as usize]) {
            participant.end_turn();
        }

        self.turn += 1;
        if self.turn >= self.order.len() as u32 {
            self.turn = 0;
            self.round += 1;
        }

        self.turn_started = Utc::now();

        if let Some(participant) = self.participants.get_mut(&self.order[self.turn as usize]) {
            participant.begin_turn();
        }
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

impl Default for Game {
    fn default() -> Self {
        Self {
            participants: HashMap::new(),
            order: Vec::new(),
            round: 0,
            turn: 0,
            game_started: Utc::now(),
            turn_started: Utc::now(),
            rng: StdRng::from_entropy(),
            next_id: 1,
        }
    }
}
