use crate::manager::{
    types::*,
    structs::trash::*,
    enums::tile::*,
};

pub struct River {
    trashes: Vec<Trash>,
}

impl River {
    pub fn new() -> River {
        River {
            trashes: Vec::new(),
        }
    }

    pub fn get_last_trash(&self) -> &Trash {
        self.trashes.last().unwrap()
    }

    pub fn get_last_trash_mut(&mut self) -> &mut Trash {
        self.trashes.last_mut().unwrap()
    }

    pub fn add_trash(&mut self, player_id: PlayerIndex, tile: Tile, tedashi: bool, is_riichi: bool){
        self.trashes.push(
            Trash {
                player_id,
                tile,
                tedashi,
                is_riichi,
                in_river: true,
            }
        );
    }
}
