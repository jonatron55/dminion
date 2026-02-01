// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! Configuration management for campaigns and application settings.
//!
//! This module handles:
//! - Path resolution via `ProjectDirs`
//! - Per-campaign settings (`settings.toml` in campaign folder)
//! - App-wide settings (`settings.toml` in preference directory)
//! - Campaign folder creation and management

mod app_settings;
mod campaign_settings;
mod error;
mod paths;

pub use app_settings::{AppSettings, SavepointSettings, SavepointTrigger, WindowSettings};
pub use campaign_settings::{
    CampaignSettings, FontSize, MonsterHitPoints, RulesSettings, RulesVersion, Theme, ThemeMode,
    UiSettings, CURRENT_SCHEMA_VERSION,
};
pub use error::ConfigError;
pub use paths::AppPaths;
