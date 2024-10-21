use crate::RoomState;
use crate::manager::structs::{
    player::*,
    room::{self, *},
};
use super::core::with_room_state;

#[tauri::command]
pub async fn create_room(room_state: tauri::State<'_, RoomState>) -> Result<(), String> {
    let mut room_option = room_state.room.lock().unwrap();
    if room_option.is_some() {
        println!("already created game room");
        Err("Already created game room".to_string())
    } else {
        *room_option = Some(Room::new());
        println!("new room");
        Ok(())
    }
}

#[tauri::command]
pub async fn discard_room(room_state: tauri::State<'_, RoomState>) -> Result<(), String> {
    let mut room_option = room_state.room.lock().unwrap();
    if room_option.is_some() {
        *room_option = None;
        println!("discarded game room");
        Ok(())
    } else {
        println!("already empty room");
        Ok(())
    }
}

#[tauri::command]
pub async fn start_game(room_state: tauri::State<'_, RoomState>) -> Result<(), String> {
    with_room_state(room_state, |room| {
        room.start_game();
        Ok(())
    })
}

#[tauri::command]
pub async fn get_player_info(room_state: tauri::State<'_, RoomState>) -> Result<String, String> {
    with_room_state(room_state, |room| {
        let players_json: Vec<String> = room.players.iter().map(|p| p.to_json()).collect();
        Ok(format!(r#"[{}]"#, players_json.join(", ")))
    })
}
