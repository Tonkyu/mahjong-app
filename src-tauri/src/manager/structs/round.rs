use std::collections::HashSet;

use crate::manager::{
    structs::{
        player::*,
        wall::*,
        river::*,
        result_info::*,
    },
    enums::tile::*,
    types::*,
};


pub struct Round<'a> {
    players: &'a mut [Player; 4],
    wall: &'a mut Wall,
    river: &'a mut River,
    turn_index: PlayerIndex,
}

impl<'a> Round<'a> {
    pub fn new(
        players: &'a mut [Player; 4],
        wall: &'a mut Wall,
        river: &'a mut River,
        field_wind: Wind,
        round_num: usize,
        homba_num: usize,
        accum_riichi_num: usize
    ) -> Self {
        let mut round = Round{
            players,
            wall,
            river,
            turn_index: round_num,
        };
        round.wall.distribute_tiles(round.players);
        round
    }

    fn is_end(&self) -> bool {
        self.wall.tsumo_num + self.wall.kan_num == 70 ||
        self.is_end_4kan()
    }

    fn is_end_4kan(&self) -> bool {
        if self.wall.kan_num == 4 {
            for player in self.players.iter() {
                if player.hand.ankans.len() + player.hand.minkans.len() == 4 {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn start(&mut self) -> ResultInfo {
        while !self.is_end() {
            self.turn(self.turn_index);
            self.turn_index  = if let Some(pon_player_id) = self.process_pon() {
                (pon_player_id + 1) % 4
            } else if let Some(minkan_player_id) = self.process_minkan() {
                (minkan_player_id + 1) % 4
            } else if self.process_chii().is_some() {
                self.default_next_turn_index()
            } else {
                self.default_next_turn_index()
            };
        }

        ResultInfo {
            agari_player: Some(0),
            tempai_players: HashSet::new(),
            players_point: [0, 0, 0, 0].to_vec(),
            riichi_num: 0,
        }
    }

    fn turn(&mut self, turn_index: PlayerIndex){
        let player = &mut self.players[turn_index];
        let tsumo_tile = player.normal_tsumo(self.wall);
        if player.can_agari() {

        } else {
            if player.consider_ankan(&self.river, &self.wall) {
                player.do_ankan(tsumo_tile, self.river, self.wall);
            } else if player.consider_kakan(self.river, self.wall) {
                player.do_kakan(self.river, self.wall);
            } else {
                player.dahai(self.river, self.wall);
            }
        }
    }

    fn process_pon(&mut self) -> Option<PlayerIndex> {
        for player in self.players.iter_mut() {
            if player.id != self.turn_index {
                if player.consider_pon(self.river) {
                    player.do_pon(self.river, self.wall);
                    return Some(player.id)
                }
            }
        }

        None
    }

    fn process_chii(&mut self) -> Option<PlayerIndex> {
        let player = &mut self.players[self.default_next_turn_index()];
        let chii_candidate = player.get_chii_candidate(self.river);
        if chii_candidate.len() > 0 {
            player.do_chii(self.river, self.wall, chii_candidate);
            return Some(player.id);
        }
        None
    }

    fn process_minkan(&mut self) -> Option<PlayerIndex> {
        let mut res: Option<PlayerIndex> = None;
        if self.wall.kan_num < 4 {
            for player in self.players.iter_mut() {
                if player.id != self.turn_index && player.consider_minkan(self.river) {
                    player.do_minkan(self.river, self.wall);
                    res = Some(player.id);
                    break;
                }
            }
        }

        // in Tenho and JongTama, new dora is opened after discard
        if res.is_some() {
            self.open_new_dora();
        }
        res
    }

    fn default_next_turn_index(&self) -> PlayerIndex {
        (self.turn_index + 1) % 4
    }

    fn open_new_dora(&mut self) -> Tile {
        self.wall.head_dora_displays[self.wall.kan_num - 1]
    }
}
