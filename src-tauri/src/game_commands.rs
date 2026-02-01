// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

use std::vec;

use serde::Deserialize;
use tauri::{AppHandle, Emitter, State as TauriState};

use crate::game::{Action, Condition, Damage, Expiry, Game, Healing, Participant, ParticipantId};
use crate::state::AppStateMutex;

#[tauri::command]
pub async fn get_game(state: TauriState<'_, AppStateMutex>) -> Result<Game, String> {
    let state = state.lock().await;
    match state.encounter.undo_stack.last() {
        Some(game) => Ok(game.clone()),
        None => Err("No game found".to_string()),
    }
}

#[tauri::command]
pub async fn new_game(app: AppHandle, state: TauriState<'_, AppStateMutex>) -> Result<(), String> {
    let mut state = state.lock().await;
    state.encounter.undo_stack = vec![Game::new()];
    state.encounter.redo_stack.clear();
    app.emit("game-updated", ()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn next_turn(app: AppHandle, state: TauriState<'_, AppStateMutex>) -> Result<(), String> {
    let mut state = state.lock().await;
    state
        .encounter
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
    state: TauriState<'_, AppStateMutex>,
    target: ParticipantId,
    damage: Damage,
) -> Result<(), String> {
    let mut state = state.lock().await;
    state
        .encounter
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
    state: TauriState<'_, AppStateMutex>,
    target: ParticipantId,
    healing: Healing,
) -> Result<(), String> {
    let mut state = state.lock().await;
    state
        .encounter
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
pub async fn add_conditions(
    app: AppHandle,
    state: TauriState<'_, AppStateMutex>,
    target: ParticipantId,
    conditions: Vec<Condition>,
) -> Result<(), String> {
    let mut state = state.lock().await;
    state
        .encounter
        .mutate(app, |game| {
            let time = game.time();
            let Some(participant) = game.participants.get_mut(&target) else {
                return Err(format!("No participant found with id {target}"));
            };

            match participant {
                Participant::Monster(monster) => monster.conditions.extend(conditions),
                Participant::Player(player) => player.conditions.extend(conditions),
                Participant::Lair(_) => return Err("Lairs may not have conditions".into()),
            }

            Ok(())
        })
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn undo(app: AppHandle, state: TauriState<'_, AppStateMutex>) -> Result<(), String> {
    let mut state = state.lock().await;
    if state.encounter.undo_stack.len() > 1 {
        let game = state.encounter.undo_stack.pop().unwrap();
        state.encounter.redo_stack.push(game.clone());
        app.emit("game-updated", game).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        return Err("Undo stack is empty".to_string());
    }
}

#[tauri::command]
pub async fn redo(app: AppHandle, state: TauriState<'_, AppStateMutex>) -> Result<(), String> {
    let mut state = state.lock().await;
    if let Some(game) = state.encounter.redo_stack.pop() {
        state.encounter.undo_stack.push(game.clone());
        app.emit("game-updated", game).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        return Err("Redo stack is empty".to_string());
    }
}

#[tauri::command]
pub async fn set_action(
    app: AppHandle,
    state: TauriState<'_, AppStateMutex>,
    target: ParticipantId,
    action: Action,
    available: bool,
) -> Result<(), String> {
    let mut state = state.lock().await;
    state
        .encounter
        .mutate(app, |game| {
            let Some(participant) = game.participants.get_mut(&target) else {
                return Err(format!("No participant found with id {target}"));
            };

            participant
                .set_action(action, available)
                .map_err(|_| "Invalid action for this participant type".to_string())?;

            Ok(())
        })
        .await?;
    Ok(())
}
