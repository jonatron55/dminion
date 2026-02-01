// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! Per-campaign settings stored in `settings.toml` within each campaign folder.

use std::fs;
use std::path::Path;

use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};

use super::ConfigError;

/// Current schema version for new campaigns.
pub const CURRENT_SCHEMA_VERSION: &str = "1.0.0";

/// Per-campaign settings including metadata, rules configuration, and UI preferences.
///
/// Stored in `<campaign>/settings.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CampaignSettings {
    /// Display name of the campaign.
    pub name: String,

    /// When the campaign was created.
    pub created: DateTime<Utc>,

    /// Schema version for the campaign folder and database.
    /// Uses semver for migration compatibility checks.
    pub schema_version: Version,

    /// Rules configuration for the campaign.
    #[serde(default)]
    pub rules: RulesSettings,

    /// UI preferences for this campaign.
    #[serde(default)]
    pub ui: UiSettings,
}

/// Rules configuration for a campaign.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RulesSettings {
    /// SRD rules version to use.
    #[serde(default)]
    pub version: RulesVersion,

    /// How monster hit points are determined.
    #[serde(default)]
    pub monster_hp: MonsterHitPoints,
}

/// UI preferences that can vary per campaign for different "feels".
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UiSettings {
    /// Color theme for the UI.
    #[serde(default)]
    pub theme: Theme,

    /// Theme mode (light/dark variant).
    #[serde(default)]
    pub theme_mode: ThemeMode,

    /// Font size preference.
    #[serde(default)]
    pub font_size: FontSize,
}

/// SRD rules version.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RulesVersion {
    /// SRD 5.1 (2014 rules).
    #[serde(rename = "5.1")]
    Srd51,

    /// SRD 5.2 (2024 rules).
    #[default]
    #[serde(rename = "5.2")]
    Srd52,
}

/// How monster hit points are determined when spawning.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MonsterHitPoints {
    /// Use the fixed average value.
    Fixed,

    /// Roll hit dice for each monster.
    #[default]
    Rolled,
}

/// Available color themes.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    Arcane,
    #[default]
    Dungeoneer,
    HighContrast,
    Intrepid,
    Mystic,
    Woodland,
}

/// Theme mode (light/dark variants).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThemeMode {
    Dawn,
    #[default]
    Dusk,
    Noon,
    Night,
    Light,
    Dark,
}

/// Font size preference.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FontSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl CampaignSettings {
    /// Creates new campaign settings with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            created: Utc::now(),
            schema_version: Version::parse(CURRENT_SCHEMA_VERSION).unwrap(),
            rules: RulesSettings::default(),
            ui: UiSettings::default(),
        }
    }

    /// Loads campaign settings from a campaign folder.
    pub fn load(campaign_path: &Path) -> Result<Self, ConfigError> {
        let path = campaign_path.join("settings.toml");
        let content = fs::read_to_string(&path)?;
        let settings = toml::from_str(&content)?;
        Ok(settings)
    }

    /// Saves campaign settings to the campaign folder.
    pub fn save(&self, campaign_path: &Path) -> Result<(), ConfigError> {
        let path = campaign_path.join("settings.toml");
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    /// Returns the path to the database file within this campaign.
    pub fn db_path(campaign_path: &Path) -> std::path::PathBuf {
        campaign_path.join("db.sqlite3")
    }

    /// Returns the path to the saves folder within this campaign.
    pub fn saves_path(campaign_path: &Path) -> std::path::PathBuf {
        campaign_path.join("saves")
    }

    /// Returns the path to the portraits folder within this campaign.
    pub fn portraits_path(campaign_path: &Path) -> std::path::PathBuf {
        campaign_path.join("portraits")
    }

    /// Returns the path to the maps folder within this campaign.
    pub fn maps_path(campaign_path: &Path) -> std::path::PathBuf {
        campaign_path.join("maps")
    }
}

impl Default for RulesSettings {
    fn default() -> Self {
        Self {
            version: RulesVersion::default(),
            monster_hp: MonsterHitPoints::default(),
        }
    }
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            theme_mode: ThemeMode::default(),
            font_size: FontSize::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_campaign_settings_roundtrip() {
        let settings = CampaignSettings::new("Test Campaign");
        let toml = toml::to_string_pretty(&settings).unwrap();
        let parsed: CampaignSettings = toml::from_str(&toml).unwrap();

        assert_eq!(parsed.name, "Test Campaign");
        assert_eq!(parsed.schema_version, settings.schema_version);
    }

    #[test]
    fn test_rules_version_serialization() {
        let rules = RulesSettings {
            version: RulesVersion::Srd51,
            monster_hp: MonsterHitPoints::Fixed,
        };
        let toml = toml::to_string(&rules).unwrap();
        assert!(toml.contains("version = \"5.1\""));
        assert!(toml.contains("monster_hp = \"fixed\""));
    }
}
