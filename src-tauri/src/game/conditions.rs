use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

use crate::game::time::{Duration, Time};

use super::Expiry;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub name: String,
    pub start_time: Time,
    pub expiry: Expiry,
    pub instigator: Option<String>,
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

    pub fn with_instigator(mut self, instigator: String) -> Self {
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

// TODO: move to frontend
// Condition::Blinded => " • A blinded creature can't see and automatically fails any ability check that requires sight.\n • Attack rolls against the creature have advantage, and the creature's attack rolls have disadvantage.",
// Condition::Bloodied => " • A creature becomes bloodied when its hit points are reduced below half their maximum.",
// Condition::Charmed => " • A charmed creature can’t attack the charmer or target the charmer with harmful abilities or magical effects.\n • The charmer has advantage on any ability check to interact socially with the creature.",
// Condition::Concentrating => " • A spellcaster loses concentration on a spell if it casts another spell that requires concentration. A spellcaster can’t concentrate on two spells at once.\n • Whenever a spellcaster takes damage while concentrating on a spell, they must make a Constitution saving throw to maintain their concentration. The DC equals 10 or half the damage taken, whichever number is higher. If the damage is taken from multiple sources, such as an arrow and a dragon's breath, the spellcaster makes a separate saving throw for each source of damage.\n • A spellcaster loses concentration on a spell if they are incapacitated or die.",
// Condition::Dead => " • This creature is no more.\n • It has ceased to be.\n • It’s a stiff.\n • It’s expired and gone to meet it’s maker.\n • Bereft of life, it rests in peace.\n • If you hadn’t vapourized it with a fireball, it’d be pushing up the daisies.\n • It’s metabolic processes are now of interest only to historians.\n • It’s kicked the bucket, shuffled off its mortal coil, run down the curtain, and joined the bleeding choir invisible.\n • This is an ex-creature.",
// Condition::Deafened => " • A deafened creature can’t hear and automatically fails any ability check that requires hearing.",
// Condition::Frightened => " • A frightened creature has disadvantage on ability checks and attack rolls while the source of its fear is within line of sight.\n • The creature can’t willingly move closer to the source of its fear.",
// Condition::Grappled => " • A grappled creature’s speed becomes 0, and it can’t benefit from any bonus to its speed.\n • The condition ends if the grappler is incapacitated (see the condition).\n • The condition also ends if an effect removes the grappled creature from the reach of the grappler or grappling effect, such as when a creature is hurled away by the thunderwave spell.",
// Condition::Incapacitated => " • An incapacitated creature can’t take actions or reactions.",
// Condition::Invisible => " • An invisible creature is impossible to see without the aid of magic or a special sense. For the purpose of hiding, the creature is heavily obscured. The creature’s location can be detected by any noise it makes or any tracks it leaves.\n • Attack rolls against the creature have disadvantage, and the creature’s attack rolls have advantage.",
// Condition::Paralyzed => " • A paralyzed creature is incapacitated and can’t move or speak.\n • The creature automatically fails Strength and Dexterity saving throws.\n • Attack rolls against the creature have advantage.\n • Any attack that hits the creature is a critical hit if the attacker is within 5 feet of the creature.",
// Condition::Petrified => " • A petrified creature is transformed, along with any nonmagical object it is wearing or carrying, into a solid inanimate substance (usually stone). Its weight increases by a factor of ten, and it ceases aging.\n • The creature is incapacitated (see the condition), can’t move or speak, and is unaware of its surroundings.\n • Attack rolls against the creature have advantage.\n • The creature automatically fails Strength and Dexterity saving throws.\n • The creature has resistance to all damage.\n • The creature is immune to poison and disease, although a poison or disease already in its system is suspended, not neutralized.",
// Condition::Poisoned => " • A poisoned creature has disadvantage on attack rolls and ability checks.",
// Condition::Prone => " • A prone creature’s only movement option is to crawl, unless it stands up and thereby ends the condition.\n • The creature has disadvantage on attack rolls.\n • An attack roll against the creature has advantage if the attacker is within 5 feet of the creature. Otherwise, the attack roll has disadvantage.",
// Condition::Restrained => " • A restrained creature’s speed becomes 0, and it can’t benefit from any bonus to its speed.\n • Attack rolls against the creature have advantage, and the creature's attack rolls have disadvantage.\n • The creature has disadvantage on Dexterity saving throws.",
// Condition::Stunned => " • A stunned creature is incapacitated (see the condition), can’t move, and can speak only falteringly.\n • The creature automatically fails Strength and Dexterity saving throws. • Attack rolls against the creature have advantage.",
// Condition::Surprised => " • A surprised creature can’t move or take an action on the first turn of the combat, and can't take a reaction until that turn ends. A member of a group can be surprised even if the other members aren’t.",
// Condition::Unconscious => " • An unconscious creature is incapacitated (see the condition), can’t move or speak, and is unaware of its surroundings\n • The creature drops whatever it’s holding and falls prone.\n • The creature automatically fails Strength and Dexterity saving throws.\n • Attack rolls against the creature have advantage.\n • Any attack that hits the creature is a critical hit if the attacker is within 5 feet of the creature.",
