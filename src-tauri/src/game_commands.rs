use std::vec;

use tauri::{AppHandle, Emitter};

use crate::game::{Action, Damage, Game, Healing, Participant};
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
pub async fn damage(
    app: AppHandle,
    state: tauri::State<'_, AppStateMutex>,
    target: u32,
    damage: Damage,
) -> Result<(), String> {
    let mut state = state.lock().await;
    state
        .gamestate
        .mutate(app, |game| {
            let time = game.time();
            let Some(participant) = game.participants.get_mut(&target) else {
                return Err(format!("No participant found with id {target}"));
            };

            let Participant::Monster(monster) = participant else {
                return Err(format!("Participant with id {target} is not a monster"));
            };

            monster.damage(time, damage);

            Ok(())
        })
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn heal(
    app: AppHandle,
    state: tauri::State<'_, AppStateMutex>,
    target: u32,
    healing: Healing,
) -> Result<(), String> {
    let mut state = state.lock().await;
    state
        .gamestate
        .mutate(app, |game| {
            let Some(participant) = game.participants.get_mut(&target) else {
                return Err(format!("No participant found with id {target}"));
            };

            let Participant::Monster(monster) = participant else {
                return Err(format!("Participant with id {target} is not a monster"));
            };

            monster.heal(healing);

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

#[tauri::command]
pub async fn set_action(
    app: AppHandle,
    state: tauri::State<'_, AppStateMutex>,
    target: u32,
    action: Action,
    available: bool,
) -> Result<(), String> {
    let mut state = state.lock().await;
    state
        .gamestate
        .mutate(app, |game| {
            let Some(participant) = game.participants.get_mut(&target) else {
                return Err(format!("No participant found with id {target}"));
            };

            participant.set_action(action, available)
                .map_err(|_| "Invalid action for this participant type".to_string())?;

            Ok(())
        })
        .await?;
    Ok(())
}
