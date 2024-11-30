use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::dice::roll;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub str: u32,
    pub dex: u32,
    pub con: u32,
    pub int: u32,
    pub wis: u32,
    pub cha: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub level: u32,
}

impl Stats {
    pub fn roll<TRng: Rng>(rng: &mut TRng) -> Self {
        Stats {
            str: roll("4d6kh3", rng).unwrap() as u32,
            dex: roll("4d6kh3", rng).unwrap() as u32,
            con: roll("4d6kh3", rng).unwrap() as u32,
            int: roll("4d6kh3", rng).unwrap() as u32,
            wis: roll("4d6kh3", rng).unwrap() as u32,
            cha: roll("4d6kh3", rng).unwrap() as u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Difficulties {
    pub easy: u32,
    pub medium: u32,
    pub hard: u32,
    pub deadly: u32,
}

const XP_PER_CR: &'static [u32; 34] = &[
    10, 25, 50, 100, 200, 450, 700, 1100, 1800, 2300, 2900, 3900, 5000, 5900, 7200, 8400, 10000,
    11500, 13000, 15000, 18000, 20000, 22000, 25000, 33000, 41000, 50000, 62000, 75000, 90000,
    105000, 120000, 135000, 155000,
];

const DIFFICULTIES_PER_LEVEL: &'static [Difficulties; 20] = &[
    Difficulties {
        easy: 25,
        medium: 50,
        hard: 75,
        deadly: 100,
    },
    Difficulties {
        easy: 50,
        medium: 100,
        hard: 150,
        deadly: 200,
    },
    Difficulties {
        easy: 75,
        medium: 150,
        hard: 225,
        deadly: 400,
    },
    Difficulties {
        easy: 125,
        medium: 250,
        hard: 375,
        deadly: 500,
    },
    Difficulties {
        easy: 250,
        medium: 500,
        hard: 750,
        deadly: 1100,
    },
    Difficulties {
        easy: 300,
        medium: 600,
        hard: 900,
        deadly: 1400,
    },
    Difficulties {
        easy: 350,
        medium: 750,
        hard: 1100,
        deadly: 1700,
    },
    Difficulties {
        easy: 450,
        medium: 900,
        hard: 1400,
        deadly: 2100,
    },
    Difficulties {
        easy: 550,
        medium: 1100,
        hard: 1600,
        deadly: 2400,
    },
    Difficulties {
        easy: 600,
        medium: 1200,
        hard: 1900,
        deadly: 2800,
    },
    Difficulties {
        easy: 800,
        medium: 1600,
        hard: 2400,
        deadly: 3600,
    },
    Difficulties {
        easy: 1000,
        medium: 2000,
        hard: 3000,
        deadly: 4500,
    },
    Difficulties {
        easy: 1100,
        medium: 2200,
        hard: 3400,
        deadly: 5100,
    },
    Difficulties {
        easy: 1250,
        medium: 2500,
        hard: 3800,
        deadly: 5700,
    },
    Difficulties {
        easy: 1400,
        medium: 2800,
        hard: 4300,
        deadly: 6400,
    },
    Difficulties {
        easy: 1600,
        medium: 3200,
        hard: 4800,
        deadly: 7200,
    },
    Difficulties {
        easy: 2000,
        medium: 3900,
        hard: 5900,
        deadly: 8800,
    },
    Difficulties {
        easy: 2100,
        medium: 4200,
        hard: 6300,
        deadly: 9500,
    },
    Difficulties {
        easy: 2400,
        medium: 4900,
        hard: 7300,
        deadly: 10900,
    },
    Difficulties {
        easy: 2800,
        medium: 5700,
        hard: 8500,
        deadly: 12700,
    },
];

pub fn modifer(score: u32) -> i32 {
    if score >= 10 {
        ((score as i32) - 10) / 2
    } else {
        ((score as i32) - 11) / 2
    }
}

pub fn xp(cr: u32) -> u32 {
    XP_PER_CR[(cr as usize).max(XP_PER_CR.len() - 1)] as u32
}
pub fn cr_value(cr: u32) -> f32 {
    match cr {
        0 => 0.0,
        1 => 0.125,
        2 => 0.25,
        3 => 0.5,
        n => (n as f32) - 3.0,
    }
}

pub fn cr_string(cr: u32) -> String {
    match cr {
        0 => "0".to_string(),
        1 => "⅛".to_string(),
        2 => "¼".to_string(),
        3 => "½".to_string(),
        n => (n - 3).to_string(),
    }
}

pub fn adjusted_xp(crs: &[u32]) -> u32 {
    let total: u32 = crs.iter().map(|cr| XP_PER_CR[*cr as usize]).sum();

    ((total as f32) * match crs.len() {
        1 => 1.0,
        2 => 1.5,
        3 | 4 | 5 | 6 => 2.0,
        7 | 8 | 9 | 10 => 2.5,
        11 | 12 | 13 | 14 => 3.0,
        _ => 4.0
    }).ceil() as u32
}
