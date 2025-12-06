use super::tiles::Hai;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MentsuType {
    Shuntsu, // 順子 (Sequence)
    Koutsu,  // 刻子 (Triplet)
    Kantsu,  // 槓子 (Kan/Quad)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mentsu {
    // 面子 (Meld)
    pub mentsu_type: MentsuType,
    pub is_minchou: bool, // 明張 (meld open?)
    pub tiles: [Hai; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Machi {
    // 待ち (Wait)
    Ryanmen, // 両面 (Two-Sided)
    Tanki,   // 単騎 (Pair wait)
    Penchan, // 辺張 (Edge wait)
    Kanchan, // 嵌張 (Closed wait)
    Shanpon, // 双碰 (Triplet-pair wait)

    // Special waits for Yakuman
    KokushiIchimen,  // 国士一面 (Kokushi single wait)
    KokushiJusanmen, // 国士十三面 (Kokushi 13-sided wait)
}

#[derive(Debug, Clone, Copy)]
pub struct AgariHand {
    // 和了手 (Winning Hand)
    pub mentsu: [Mentsu; 4], // 面子 (4 melds)
    pub atama: (Hai, Hai),   // 頭 (1 pair)
    pub agari_hai: Hai,      // 和了牌 (The winning tile)
    pub machi: Machi,        // 待ち (The wait type)
}

#[derive(Debug, Clone)]
pub enum HandOrganization {
    YonmentsuIchiatama(AgariHand), // 四面子一頭 (4 Melds, 1 Pair)
    Irregular {
        // Irregular Hand
        counts: [u8; 34],
        agari_hai: Hai,
    },
}

#[derive(Debug, Clone)]
pub enum HandStructure {
    YonmentsuIchiatama(AgariHand),

    // 七対子 (Seven Pairs)
    Chiitoitsu {
        pairs: [(Hai, Hai); 7],
        agari_hai: Hai,
        machi: Machi,
    },

    // 国士無双 (Thirteen Orphans)
    KokushiMusou {
        tiles: [Hai; 13],
        atama: (Hai, Hai),
        _agari_hai: Hai,
        _machi: Machi,
    },

    // 九蓮宝燈 (Nine Gates)
    ChuurenPoutou {
        hand: AgariHand,
        _is_junsei: bool, // 純正 (true 9-sided wait)
    },
}
