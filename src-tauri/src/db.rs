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

pub use connection::CampaignDb;
pub use error::DbError;
