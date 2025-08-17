use sqlx::SqlitePool;
use tauri::async_runtime::Mutex;

use crate::game::Game;

pub struct AppState {
    pub gamestate: GameState,
    // pub db: SqlitePool,
}

pub type AppStateMutex = Mutex<AppState>;

#[derive(Default)]
pub struct GameState {
    pub undo_stack: Vec<Game>,
    pub redo_stack: Vec<Game>,
}

impl GameState {
    pub async fn mutate<F>(&mut self, f: F) -> Result<(), String>
    where
        F: FnOnce(&mut Game) -> Result<(), String>,
    {
        if let Some(mut game) = self.undo_stack.last().cloned() {
            f(&mut game).map_err(|e| e.to_string())?;
            self.undo_stack.push(game);
            self.redo_stack.clear();
            Ok(())
        } else {
            Err("No game found".to_string())
        }
    }
}
