use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

use super::Duration;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Condition {
    Blinded {
        duration: Duration,
    },
    Bloodied,
    Charmed {
        duration: Duration,
    },
    Dead,
    Deafened {
        duration: Duration,
    },
    Frightened {
        duration: Duration,
    },
    Grappled {
        source: String,
    },
    Incapacitated {
        duration: Duration,
    },
    Invisible {
        duration: Duration,
    },
    Marked {
        source: String,
        duration: Duration,
    },
    Paralyzed {
        duration: Duration,
    },
    Petrified {
        duration: Duration,
    },
    Poisoned {
        duration: Duration,
    },
    Prone,
    Restrained {
        duration: Duration,
    },
    Spellcasting {
        spell: String,
        concentration: bool,
        duration: Duration,
    },
    Stunned {
        duration: Duration,
    },
    Surprised,
    Unconscious,
}

impl Condition {
    pub fn begin_turn(self) -> Option<Self> {
        match self.duration() {
            Some(Duration::StartOfTurn) => None,
            Some(Duration::Seconds(seconds)) => {
                if seconds > 0 {
                    Some(self.with_duration(Duration::Seconds(seconds - 6)))
                } else {
                    None
                }
            }
            _ => Some(self),
        }
    }

    pub fn end_turn(self) -> Option<Self> {
        if matches!(self, Condition::Surprised) {
            return None;
        }

        match self.duration() {
            Some(Duration::EndOfTurn) => None,
            _ => Some(self),
        }
    }

    pub fn duration(&self) -> Option<Duration> {
        match self {
            Condition::Blinded { duration } => Some(duration.clone()),
            Condition::Charmed { duration } => Some(duration.clone()),
            Condition::Deafened { duration } => Some(duration.clone()),
            Condition::Frightened { duration } => Some(duration.clone()),
            Condition::Incapacitated { duration } => Some(duration.clone()),
            Condition::Invisible { duration } => Some(duration.clone()),
            Condition::Marked { duration, .. } => Some(duration.clone()),
            Condition::Paralyzed { duration } => Some(duration.clone()),
            Condition::Petrified { duration } => Some(duration.clone()),
            Condition::Poisoned { duration } => Some(duration.clone()),
            Condition::Restrained { duration } => Some(duration.clone()),
            Condition::Spellcasting { duration, .. } => Some(duration.clone()),
            Condition::Stunned { duration } => Some(duration.clone()),
            _ => None,
        }
    }

    pub fn with_duration(self, duration: Duration) -> Self {
        match self {
            Condition::Blinded { .. } => Condition::Blinded { duration },
            Condition::Charmed { .. } => Condition::Charmed { duration },
            Condition::Deafened { .. } => Condition::Deafened { duration },
            Condition::Frightened { .. } => Condition::Frightened { duration },
            Condition::Incapacitated { .. } => Condition::Incapacitated { duration },
            Condition::Invisible { .. } => Condition::Invisible { duration },
            Condition::Marked { source, .. } => Condition::Marked { source, duration },
            Condition::Paralyzed { .. } => Condition::Paralyzed { duration },
            Condition::Petrified { .. } => Condition::Petrified { duration },
            Condition::Poisoned { .. } => Condition::Poisoned { duration },
            Condition::Restrained { .. } => Condition::Restrained { duration },
            Condition::Spellcasting {
                spell,
                concentration,
                ..
            } => Condition::Spellcasting {
                spell,
                concentration,
                duration,
            },
            Condition::Stunned { .. } => Condition::Stunned { duration },
            _ => self,
        }
    }

    pub fn source(&self) -> Option<String> {
        match self {
            Condition::Grappled { source } => Some(source.clone()),
            Condition::Marked { source, .. } => Some(source.clone()),
            _ => None,
        }
    }

    pub fn implications(&self) -> Vec<Condition> {
        match self {
            Condition::Grappled { .. } => vec![Condition::Restrained {
                duration: Duration::Indefinite,
            }],
            Condition::Paralyzed { duration } => vec![Condition::Incapacitated {
                duration: duration.clone(),
            }],
            Condition::Petrified { duration } => vec![
                Condition::Incapacitated {
                    duration: duration.clone(),
                },
                Condition::Unconscious,
            ],
            Condition::Stunned { duration } => vec![Condition::Incapacitated {
                duration: duration.clone(),
            }],
            Condition::Unconscious => vec![
                Condition::Incapacitated {
                    duration: Duration::Indefinite,
                },
                Condition::Prone,
            ],
            _ => vec![],
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Condition::Blinded { .. } => write!(f, "Blinded"),
            Condition::Bloodied => write!(f, "Bloodied"),
            Condition::Charmed { .. } => write!(f, "Charmed"),
            Condition::Dead => write!(f, "Dead"),
            Condition::Deafened { .. } => write!(f, "Deafened"),
            Condition::Frightened { .. } => write!(f, "Frightened"),
            Condition::Grappled { .. } => write!(f, "Grappled"),
            Condition::Incapacitated { .. } => write!(f, "Incapacitated"),
            Condition::Invisible { .. } => write!(f, "Invisible"),
            Condition::Marked { .. } => write!(f, "Marked"),
            Condition::Paralyzed { .. } => write!(f, "Paralyzed"),
            Condition::Petrified { .. } => write!(f, "Petrified"),
            Condition::Poisoned { .. } => write!(f, "Poisoned"),
            Condition::Prone => write!(f, "Prone"),
            Condition::Restrained { .. } => write!(f, "Restrained"),
            Condition::Spellcasting {
                spell,
                concentration: true,
                ..
            } => write!(f, "Concentrating on {spell}"),
            Condition::Spellcasting { spell, .. } => write!(f, "Casting {spell}"),
            Condition::Stunned { .. } => write!(f, "Stunned"),
            Condition::Surprised => write!(f, "Surprised"),
            Condition::Unconscious => write!(f, "Unconscious"),
        }
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
