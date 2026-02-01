// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! Configuration-related error types.

use std::path::PathBuf;

/// Errors that can occur during configuration operations.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Failed to read a configuration file.
    #[error("failed to read settings: {0}")]
    Read(#[from] std::io::Error),

    /// Failed to parse a TOML configuration file.
    #[error("invalid settings format: {0}")]
    Parse(#[from] toml::de::Error),

    /// Failed to serialize settings to TOML.
    #[error("failed to serialize settings: {0}")]
    Serialize(#[from] toml::ser::Error),

    /// Campaign folder already exists with the given name.
    #[error("campaign folder already exists: {0}")]
    FolderExists(PathBuf),

    /// Could not determine application directories.
    #[error("could not determine application directories")]
    NoAppDirs,

    /// Campaign folder not found.
    #[error("campaign not found: {0}")]
    CampaignNotFound(PathBuf),

    /// Invalid campaign name (empty or contains invalid characters).
    #[error("invalid campaign name: {0}")]
    InvalidCampaignName(String),
}
