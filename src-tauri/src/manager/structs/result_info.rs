use std::collections::HashSet;
use crate::manager::types::PlayerIndex;

pub struct ResultInfo {
    pub agari_player: Option<PlayerIndex>,
    pub tempai_players: HashSet<PlayerIndex>,
    pub players_point: Vec<i8>,
    pub riichi_num: usize,
}
