mod dice;
mod dice_commands;
mod game;
mod game_commands;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            dice_commands::roll,
            game_commands::new_game,
            game_commands::next_turn,
            game_commands::undo,
            game_commands::redo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
