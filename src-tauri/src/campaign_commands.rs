// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! Tauri commands for campaign lifecycle management.

use std::path::PathBuf;

use serde::Serialize;
use tauri::State as TauriState;

use crate::config::CampaignSettings;
use crate::db::CampaignDb;
use crate::state::{AppStateMutex, Campaign};

/// Summary information about a campaign for list display.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignSummary {
    /// Path to the campaign folder.
    pub path: PathBuf,

    /// Display name of the campaign.
    pub name: String,

    /// When the campaign was created (ISO 8601 format).
    pub created: String,
}

/// Lists all available campaigns in the data directory.
#[tauri::command]
pub async fn list_campaigns(
    state: TauriState<'_, AppStateMutex>,
) -> Result<Vec<CampaignSummary>, String> {
    let state = state.lock().await;
    let campaign_paths = state.paths.list_campaigns().map_err(|e| e.to_string())?;

    let mut summaries = Vec::with_capacity(campaign_paths.len());
    for path in campaign_paths {
        match CampaignSettings::load(&path) {
            Ok(settings) => {
                summaries.push(CampaignSummary {
                    path,
                    name: settings.name,
                    created: settings.created.to_rfc3339(),
                });
            }
            Err(e) => {
                // Log but skip campaigns with invalid settings
                eprintln!("skipping campaign at {}: {}", path.display(), e);
            }
        }
    }

    Ok(summaries)
}

/// Creates a new campaign with the given name.
///
/// Returns the path to the created campaign folder.
#[tauri::command]
pub async fn create_campaign(
    state: TauriState<'_, AppStateMutex>,
    name: String,
) -> Result<PathBuf, String> {
    let mut state = state.lock().await;

    // Create campaign folder
    let path = state
        .paths
        .create_campaign_folder(&name)
        .map_err(|e| e.to_string())?;

    // Create and save campaign settings
    let settings = CampaignSettings::new(&name);
    settings.save(&path).map_err(|e| e.to_string())?;

    // Create and migrate database
    let db_path = CampaignSettings::db_path(&path);
    let db = CampaignDb::open(&db_path)
        .await
        .map_err(|e| e.to_string())?;
    db.migrate().await.map_err(|e| e.to_string())?;

    // Add to recent campaigns and save app settings
    state.app_settings.add_recent_campaign(path.clone());
    state
        .app_settings
        .save(state.paths.preference_dir())
        .map_err(|e| e.to_string())?;

    // Close the database (campaign isn't open yet, just created)
    db.close().await;

    Ok(path)
}

/// Opens an existing campaign at the given path.
///
/// Closes any currently open campaign first.
#[tauri::command]
pub async fn open_campaign(
    state: TauriState<'_, AppStateMutex>,
    path: PathBuf,
) -> Result<CampaignSummary, String> {
    let mut state = state.lock().await;

    // Close any existing campaign
    if let Some(existing) = state.campaign.take() {
        existing.db.close().await;
    }

    // Validate campaign folder exists
    if !path.exists() || !path.join("settings.toml").exists() {
        return Err(format!("campaign not found: {}", path.display()));
    }

    // Load campaign settings
    let settings = CampaignSettings::load(&path).map_err(|e| e.to_string())?;

    // Open database and run migrations
    let db_path = CampaignSettings::db_path(&path);
    let db = CampaignDb::open(&db_path)
        .await
        .map_err(|e| e.to_string())?;
    db.migrate().await.map_err(|e| e.to_string())?;

    // Create summary for return
    let summary = CampaignSummary {
        path: path.clone(),
        name: settings.name.clone(),
        created: settings.created.to_rfc3339(),
    };

    // Store campaign in state
    state.campaign = Some(Campaign {
        path: path.clone(),
        settings,
        db,
    });

    // Update recent campaigns
    state.app_settings.add_recent_campaign(path);
    state
        .app_settings
        .save(state.paths.preference_dir())
        .map_err(|e| e.to_string())?;

    Ok(summary)
}

/// Closes the currently open campaign.
///
/// Does nothing if no campaign is open.
#[tauri::command]
pub async fn close_campaign(state: TauriState<'_, AppStateMutex>) -> Result<(), String> {
    let mut state = state.lock().await;

    if let Some(campaign) = state.campaign.take() {
        // Save any pending settings changes
        campaign
            .settings
            .save(&campaign.path)
            .map_err(|e| e.to_string())?;

        // Close database connection
        campaign.db.close().await;
    }

    Ok(())
}

/// Gets information about the currently open campaign.
///
/// Returns `None` if no campaign is open.
#[tauri::command]
pub async fn get_current_campaign(
    state: TauriState<'_, AppStateMutex>,
) -> Result<Option<CampaignSummary>, String> {
    let state = state.lock().await;

    match &state.campaign {
        Some(campaign) => Ok(Some(CampaignSummary {
            path: campaign.path.clone(),
            name: campaign.settings.name.clone(),
            created: campaign.settings.created.to_rfc3339(),
        })),
        None => Ok(None),
    }
}
