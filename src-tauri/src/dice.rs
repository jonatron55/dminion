// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! This module parses and evaluates dice expressions using typical notation
//! such as `3d8 + 2`. The normal arithmetic  operations `+`, `-`, `*`, and `/`
//! are supported with `×` and `÷` recognized as alternate forms of `*` and `/`.
//! Products precede sums unless grouped be parentheses. Dice rolls are
//! expressed as *count*`d`*sides* where *sides* is 4, 6, 8, 10, 12, 20, or 100
//! (the sequence `d%` is interpreted as `d100`). If the die count is omitted
//! (e.g. `d20 + 5`), it is assumed to be 1 and if the number of sides are
//! omitted (e.g. `4d + 1`), then the dice are assumed to be six-sided. A roll
//! may be followed by any number of selection modifiers, too keep or discard
//! certain dice:
//!
//! - `k<n>` or `kh<n>`: keep the highest `<n>` dice. If `<n>` is omitted, it is
//!   assumed to be 1.
//! - `kl<n>`: keep the lowest `<n>` dice. If `<n>` is omitted, it is assumed to
//!   be 1.
//! - `d<n>` or `dl<n>`: discard the lowest `<n>` dice. If `<n>` is omitted, it
//!   is assumed to be 1.
//! - `dh<n>`: discard the highest `<n>` dice. If `<n>` is omitted, it is
//!   assumed to be 1.
//! - `adv` or `ad`: reroll the preceding expression and take the higher result.
//! - `dis` or `da`: reroll the preceding expression and take the lower result.
//!
//! For example, to roll 4d6 and keep the highest 3 (common in D&D character
//! generation), you could write `4d6kh3` or equivalently `4d6d1` (roll 4d6 and
//! discard the lowest 1). Though uncommon, it is possible to chain several
//! selections and they will be evaluated in order from left to right. `adv` and
//! `dis` refer to "advantage" and "disadvantage" respectively, common in D&D 5e
//! and they have the effect of rerolling the entire previous sub-expression
//! (including any previous selections) and taking the higher or lower total
//! respectively.
//!
//! Only integers are supported, and the result of an expression is always an
//! integer. When division is performed, the result is rounded down to the
//! nearest integer before the next operation is performed.
//!
//! Grammar
//! -------
//!
//! Dice expressions are parsed according to the following grammar:
//!
//! ```ebnf
//! root = sum;
//! sum = term, { ("+" | "-"), term };
//! term = factor, { ("*" | "/"), factor };
//! factor = "(", sum, ")" | negation | integer | roll;
//! negation = "-", factor;
//! roll = [integer], "d", [integer], [selection];
//! selection = (
//!         "k", integer |
//!         "kh", integer |
//!         "kl", integer |
//!         "d", integer |
//!         "dh", integer |
//!         "dl", integer |
//!         "adv" | "ad" |
//!         "dis" | "da"
//!     ), [selection];
//! integer = /[0-9]+/;
//! ```

use std::{
    fmt::Debug,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};

use eval::DieRoll;
use rand::{rngs::StdRng, Rng};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ts_rs::TS;

pub mod ast;
pub mod eval;
pub mod lexer;
pub mod lookahead;
pub mod parser;
pub mod pp;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parse error {0}")]
    ParseError(#[from] parser::Error),
    #[error("Evaluation error {0}")]
    EvalError(#[from] eval::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Roll {
    pub value: i32,
    pub dice: Vec<DieRoll>,
}

#[derive(Clone)]
pub struct DiceExpr {
    root: Box<ast::Node>,
}

impl DiceExpr {
    pub fn parse(input: &str) -> Result<Self> {
        let root = parser::parse(input)?;
        Ok(DiceExpr { root })
    }

    pub fn roll<TRng: Rng>(&self, rng: &mut TRng) -> Result<Roll> {
        let mut evaluator = eval::Evaluator::new(eval::Evaluation::Rand(rng));
        let value = evaluator.eval(self.root.as_ref())?;
        Ok(Roll {
            value,
            dice: evaluator.rolls.clone(),
        })
    }

    pub fn mid(&self) -> Result<i32> {
        let mut evaluator = eval::Evaluator::new(eval::Evaluation::<StdRng>::Mid);
        Ok(evaluator.eval(self.root.as_ref())?)
    }

    pub fn min(&self) -> Result<i32> {
        let mut evaluator = eval::Evaluator::new(eval::Evaluation::<StdRng>::Min);
        Ok(evaluator.eval(self.root.as_ref())?)
    }

    pub fn max(&self) -> Result<i32> {
        let mut evaluator = eval::Evaluator::new(eval::Evaluation::<StdRng>::Max);
        Ok(evaluator.eval(self.root.as_ref())?)
    }
}

impl Display for DiceExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut pp = pp::PP::new(f);
        pp.write(self.root.as_ref())
    }
}

impl Debug for DiceExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut pp = pp::PP::new(f);
        pp.write(self.root.as_ref())
    }
}

impl Serialize for DiceExpr {
    fn serialize<S: Serializer>(&self, serializer: S) -> StdResult<S::Ok, S::Error> {
        serializer.serialize_str(format!("{self}").as_str())
    }
}

impl<'de> Deserialize<'de> for DiceExpr {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> StdResult<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        DiceExpr::parse(&s).map_err(serde::de::Error::custom)
    }
}

pub fn roll<TRng: Rng>(input: &str, rng: &mut TRng) -> Result<i32> {
    let root = parser::parse(&input)?;
    let mut evaluator = eval::Evaluator::new(eval::Evaluation::Rand(rng));
    Ok(evaluator.eval(root.as_ref())?)
}
