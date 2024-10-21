use std::sync::Mutex;
use super::structs::room::Room;

pub struct RoomState {
    pub room: Mutex<Option<Room>>,
}

impl RoomState {
    pub fn new() -> Self {
        Self {
            room: Mutex::from(None),
        }
    }
}
