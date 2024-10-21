use rand::Rng;

use crate::manager::{
    types::*,
    structs::{
        hand::*,
        wall::*,
        river::*,
    },
    enums::tile::*,
};

pub struct Player {
    pub id: PlayerIndex,
    pub name: String,
    pub hand: Hand,
    pub wind: Wind,
    pub is_parent: bool,
    pub point: usize,
}

impl Player {
    pub fn to_json(&self) -> String {
        format!(r#"{{"id": {}, "name": "{}", "is_parent": {}, "point": {}, "wind": "{}", "hand": {}}}"#,
                self.id, self.name, self.is_parent, self.point, self.wind, self.hand.to_json())
    }
}

impl Player {
    pub fn new(id: PlayerIndex, name: String, wind: Wind, is_parent: bool) -> Player {
        Player {
            id,
            name,
            hand: Hand::new(),
            wind,
            is_parent,
            point: 25000,
        }
    }

    fn turn(&mut self, wall: &mut Wall, river: &mut River) -> Tile {
        self.normal_tsumo(wall);
        self.dahai(river, wall)
    }

    pub fn normal_tsumo(&mut self, wall: &mut Wall) -> Tile {
        let tsumo_tile = wall.get_next_tsumo();
        self.hand.add_tile(tsumo_tile);
        tsumo_tile
    }

    fn rinsyan_tsumo(&mut self, wall: &mut Wall) {
        let rinsyan_tile = wall.get_next_rinsyan();
        self.hand.add_tile(rinsyan_tile);
    }

    pub fn dahai(&mut self, river: &mut River, wall: &Wall) -> Tile {
        let tile = self.select_tile_random();
        self.hand.remove_tile(tile);
        river.add_trash(self.id, tile, false, false);
        tile
    }

    fn select_tile_left(&self) -> Tile {
        for tile in enum_iterator::all::<Tile>() {
            if let Some(&count) = self.hand.tehai.get(&tile) {
                if count > 0 {
                    return tile;
                }
            }
        }
        panic!("No valid tile found");
    }

    fn select_tile_random(&self) -> Tile {
        let total_weight: f64 = self.hand.tehai
        .iter()
        .filter(|(_, &weight)| weight != 0)
        .map(|(_, weight)| 1.0 / (*weight as f64) / (*weight as f64) )
        .sum();
        let mut rng = rand::thread_rng();
        let mut random_weight = rng.gen_range(0.0..total_weight);
        for (tile, weight) in self.hand.tehai.iter().filter(|(_, &weight)| weight != 0) {
            let inverse_weight = 1.0 / (*weight as f64) / (*weight as f64) ;
            if random_weight < inverse_weight {
                return tile.clone();
            }
            random_weight -= inverse_weight;
        }
        panic!("No valid tile found")
    }


    pub fn can_ankan(&self, river: &River) -> bool {
        false
    }

    pub fn can_riichi(&self) -> bool {
        false
    }

    pub fn can_agari(&self) -> bool {
        false
    }

    pub fn consider_ankan(&self, river: &River, wall: &Wall) -> bool {
        if self.can_ankan(river) {
            true // todo
        } else {
            false
        }
    }


    pub fn consider_pon(&self, river: &River) -> bool {
        if self.can_pon(river) {
            true // todo
        } else{
            false
        }
    }

    pub fn consider_minkan(&self, river: &River) -> bool {
        if self.can_minkan(river) {
            true // todo
        } else {
            false
        }
    }

    pub fn consider_kakan(&self, river: &River, wall: &Wall) -> bool {
        if self.can_kakan() {
            true // todo
        } else {
            false
        }
    }

    fn can_minkan(&self, river: &River) -> bool {
        if let Some(count) = self.hand.tehai.get(&river.get_last_trash().tile) {
            *count == 3
        } else {
            false
        }
    }

    fn can_kakan(&self) -> bool {
        false
    }

    pub fn do_ankan(&mut self, tile: Tile, river: &mut River, wall: &mut Wall) -> Tile {
        *self.hand.tehai.get_mut(&tile).unwrap() = 0;
        self.hand.ankans.insert(tile);

        self.rinsyan_tsumo(wall);
        self.dahai(river, wall)
    }

    pub fn do_minkan(&mut self, river: &mut River, wall: &mut Wall) -> Tile {
        let minkan_trash = river.get_last_trash_mut();
        *self.hand.tehai.get_mut(&minkan_trash.tile).unwrap() -= 3;
        self.hand.minkans.insert(minkan_trash.tile);
        minkan_trash.in_river = false;

        self.rinsyan_tsumo(wall);
        self.dahai(river, wall)
    }

    pub fn do_pon(&mut self, river: &mut River, wall: &Wall) -> Tile {
        let pon_trash = river.get_last_trash_mut();
        *self.hand.tehai.get_mut(&pon_trash.tile).unwrap() -= 2;
        self.hand.pons.insert(pon_trash.tile);
        pon_trash.in_river = false;
        self.dahai(river, wall)
    }

    pub fn do_kakan(&mut self, river: &mut River, wall: &mut Wall) -> Tile {
        let kakan_trash = river.get_last_trash_mut();
        assert!(self.hand.pons.remove(&kakan_trash.tile));
        self.hand.minkans.insert(kakan_trash.tile);
        kakan_trash.in_river = false;

        // check_chankan()

        self.rinsyan_tsumo(wall);
        self.dahai(river, wall)
    }

    fn can_pon(&self, river: &River) -> bool {
        if let Some(count) = self.hand.tehai.get(&river.get_last_trash().tile) {
            *count >= 2
        } else {
            false
        }
    }

    pub fn do_chii(&mut self, river: &mut River, wall: &Wall, chii_candidate: Vec<(Tile, Tile)>) -> Tile {
        let chii_trash = river.get_last_trash_mut();
        let candidate_index = 0;

        let tile0 = chii_candidate[candidate_index].0;
        let tile1 = chii_candidate[candidate_index].1;
        *self.hand.tehai.get_mut(&tile0).unwrap() -= 1;
        *self.hand.tehai.get_mut(&tile1).unwrap() -= 1;
        self.hand.chiis.insert((chii_trash.tile, tile0, tile1));
        chii_trash.in_river = false;
        self.dahai(river, wall)
    }

    pub fn get_chii_candidate(&self, river: &River) -> Vec<(Tile, Tile)> {
        let tile = river.get_last_trash().tile;
        let tile1f = tile.next_kazuhai();
        let tile2f = if tile1f.is_some() { tile1f.unwrap().next_kazuhai() } else { None };
        let tile1b = tile.prev_kazuhai();
        let tile2b = if tile1b.is_some() { tile1b.unwrap().prev_kazuhai() } else { None };

        let mut res = Vec::new();

        if let (Some(t1), Some(t2)) = (tile1f, tile2f) {
            if self.hand.have_in_tehai(t1) && self.hand.have_in_tehai(t2) {
                res.push((t1, t2));
            }
        }
        if let (Some(t1), Some(t2)) = (tile1f, tile1b) {
            if self.hand.have_in_tehai(t1) && self.hand.have_in_tehai(t2) {
                res.push((t1, t2));
            }
        }
        if let (Some(t1), Some(t2)) = (tile1b, tile2b) {
            if self.hand.have_in_tehai(t1) && self.hand.have_in_tehai(t2) {
                res.push((t1, t2));
            }
        }
        res
    }
}
