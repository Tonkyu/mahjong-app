use crate::manager::structs::room::*;

#[tauri::command]
pub fn create_room() -> usize {
    let mut room = Room::new();
    room.room_id
}
