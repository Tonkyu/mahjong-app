mod manager;
use manager::room_state::*;

mod commands;
use crate::commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let room_state: RoomState = RoomState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(room_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            game_manager::create_room,
            game_manager::discard_room,
            game_manager::get_player_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
