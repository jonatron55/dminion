// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! Portrait resolution service.
//!
//! Resolves portrait base names to actual file paths using prioritized search:
//! 1. Campaign `portraits/` folder with requested size
//! 2. Campaign `portraits/` folder with alternate size
//! 3. App installation `portraits/` folder with same search order
//! 4. Built-in placeholder image if no match found

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Portrait image size variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PortraitSize {
    /// Small thumbnail for lists and compact views.
    Small,

    /// Full-size portrait for detailed views.
    Full,
}

/// Portrait subject type for placeholder selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PortraitSubject {
    /// A monster portrait.
    Monster,

    /// A player character portrait.
    Player,
}

impl PortraitSize {
    /// Returns the file suffix for this size (e.g., ".small").
    fn suffix(self) -> &'static str {
        match self {
            Self::Small => ".small",
            Self::Full => ".full",
        }
    }

    /// Returns the alternate size for fallback.
    fn alternate(self) -> Self {
        match self {
            Self::Small => Self::Full,
            Self::Full => Self::Small,
        }
    }
}

/// Service for resolving portrait base names to file paths.
pub struct PortraitService {
    campaign_portraits: Option<PathBuf>,
    app_portraits: PathBuf,
    static_path: PathBuf,
}

/// Supported image extensions in priority order.
const EXTENSIONS: &[&str] = &["webp", "png", "jpg", "jpeg"];

impl PortraitService {
    /// Creates a new portrait service.
    ///
    /// - `campaign_path`: Optional path to the current campaign folder.
    /// - `app_data_path`: Path to the app's data directory for shared portraits.
    /// - `static_path`: Path to the app's static assets (for placeholders).
    pub fn new(campaign_path: Option<&Path>, app_data_path: &Path, static_path: &Path) -> Self {
        Self {
            campaign_portraits: campaign_path.map(|p| p.join("portraits")),
            app_portraits: app_data_path.join("portraits"),
            static_path: static_path.to_path_buf(),
        }
    }

    /// Resolves a portrait base name to a file path.
    ///
    /// Returns the path to the best matching portrait file, or the appropriate
    /// placeholder if no match is found. The base name should not include the
    /// size suffix or extension (e.g., "goblin" not "goblin.small.jpg").
    pub fn resolve(
        &self,
        base_name: &str,
        size: PortraitSize,
        subject: PortraitSubject,
    ) -> PathBuf {
        // Try campaign folder first
        if let Some(campaign_dir) = &self.campaign_portraits {
            if let Some(path) = self.find_in_dir(campaign_dir, base_name, size) {
                return path;
            }
            // Try alternate size in campaign folder
            if let Some(path) = self.find_in_dir(campaign_dir, base_name, size.alternate()) {
                return path;
            }
        }

        // Try app portraits folder
        if let Some(path) = self.find_in_dir(&self.app_portraits, base_name, size) {
            return path;
        }
        // Try alternate size in app folder
        if let Some(path) = self.find_in_dir(&self.app_portraits, base_name, size.alternate()) {
            return path;
        }

        // Return placeholder for the subject type
        self.placeholder(size, subject)
    }

    /// Searches for a portrait file in the given directory.
    ///
    /// Looks for files matching `{base_name}{size_suffix}.{ext}` for each
    /// supported extension.
    fn find_in_dir(&self, dir: &Path, base_name: &str, size: PortraitSize) -> Option<PathBuf> {
        let suffix = size.suffix();

        for ext in EXTENSIONS {
            let filename = format!("{}{}.{}", base_name, suffix, ext);
            let path = dir.join(&filename);
            if path.is_file() {
                return Some(path);
            }
        }

        None
    }

    /// Returns the placeholder image path.
    pub fn placeholder(&self, size: PortraitSize, subject: PortraitSubject) -> PathBuf {
        let subject_name = match subject {
            PortraitSubject::Monster => "unknown-monster",
            PortraitSubject::Player => "unknown-player",
        };
        let filename = format!("{}{}.jpg", subject_name, size.suffix());
        self.static_path.join("images/portraits").join(filename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portrait_size_suffix() {
        assert_eq!(PortraitSize::Small.suffix(), ".small");
        assert_eq!(PortraitSize::Full.suffix(), ".full");
    }

    #[test]
    fn test_portrait_size_alternate() {
        assert_eq!(PortraitSize::Small.alternate(), PortraitSize::Full);
        assert_eq!(PortraitSize::Full.alternate(), PortraitSize::Small);
    }
}
