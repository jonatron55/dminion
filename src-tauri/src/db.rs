// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! Database access layer for campaign data.
//!
//! This module provides:
//! - Connection management for per-campaign SQLite databases
//! - Query functions for monsters, players, encounters, etc.
//! - Schema migration support

mod connection;
mod error;
mod monsters;
mod parties;
mod players;

pub use connection::CampaignDb;
pub use error::DbError;
pub use monsters::{MonsterData, MonsterRecord};
pub use parties::{PartyData, PartyRecord};
pub use players::{PlayerClassData, PlayerClassRecord, PlayerData, PlayerRecord};
