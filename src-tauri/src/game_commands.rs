use std::vec;

use tauri::{AppHandle, Emitter};

use crate::game::Game;
use crate::state::AppStateMutex;

#[tauri::command]
pub async fn get_game(state: tauri::State<'_, AppStateMutex>) -> Result<Game, String> {
    let state = state.lock().await;
    match state.gamestate.undo_stack.last() {
        Some(game) => Ok(game.clone()),
        None => Err("No game found".to_string()),
    }
}

#[tauri::command]
pub async fn new_game(
    app: AppHandle,
    state: tauri::State<'_, AppStateMutex>,
) -> Result<(), String> {
    let mut state = state.lock().await;
    state.gamestate.undo_stack = vec![Game::new()];
    state.gamestate.redo_stack.clear();
    app.emit("game-updated", ()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn next_turn(
    app: AppHandle,
    state: tauri::State<'_, AppStateMutex>,
) -> Result<(), String> {
    let mut state = state.lock().await;
    state
        .gamestate
        .mutate(app, |game| {
            game.next_turn();
            Ok(())
        })
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn undo(app: AppHandle, state: tauri::State<'_, AppStateMutex>) -> Result<(), String> {
    let mut state = state.lock().await;
    if state.gamestate.undo_stack.len() > 1 {
        let game = state.gamestate.undo_stack.pop().unwrap();
        state.gamestate.redo_stack.push(game.clone());
        app.emit("game-updated", game).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        return Err("Undo stack is empty".to_string());
    }
}

#[tauri::command]
pub async fn redo(app: AppHandle, state: tauri::State<'_, AppStateMutex>) -> Result<(), String> {
    let mut state = state.lock().await;
    if let Some(game) = state.gamestate.redo_stack.pop() {
        state.gamestate.undo_stack.push(game.clone());
        app.emit("game-updated", game).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        return Err("Redo stack is empty".to_string());
    }
}
