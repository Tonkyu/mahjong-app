use enum_iterator::Sequence;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Sequence)]
pub enum Tile {
    Character(Character), // 萬子 (1-9)
    Bamboo(Bamboo),       // 索子 (1-9)
    Dot(Dot),             // 筒子 (1-9)
    Wind(Wind),           // 風牌 (東南西北)
    Dragon(Dragon),       // 三元牌 (白發中)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Sequence)]
pub enum Bamboo {
    S1, S2, S3, S4, S5, S6, S7, S8, S9,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Sequence)]
pub enum Character {
    M1, M2, M3, M4, M5, M6, M7, M8, M9,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Sequence)]
pub enum Dot {
    P1, P2, P3, P4, P5, P6, P7, P8, P9,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Sequence)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Sequence)]
pub enum Dragon {
    White,
    Green,
    Red,
}


impl Tile {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Tile::Character(Character::M1))
            || matches!(self, Tile::Character(Character::M9))
            || matches!(self, Tile::Bamboo(Bamboo::S1))
            || matches!(self, Tile::Bamboo(Bamboo::S9))
            || matches!(self, Tile::Dot(Dot::P1))
            || matches!(self, Tile::Dot(Dot::P9))
    }

    pub fn is_wind(&self) -> bool {
        matches!(self, Tile::Wind(_))
    }

    pub fn is_dragon(&self) -> bool {
        matches!(self, Tile::Dragon(_))
    }

    pub fn is_jihai(&self) -> bool {
        self.is_wind() || self.is_dragon()
    }

    pub fn is_kazuhai(&self) -> bool {
        !self.is_jihai()
    }

    pub fn next_kazuhai(&self) -> Option<Tile> {
        match self {
            Tile::Character(Character::M1) => Some(Tile::Character(Character::M2)),
            Tile::Character(Character::M2) => Some(Tile::Character(Character::M3)),
            Tile::Character(Character::M3) => Some(Tile::Character(Character::M4)),
            Tile::Character(Character::M4) => Some(Tile::Character(Character::M5)),
            Tile::Character(Character::M5) => Some(Tile::Character(Character::M6)),
            Tile::Character(Character::M6) => Some(Tile::Character(Character::M7)),
            Tile::Character(Character::M7) => Some(Tile::Character(Character::M8)),
            Tile::Character(Character::M8) => Some(Tile::Character(Character::M9)),

            Tile::Bamboo(Bamboo::S1) => Some(Tile::Bamboo(Bamboo::S2)),
            Tile::Bamboo(Bamboo::S2) => Some(Tile::Bamboo(Bamboo::S3)),
            Tile::Bamboo(Bamboo::S3) => Some(Tile::Bamboo(Bamboo::S4)),
            Tile::Bamboo(Bamboo::S4) => Some(Tile::Bamboo(Bamboo::S5)),
            Tile::Bamboo(Bamboo::S5) => Some(Tile::Bamboo(Bamboo::S6)),
            Tile::Bamboo(Bamboo::S6) => Some(Tile::Bamboo(Bamboo::S7)),
            Tile::Bamboo(Bamboo::S7) => Some(Tile::Bamboo(Bamboo::S8)),
            Tile::Bamboo(Bamboo::S8) => Some(Tile::Bamboo(Bamboo::S9)),

            Tile::Dot(Dot::P1) => Some(Tile::Dot(Dot::P2)),
            Tile::Dot(Dot::P2) => Some(Tile::Dot(Dot::P3)),
            Tile::Dot(Dot::P3) => Some(Tile::Dot(Dot::P4)),
            Tile::Dot(Dot::P4) => Some(Tile::Dot(Dot::P5)),
            Tile::Dot(Dot::P5) => Some(Tile::Dot(Dot::P6)),
            Tile::Dot(Dot::P6) => Some(Tile::Dot(Dot::P7)),
            Tile::Dot(Dot::P7) => Some(Tile::Dot(Dot::P8)),
            Tile::Dot(Dot::P8) => Some(Tile::Dot(Dot::P9)),

            _ => None,
        }
    }

    pub fn prev_kazuhai(&self) -> Option<Tile> {
        match self {
            Tile::Character(Character::M2) => Some(Tile::Character(Character::M1)),
            Tile::Character(Character::M3) => Some(Tile::Character(Character::M2)),
            Tile::Character(Character::M4) => Some(Tile::Character(Character::M3)),
            Tile::Character(Character::M5) => Some(Tile::Character(Character::M4)),
            Tile::Character(Character::M6) => Some(Tile::Character(Character::M5)),
            Tile::Character(Character::M7) => Some(Tile::Character(Character::M6)),
            Tile::Character(Character::M8) => Some(Tile::Character(Character::M7)),
            Tile::Character(Character::M9) => Some(Tile::Character(Character::M8)),

            Tile::Bamboo(Bamboo::S2) => Some(Tile::Bamboo(Bamboo::S1)),
            Tile::Bamboo(Bamboo::S3) => Some(Tile::Bamboo(Bamboo::S2)),
            Tile::Bamboo(Bamboo::S4) => Some(Tile::Bamboo(Bamboo::S3)),
            Tile::Bamboo(Bamboo::S5) => Some(Tile::Bamboo(Bamboo::S4)),
            Tile::Bamboo(Bamboo::S6) => Some(Tile::Bamboo(Bamboo::S5)),
            Tile::Bamboo(Bamboo::S7) => Some(Tile::Bamboo(Bamboo::S6)),
            Tile::Bamboo(Bamboo::S8) => Some(Tile::Bamboo(Bamboo::S7)),
            Tile::Bamboo(Bamboo::S9) => Some(Tile::Bamboo(Bamboo::S8)),

            Tile::Dot(Dot::P2) => Some(Tile::Dot(Dot::P1)),
            Tile::Dot(Dot::P3) => Some(Tile::Dot(Dot::P2)),
            Tile::Dot(Dot::P4) => Some(Tile::Dot(Dot::P3)),
            Tile::Dot(Dot::P5) => Some(Tile::Dot(Dot::P4)),
            Tile::Dot(Dot::P6) => Some(Tile::Dot(Dot::P5)),
            Tile::Dot(Dot::P7) => Some(Tile::Dot(Dot::P6)),
            Tile::Dot(Dot::P8) => Some(Tile::Dot(Dot::P7)),
            Tile::Dot(Dot::P9) => Some(Tile::Dot(Dot::P8)),

            _ => None,
        }
    }
}
