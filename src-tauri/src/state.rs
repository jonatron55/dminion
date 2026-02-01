// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use std::path::PathBuf;

use tauri::{async_runtime::Mutex, AppHandle, Emitter};

use crate::config::{AppPaths, AppSettings, CampaignSettings};
use crate::db::CampaignDb;
use crate::game::Game;

/// Application state managed by Tauri.
pub struct AppState {
    /// Path resolver for app directories.
    pub paths: AppPaths,

    /// App-wide settings (window state, recent campaigns, etc.).
    pub app_settings: AppSettings,

    /// Currently open campaign, if any.
    pub campaign: Option<Campaign>,

    /// In-memory encounter state with undo/redo.
    pub encounter: EncounterState,
}

pub type AppStateMutex = Mutex<AppState>;

/// An open campaign with its settings and database connection.
pub struct Campaign {
    /// Path to the campaign folder.
    pub path: PathBuf,

    /// Campaign settings from settings.toml.
    pub settings: CampaignSettings,

    /// Database connection.
    pub db: CampaignDb,
}

/// In-memory encounter state with undo/redo support.
#[derive(Default)]
pub struct EncounterState {
    pub undo_stack: Vec<Game>,
    pub redo_stack: Vec<Game>,
}

// Keep the old type alias for backward compatibility during migration
pub type GameState = EncounterState;

impl EncounterState {
    pub async fn mutate<F>(&mut self, app: AppHandle, f: F) -> Result<(), String>
    where
        F: FnOnce(&mut Game) -> Result<(), String>,
    {
        if let Some(mut game) = self.undo_stack.last().cloned() {
            f(&mut game).map_err(|e| e.to_string())?;
            self.undo_stack.push(game.clone());
            self.redo_stack.clear();
            app.emit("game-updated", game).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("No game found".to_string())
        }
    }
}
