// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

//! Encounter savepoint service.
//!
//! Manages periodic snapshots of encounter state for recovery after
//! unexpected app closure. Savepoints are stored as complete encounter
//! state in TOML format within the campaign's `saves/` folder.

use std::fs;
use std::io::Error as IoError;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use toml::{de::Error as DeError, ser::Error as SerError};

use crate::game::Game;

/// Errors that can occur during savepoint operations.
#[derive(Debug, Error)]
pub enum SavepointError {
    /// Failed to read or write a savepoint file.
    #[error("I/O error: {0}")]
    Io(#[from] IoError),

    /// Failed to serialize encounter state.
    #[error("serialization error: {0}")]
    Serialize(#[from] SerError),

    /// Failed to deserialize encounter state.
    #[error("deserialization error: {0}")]
    Deserialize(#[from] DeError),
}

/// A complete snapshot of encounter state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Savepoint {
    /// The turn number when this savepoint was created.
    pub turn_number: u32,

    /// The complete game state.
    pub game: Game,
}

/// Service for managing encounter savepoints.
pub struct SavepointService {
    saves_dir: PathBuf,
    max_count: u32,
}

impl SavepointService {
    /// Creates a new savepoint service for the given campaign.
    ///
    /// - `campaign_path`: Path to the campaign folder.
    /// - `max_count`: Maximum number of savepoints to retain.
    pub fn new(campaign_path: &Path, max_count: u32) -> Self {
        Self {
            saves_dir: campaign_path.join("saves"),
            max_count,
        }
    }

    /// Writes a savepoint for the current encounter state.
    ///
    /// The savepoint is saved as `turn-NNN.toml` where NNN is the turn number.
    /// Prunes older savepoints if the count exceeds `max_count`.
    pub fn write(&self, game: &Game) -> Result<PathBuf, SavepointError> {
        // Ensure saves directory exists
        fs::create_dir_all(&self.saves_dir)?;

        // Calculate turn number (round * turns_per_round + turn_in_round)
        let turn_number = self.calculate_turn_number(game);
        let filename = format!("turn-{:05}.toml", turn_number);
        let path = self.saves_dir.join(&filename);

        // Create savepoint
        let savepoint = Savepoint {
            turn_number,
            game: game.clone(),
        };

        // Serialize and write
        let content = toml::to_string_pretty(&savepoint)?;
        fs::write(&path, content)?;

        // Prune old savepoints
        self.prune()?;

        Ok(path)
    }

    /// Reads the most recent savepoint, if any.
    pub fn read_latest(&self) -> Result<Option<Savepoint>, SavepointError> {
        let savepoints = self.list_savepoints()?;
        if let Some(path) = savepoints.last() {
            let content = fs::read_to_string(path)?;
            let savepoint: Savepoint = toml::from_str(&content)?;
            Ok(Some(savepoint))
        } else {
            Ok(None)
        }
    }

    /// Lists all savepoint files, sorted by turn number (oldest first).
    pub fn list_savepoints(&self) -> Result<Vec<PathBuf>, SavepointError> {
        if !self.saves_dir.exists() {
            return Ok(Vec::new());
        }

        let mut savepoints: Vec<PathBuf> = fs::read_dir(&self.saves_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension().and_then(|s| s.to_str()) == Some("toml")
                    && p.file_name()
                        .and_then(|s| s.to_str())
                        .is_some_and(|s| s.starts_with("turn-"))
            })
            .collect();

        savepoints.sort();
        Ok(savepoints)
    }

    /// Clears all savepoints (e.g., when an encounter ends normally).
    pub fn clear(&self) -> Result<(), SavepointError> {
        for path in self.list_savepoints()? {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// Prunes old savepoints to stay within `max_count`.
    fn prune(&self) -> Result<(), SavepointError> {
        let savepoints = self.list_savepoints()?;
        let excess = savepoints.len().saturating_sub(self.max_count as usize);

        for path in savepoints.into_iter().take(excess) {
            fs::remove_file(path)?;
        }

        Ok(())
    }

    /// Calculates a monotonic turn number from the game state.
    ///
    /// Uses `round * max_turns + turn` where max_turns is the participant count.
    fn calculate_turn_number(&self, game: &Game) -> u32 {
        let participants = game.order.len() as u32;
        if participants == 0 {
            return 0;
        }
        game.round * participants + game.turn
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_savepoint_write_and_read() {
        let temp = tempdir().unwrap();
        let service = SavepointService::new(temp.path(), 50);

        let game = Game::new();
        let path = service.write(&game).unwrap();

        assert!(path.exists());
        assert!(path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("turn-"));

        let loaded = service.read_latest().unwrap().unwrap();
        assert_eq!(loaded.turn_number, 0);
    }

    #[test]
    fn test_savepoint_pruning() {
        let temp = tempdir().unwrap();
        let service = SavepointService::new(temp.path(), 3);

        // Write more than max_count savepoints
        for _ in 0..5 {
            let game = Game::new();
            service.write(&game).unwrap();
        }

        let savepoints = service.list_savepoints().unwrap();
        assert!(savepoints.len() <= 3);
    }

    #[test]
    fn test_savepoint_clear() {
        let temp = tempdir().unwrap();
        let service = SavepointService::new(temp.path(), 50);

        let game = Game::new();
        service.write(&game).unwrap();
        service.write(&game).unwrap();

        assert!(!service.list_savepoints().unwrap().is_empty());

        service.clear().unwrap();
        assert!(service.list_savepoints().unwrap().is_empty());
    }
}
