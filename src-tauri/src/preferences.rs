// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use std::fs;

use serde::{Deserialize, Serialize};
use tauri::{async_runtime::Mutex, AppHandle, Emitter, Manager};

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppPreferences {
    ui: UiPreferences,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiPreferences {
    theme: Theme,
    theme_mode: ThemeMode,
    font_size: FontSize,
    font_style: FontStyle,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GamePreferences {
    monster_hp: MonsterHitPoints,
    rules_version: RulesVersion,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    Arcane,
    Dungeoneer,
    HighContrast,
    Intrepid,
    Mystic,
    Woodland,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThemeMode {
    Dawn,
    Dusk,
    Noon,
    Night,
    Light,
    Dark,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FontSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FontStyle {
    Sans,
    Serif,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MonsterHitPoints {
    Fixed,
    Rolled,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RulesVersion {
    Srd51,
    Srd523,
}

pub type AppPreferencesMutex = Mutex<AppPreferences>;

impl AppPreferences {
    pub fn load(app: &AppHandle) -> Self {
        let path = app
            .path()
            .app_config_dir()
            .unwrap()
            .join("preferences.toml");

        if !path.exists() {
            return Self::default();
        }

        let Ok(toml) = fs::read_to_string(path) else {
            return Self::default();
        };

        let Ok(prefs) = toml::from_str::<Self>(&toml) else {
            return Self::default();
        };

        prefs
    }

    pub fn save(&self, app: &AppHandle) {
        let path = app
            .path()
            .app_config_dir()
            .unwrap()
            .join("preferences.toml");

        if let Ok(toml) = toml::to_string_pretty(self) {
            let _ = fs::create_dir_all(path.parent().unwrap());
            let _ = fs::write(path, toml);
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dungeoneer
    }
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Dusk
    }
}

impl Default for FontSize {
    fn default() -> Self {
        FontSize::Medium
    }
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle::Serif
    }
}

impl Default for MonsterHitPoints {
    fn default() -> Self {
        MonsterHitPoints::Rolled
    }
}

impl Default for RulesVersion {
    fn default() -> Self {
        RulesVersion::Srd523
    }
}
