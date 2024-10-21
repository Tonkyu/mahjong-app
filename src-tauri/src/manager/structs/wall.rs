use crate::manager::{
    enums::tile::*,
    structs::player::*,
};

pub struct Wall {
    tehais: Vec<Tile>,
    tsumos: Vec<Tile>,
    pub head_dora_displays: Vec<Tile>,
    pub ura_dora_displays: Vec<Tile>,
    rinsyan: Vec<Tile>,

    pub tsumo_num: usize,
    pub kan_num: usize,
}

impl Wall {
    pub fn get_next_tsumo(&mut self) -> Tile {
        let num = self.tsumo_num;
        self.tsumo_num += 1;
        self.tsumos[num]
    }

    pub fn get_next_rinsyan(&mut self) -> Tile {
        let num = self.kan_num;
        self.kan_num += 1;
        self.rinsyan[num]
    }

    pub fn new() -> Wall {
        let mut tiles = Vec::<Tile>::new();

        for _ in 0..4 {
            for tile in enum_iterator::all::<Tile>() {
                tiles.push(tile);
            }
        }

        assert_eq!(tiles.len(), 136);

        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        tiles.shuffle(&mut rng);
        Wall {
            tehais: tiles[..52].to_vec(),
            tsumos: tiles[52..122].to_vec(),
            head_dora_displays: tiles[122..127].to_vec(),
            ura_dora_displays: tiles[127..132].to_vec(),
            rinsyan: tiles[132..136].to_vec(),
            tsumo_num: 0,
            kan_num: 0,
        }
    }

    pub fn distribute_tiles(&self, players: &mut [Player; 4]) {
        for (i, player) in players.iter_mut().enumerate() {
            for tile in &self.tehais[i*13..(i+1)*13] {
                player.hand.add_tile(*tile);
            }
        }
    }
}

