// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! Path resolution for application and campaign directories.

use std::fs;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use super::ConfigError;

/// Maximum length for campaign folder names (including disambiguation suffix).
const MAX_FOLDER_NAME_LEN: usize = 16;

/// Application path resolver using `ProjectDirs`.
///
/// Provides access to standard application directories:
/// - `data_local_dir`: Campaign folders and user data
/// - `preference_dir`: App-wide settings
#[derive(Debug, Clone)]
pub struct AppPaths {
    data_dir: PathBuf,
    preference_dir: PathBuf,
}

impl AppPaths {
    /// Creates a new `AppPaths` instance.
    ///
    /// Returns `ConfigError::NoAppDirs` if the system directories cannot be determined.
    pub fn new() -> Result<Self, ConfigError> {
        let dirs = ProjectDirs::from("com", "dungeon-minion", "Dungeon Minion")
            .ok_or(ConfigError::NoAppDirs)?;

        Ok(Self {
            data_dir: dirs.data_local_dir().to_path_buf(),
            preference_dir: dirs.preference_dir().to_path_buf(),
        })
    }

    /// Returns the data directory where campaign folders are stored.
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// Returns the preference directory for app-wide settings.
    pub fn preference_dir(&self) -> &Path {
        &self.preference_dir
    }

    /// Returns the path to the app-wide settings file.
    pub fn app_settings_path(&self) -> PathBuf {
        self.preference_dir.join("settings.toml")
    }

    /// Lists all campaign folders in the data directory.
    pub fn list_campaigns(&self) -> Result<Vec<PathBuf>, ConfigError> {
        if !self.data_dir.exists() {
            return Ok(Vec::new());
        }

        let mut campaigns = Vec::new();
        for entry in fs::read_dir(&self.data_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && path.join("settings.toml").exists() {
                campaigns.push(path);
            }
        }
        Ok(campaigns)
    }

    /// Resolves a campaign folder path, creating it if necessary.
    ///
    /// The folder name is derived from the campaign name:
    /// - Converted to kebab-case
    /// - Limited to 16 characters, including any numeric suffix
    /// - Numeric suffix added if name already exists
    ///
    /// Creates the folder structure with `saves/`, `portraits/`, and `maps/` subfolders.
    pub fn create_campaign_folder(&self, name: &str) -> Result<PathBuf, ConfigError> {
        let folder_name = Self::normalize_folder_name(name);
        let base_path = self.data_dir.join(&folder_name);

        // Find unique path with numeric suffix if needed
        let path = self.find_unique_path(&base_path);

        // Create folder structure
        fs::create_dir_all(&path)?;
        fs::create_dir_all(path.join("saves"))?;
        fs::create_dir_all(path.join("portraits"))?;
        fs::create_dir_all(path.join("maps"))?;

        Ok(path)
    }

    /// Returns the campaign folder path for a given folder name.
    ///
    /// Does not create the folder; returns error if it doesn't exist.
    pub fn campaign_path(&self, folder_name: &str) -> Result<PathBuf, ConfigError> {
        let path = self.data_dir.join(folder_name);
        if !path.exists() || !path.join("settings.toml").exists() {
            return Err(ConfigError::CampaignNotFound(path));
        }

        Ok(path)
    }

    /// Normalizes a campaign name to a valid folder name.
    ///
    /// - Converts to lowercase
    /// - Replaces spaces and underscores with hyphens
    /// - Removes non-alphanumeric characters (except hyphens)
    /// - Collapses multiple hyphens
    /// - Truncates to 16 characters
    fn normalize_folder_name(name: &str) -> String {
        if name.trim().is_empty() {
            return "campaign".to_string();
        }

        let normalized = name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .flat_map(|c| c.to_lowercase());

        // Collapse multiple hyphens and trim leading/trailing hyphens
        let mut result = String::with_capacity(name.len());
        let mut prev_hyphen = true; // Start true to skip leading hyphens
        for c in normalized {
            if c == '-' {
                if !prev_hyphen {
                    result.push(c);
                    prev_hyphen = true;
                }
            } else {
                result.push(c);
                prev_hyphen = false;
            }
        }

        // Remove trailing hyphen
        if result.ends_with('-') {
            result.pop();
        }

        if result.is_empty() {
            return "campaign".to_string();
        }

        // Truncate to max length
        if result.len() > MAX_FOLDER_NAME_LEN {
            result.truncate(MAX_FOLDER_NAME_LEN);
            // Don't end with a hyphen after truncation
            while result.ends_with('-') {
                result.pop();
            }
        }

        result
    }

    /// Finds a unique path by appending numeric suffixes if needed.
    fn find_unique_path(&self, base_path: &Path) -> PathBuf {
        if !base_path.exists() {
            return base_path.to_path_buf();
        }

        let base_name = base_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("campaign");

        for i in 2.. {
            let suffix = format!("-{}", i);
            let max_base_len = MAX_FOLDER_NAME_LEN.saturating_sub(suffix.len());
            let mut trimmed_base = base_name.to_string();
            trimmed_base.truncate(max_base_len);

            while trimmed_base.ends_with('-') {
                trimmed_base.pop();
            }

            if trimmed_base.is_empty() {
                trimmed_base.push_str("campaign");
            }

            let new_name = format!("{}{}", trimmed_base, suffix);
            let new_path = base_path.with_file_name(new_name);
            if !new_path.exists() {
                return new_path;
            }
        }

        // Fallback (should never reach here)
        base_path.to_path_buf()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_folder_name() {
        assert_eq!(
            AppPaths::normalize_folder_name("Dragon Heist"),
            "dragon-heist"
        );
        assert_eq!(
            AppPaths::normalize_folder_name("Orcs 'n' Goblins!"),
            "orcs-n-goblins"
        );
        assert_eq!(
            AppPaths::normalize_folder_name("  Spaced  Out  "),
            "spaced-out"
        );
        assert_eq!(
            AppPaths::normalize_folder_name("A Very Long Campaign Name Indeed"),
            "a-very-long-camp"
        );
        assert_eq!(
            AppPaths::normalize_folder_name("Revenge---Of---The---Hyphens"),
            "revenge-of-the-h"
        );
        assert_eq!(
            AppPaths::normalize_folder_name("Quest to /dev/null"),
            "quest-to-dev-nul"
        );
    }

    #[test]
    fn test_normalize_folder_name_invalid() {
        assert_eq!(AppPaths::normalize_folder_name(""), "campaign");
        assert_eq!(AppPaths::normalize_folder_name("   "), "campaign");
        assert_eq!(AppPaths::normalize_folder_name("---"), "campaign");
    }
}
