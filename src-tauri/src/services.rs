// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! Services for campaign functionality.
//!
//! This module provides:
//! - Portrait resolution with prioritized search
//! - Encounter savepoint management

mod portrait;
mod savepoint;

pub use portrait::{PortraitService, PortraitSize, PortraitSubject};
pub use savepoint::SavepointService;
