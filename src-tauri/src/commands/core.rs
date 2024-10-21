use crate::RoomState;
use crate::manager::structs::room::*;

pub fn with_room_state<T, F>(room_state: tauri::State<'_, RoomState>, f: F) -> Result<T, String>
where
    F: FnOnce(&mut Room) -> Result<T, String>,
{
    if let Some(ref mut room) = *room_state.room.lock().map_err(|_| "Failed to acquire lock")? {
        f(room)
    } else {
        println!("no valid game room");
        Err("no valid game room".to_string())
    }
}
