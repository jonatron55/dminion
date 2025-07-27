use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use super::{Lair, Monster, Player};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Participant {
    Lair(Lair),
    Monster(Monster),
    Player(Player),
}

impl Participant {
    pub fn name(&self) -> &str {
        match self {
            Participant::Lair(lair) => &lair.name,
            Participant::Monster(monster) => &monster.name,
            Participant::Player(player) => &player.name,
        }
    }

    pub fn initiative(&self) -> i32 {
        match self {
            Participant::Lair(_) => 20,
            Participant::Monster(monster) => monster.initiative,
            Participant::Player(player) => player.initiative,
        }
    }

    pub fn tiebreaker(&self) -> i32 {
        match self {
            Participant::Lair(_) => i32::MIN,
            Participant::Monster(monster) => monster.tiebreaker,
            Participant::Player(player) => player.tiebreaker,
        }
    }

    pub fn begin_turn(&mut self) {
        match self {
            Participant::Lair(_) => {}
            Participant::Monster(monster) => monster.begin_turn(),
            Participant::Player(player) => {}
        }
    }

    pub fn end_turn(&mut self) {}
}

impl PartialEq for Participant {
    fn eq(&self, other: &Self) -> bool {
        self.initiative() == other.initiative() && self.tiebreaker() == other.tiebreaker()
    }
}

impl Eq for Participant {}

impl PartialOrd for Participant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.initiative().partial_cmp(&other.initiative()) {
            Some(Ordering::Equal) => self.tiebreaker().partial_cmp(&other.tiebreaker()),
            other => other,
        }
    }
}

impl Ord for Participant {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.initiative().partial_cmp(&other.initiative()) {
            Some(Ordering::Equal) => self.tiebreaker().cmp(&other.tiebreaker()),
            Some(other) => other,
            None => unreachable!(),
        }
    }
}
