#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Suit {
    // 数牌 (Number)
    Manzu, // 萬子 (Characters)
    Pinzu, // 筒子 (Circles)
    Souzu, // 索子 (Bamboo)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Kaze {
    // 風牌 (Wind)
    Ton,  // 東 (East)
    Nan,  // 南 (South)
    Shaa, // 西 (West)
    Pei,  // 北 (North)
}

impl Default for Kaze {
    fn default() -> Self {
        Kaze::Ton
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Sangenpai {
    // 三元牌 (Dragon)
    Haku,  // 白 (White)
    Hatsu, // 發 (Green)
    Chun,  // 中 (Red)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Jihai {
    // 字牌 (Honor)
    Kaze(Kaze),
    Sangen(Sangenpai),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Suhai {
    pub number: u8,
    pub suit: Suit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Hai {
    // 牌 (Tile)
    Suhai(Suhai), // 数牌 (Number, 1-9)
    Jihai(Jihai), // 字牌 (Honor)
}

impl Hai {
    // simple (2-8)
    pub fn is_simple(&self) -> bool {
        match self {
            Hai::Suhai(Suhai { number: n, .. }) => *n >= 2 && *n <= 8,
            Hai::Jihai(_) => false,
        }
    }

    // terminal (1 or 9)
    pub fn is_terminal(&self) -> bool {
        match self {
            Hai::Suhai(Suhai { number: n, .. }) => *n == 1 || *n == 9,
            Hai::Jihai(_) => false,
        }
    }

    // honor tile (wind or dragon)
    pub fn is_jihai(&self) -> bool {
        matches!(self, Hai::Jihai(_))
    }

    // terminal or honor (yaochuu)
    pub fn is_yaochuu(&self) -> bool {
        self.is_terminal() || self.is_jihai()
    }
}

pub fn tile_to_index(tile: &Hai) -> usize {
    match tile {
        Hai::Suhai(Suhai {
            number: n,
            suit: Suit::Manzu,
        }) => (*n - 1) as usize, // 0-8
        Hai::Suhai(Suhai {
            number: n,
            suit: Suit::Pinzu,
        }) => (*n - 1) as usize + 9, // 9-17
        Hai::Suhai(Suhai {
            number: n,
            suit: Suit::Souzu,
        }) => (*n - 1) as usize + 18, // 18-26
        Hai::Jihai(Jihai::Kaze(Kaze::Ton)) => 27,
        Hai::Jihai(Jihai::Kaze(Kaze::Nan)) => 28,
        Hai::Jihai(Jihai::Kaze(Kaze::Shaa)) => 29,
        Hai::Jihai(Jihai::Kaze(Kaze::Pei)) => 30,
        Hai::Jihai(Jihai::Sangen(Sangenpai::Haku)) => 31,
        Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu)) => 32,
        Hai::Jihai(Jihai::Sangen(Sangenpai::Chun)) => 33,
    }
}

pub fn index_to_tile(index: usize) -> Hai {
    match index {
        0..=8 => Hai::Suhai(Suhai {
            number: (index + 1) as u8,
            suit: Suit::Manzu,
        }),
        9..=17 => Hai::Suhai(Suhai {
            number: ((index - 9) + 1) as u8,
            suit: Suit::Pinzu,
        }),
        18..=26 => Hai::Suhai(Suhai {
            number: ((index - 18) + 1) as u8,
            suit: Suit::Souzu,
        }),
        27 => Hai::Jihai(Jihai::Kaze(Kaze::Ton)),
        28 => Hai::Jihai(Jihai::Kaze(Kaze::Nan)),
        29 => Hai::Jihai(Jihai::Kaze(Kaze::Shaa)),
        30 => Hai::Jihai(Jihai::Kaze(Kaze::Pei)),
        31 => Hai::Jihai(Jihai::Sangen(Sangenpai::Haku)),
        32 => Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu)),
        33 => Hai::Jihai(Jihai::Sangen(Sangenpai::Chun)),
        _ => panic!("Invalid tile index: {}", index),
    }
}
