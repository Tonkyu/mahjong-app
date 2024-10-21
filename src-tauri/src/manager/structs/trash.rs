use crate::manager::{
    types::*,
    enums::tile::*,
};

pub struct Trash {
    pub player_id: PlayerIndex,
    pub tile: Tile,
    pub tedashi: bool,
    pub is_riichi: bool,
    pub in_river: bool, // false when pon or chi
}
