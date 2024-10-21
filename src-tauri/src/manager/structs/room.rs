use crate::manager::{
    structs::{
        player::*,
        wall::*,
        river::*,
        round::*,
        result_info::*,
    },
    enums::tile::*,
};

pub struct Room {
    pub players: [Player; 4],
    pub field_wind: Wind,
    pub round_num: usize,
    pub homba_num: usize,
    pub accum_riichi_num: usize,
}

impl Room {
    pub fn new() -> Room {
        let players = [
                                    Player::new(0, String::from("A"), Wind::East, true),
                                    Player::new(1, String::from("B"), Wind::South, false),
                                    Player::new(2, String::from("C"), Wind::West, false),
                                    Player::new(3, String::from("D"), Wind::North, false),
                                ];
        Room {
            players,
            field_wind: Wind::East,
            round_num: 0,
            homba_num: 0,
            accum_riichi_num: 0,
        }
    }

    pub fn start_game(&mut self) {
        assert_eq!(self.field_wind, Wind::East);
        assert_eq!(self.round_num, 0);
        assert_eq!(self.homba_num, 0);
        assert_eq!(self.accum_riichi_num, 0);

        let mut play_next = true;

        while play_next {
            let result_info = self.new_round();
            play_next = if let Some(agari_player) = result_info.agari_player {
                self.accum_riichi_num = 0;
                if agari_player == self.round_num {
                    self.homba_num += 1;
                    true
                } else {
                    self.forward_parent()
                }
            } else {
                self.accum_riichi_num += result_info.riichi_num;
                self.homba_num += 1;
                true
            };

        }
    }

    fn new_round(&mut self) -> ResultInfo {
        let mut wall = Wall::new();
        let mut river = River::new();
        let mut round = Round::new(
                                    &mut self.players,
                                    &mut wall,
                                    &mut river,
                                    self.field_wind,
                                    self.round_num,
                                    self.homba_num,
                                    self.accum_riichi_num
                                );
        round.start()
    }

    fn forward_parent(&mut self) -> bool {
        if self.round_num < 3 {
            self.round_num += 1;
            true
        } else if self.field_wind == Wind::East {
            self.field_wind = Wind::South;
            self.round_num = 0;
            true
        } else {
            false
        }
    }
}
