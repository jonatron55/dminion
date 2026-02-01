// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! App-wide settings stored in `settings.toml` in the preference directory.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::ConfigError;

/// App-wide settings that are not campaign-specific.
///
/// Stored in `<preference_dir>/settings.toml`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppSettings {
    /// Recently opened campaign paths, most recent first.
    #[serde(default)]
    pub recent_campaigns: Vec<PathBuf>,

    /// Main window state.
    #[serde(default)]
    pub window: WindowSettings,

    /// Player-facing window state.
    #[serde(default)]
    pub player_window: WindowSettings,

    /// Savepoint configuration.
    #[serde(default)]
    pub savepoints: SavepointSettings,
}

/// Window position and size state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WindowSettings {
    /// X position of the window.
    pub x: Option<i32>,

    /// Y position of the window.
    pub y: Option<i32>,

    /// Width of the window.
    pub width: Option<u32>,

    /// Height of the window.
    pub height: Option<u32>,

    /// Whether the window is maximized.
    #[serde(default)]
    pub maximized: bool,
}

/// Savepoint configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SavepointSettings {
    /// When to create savepoints.
    #[serde(default)]
    pub trigger: SavepointTrigger,

    /// Maximum number of savepoints to keep. Older ones are pruned.
    #[serde(default = "default_max_savepoints")]
    pub max_count: u32,
}

/// When to automatically create encounter savepoints.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavepointTrigger {
    /// Create a savepoint after each turn.
    #[default]
    Turn,

    /// Create a savepoint after each round.
    Round,
}

fn default_max_savepoints() -> u32 {
    50
}

impl AppSettings {
    /// Loads app settings from the preference directory.
    ///
    /// Returns default settings if the file doesn't exist or can't be parsed.
    pub fn load(preference_dir: &Path) -> Self {
        let path = preference_dir.join("settings.toml");

        if !path.exists() {
            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Saves app settings to the preference directory.
    pub fn save(&self, preference_dir: &Path) -> Result<(), ConfigError> {
        fs::create_dir_all(preference_dir)?;
        let path = preference_dir.join("settings.toml");
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    /// Adds a campaign to the recent list, moving it to the front if already present.
    pub fn add_recent_campaign(&mut self, path: PathBuf) {
        // Remove if already in list
        self.recent_campaigns.retain(|p| p != &path);

        // Add to front
        self.recent_campaigns.insert(0, path);

        // Limit list size
        const MAX_RECENT: usize = 10;
        self.recent_campaigns.truncate(MAX_RECENT);
    }

    /// Removes a campaign from the recent list.
    pub fn remove_recent_campaign(&mut self, path: &Path) {
        self.recent_campaigns.retain(|p| p != path);
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            width: Some(1280),
            height: Some(720),
            maximized: false,
        }
    }
}

impl Default for SavepointSettings {
    fn default() -> Self {
        Self {
            trigger: SavepointTrigger::default(),
            max_count: default_max_savepoints(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_settings_roundtrip() {
        let mut settings = AppSettings::default();
        settings.add_recent_campaign(PathBuf::from("/test/campaign"));
        settings.window.maximized = true;
        settings.savepoints.trigger = SavepointTrigger::Round;

        let toml = toml::to_string_pretty(&settings).unwrap();
        let parsed: AppSettings = toml::from_str(&toml).unwrap();

        assert_eq!(parsed.recent_campaigns.len(), 1);
        assert!(parsed.window.maximized);
        assert_eq!(parsed.savepoints.trigger, SavepointTrigger::Round);
    }

    #[test]
    fn test_recent_campaigns_ordering() {
        let mut settings = AppSettings::default();
        settings.add_recent_campaign(PathBuf::from("/a"));
        settings.add_recent_campaign(PathBuf::from("/b"));
        settings.add_recent_campaign(PathBuf::from("/a")); // Move to front

        assert_eq!(settings.recent_campaigns[0], PathBuf::from("/a"));
        assert_eq!(settings.recent_campaigns[1], PathBuf::from("/b"));
        assert_eq!(settings.recent_campaigns.len(), 2);
    }
}
