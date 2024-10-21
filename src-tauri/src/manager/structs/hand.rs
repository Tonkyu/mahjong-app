use crate::manager::enums::tile::Tile;
use std::collections::{HashMap, HashSet};


pub struct Hand {
    pub tehai: HashMap<Tile, usize>,
    pub pons: HashSet<Tile>,
    pub chiis: HashSet<(Tile,Tile, Tile)>,
    pub ankans: HashSet<Tile>,
    pub minkans: HashSet<Tile>,
}

impl Hand {
    pub fn to_json(&self) -> String {
        format!(r#"{{"hoge": "(to be implemented)"}}"#)
    }
}

impl Hand {
    pub fn new() -> Hand {
        let mut tehai = HashMap::new();
        for tile in enum_iterator::all::<Tile>() {
            tehai.insert(tile, 0);
        }
        Hand {
            tehai: tehai,
            pons: HashSet::new(),
            chiis: HashSet::new(),
            ankans: HashSet::new(),
            minkans: HashSet::new(),
        }
    }

    pub fn add_tile(&mut self, tile: Tile) {
        if let Some(count) = self.tehai.get_mut(&tile) {
            *count += 1;
        } else {
            panic!("Error on Hand::add_tile");
        }
    }

    pub fn remove_tile(&mut self, tile: Tile) {
        assert_ne!(*self.tehai.entry(tile).or_insert(0), 0, "Tried to remove non-existing tile");
        *self.tehai.entry(tile).or_insert(0) -= 1;
    }

    pub fn is_menzen(&self) -> bool {
        true
    }

    pub fn have_in_tehai(&self, tile: Tile) -> bool {
        if let Some(count) = self.tehai.get(&tile) {
            *count > 1
        } else {
            panic!("Error on Hand::have_in_tehai");
        }
    }

    pub fn have_anko(&self, tile: Tile) -> bool {
        if let Some(count) = self.tehai.get(&tile) {
            *count >= 3
        } else {
            panic!("Error on Hand::have_anko");
        }
    }
}

