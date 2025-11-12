/// # Core Tile Definitions
/// 
/// This module defines the most basic components of a Mahjong tile.
pub mod tiles {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    /// Represents the three numbered suits.
    pub enum Suhai { // 数牌 (Numbered Tiles)
        Manzu, // 萬子 (Characters)
        Pinzu, // 筒子 (Circles)
        Souzu, // 索子 (Bamboo)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    /// Represents the four wind directions.
    pub enum Kaze { // 風牌 (Wind Tiles)
        Ton,   // 東 (East)
        Nan,   // 南 (South)
        Shaa,  // 西 (West)
        Pei,   // 北 (North)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    /// Represents the three dragons.
    pub enum Sangenpai { // 三元牌 (Dragon Tiles)
        Haku,  // 白 (White)
        Hatsu, // 發 (Green)
        Chun,  // 中 (Red)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    /// Represents any honor tile (Wind or Dragon).
    pub enum Jihai { // 字牌 (Honor Tiles)
        Kaze(Kaze),
        Sangen(Sangenpai),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    /// Represents a single Mahjong tile.
    pub enum Hai { // 牌 (Tile)
        Suhai(u8, Suhai), // 数牌 (Numbered tile, 1-9)
        Jihai(Jihai),      // 字牌 (Honor tile)
    }
}

/// # Hand Structure and Composition
/// 
/// This module defines how a hand is constructed, including melds, waits,
/// and the overall valid hand patterns.
pub mod hand {
    use super::tiles::Hai;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// Represents the type of a meld (group of tiles).
    pub enum MentsuType { /// 面子の種類 (Type of Meld)
        Shuntsu, // 順子 (Sequence)
        Koutsu,  // 刻子 (Triplet)
        Kantsu,  // 槓子 (Kan/Quad)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// Represents a single meld in the hand.
    pub struct Mentsu { /// 面子 (Meld)
        pub mentsu_type: MentsuType,
        pub is_minchou: bool, // 明張 (Is the meld open?)
        pub tiles: [Hai; 4], // Use 4 tiles; for Shuntsu/Koutsu, the 4th is unused.
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// Represents the type of wait for a winning hand.
    pub enum Machi { // 待ちの種類 (Type of Wait)
        Ryanmen, // 両面 (Two-Sided)
        Tanki,   // 単騎 (Pair wait)
        Penchan, // 辺張 (Edge wait, e.g., 1-2 waiting on 3)
        Kanchan, // 嵌張 (Closed wait, e.g., 4-6 waiting on 5)
        Shanpon, // 双碰 (Triplet-pair wait)
        
        // Special waits for Yakuman
        KokushiIchimen,      // 国士一面 (Kokushi single wait)
        KokushiJusanmen, // 国士十三面 (Kokushi 13-sided wait)
        Kyuumen,         // 九面 (Nine-sided wait for Nine Gates)
    }

    #[derive(Debug, Clone, Copy)]
    /// Represents a standard 4-meld, 1-pair winning hand.
    pub struct AgariHand { // 和了手 (Winning Hand)
        pub mentsu: [Mentsu; 4],  // 面子 (The 4 melds)
        pub atama: (Hai, Hai),   // 頭 (The pair)
        pub agari_hai: Hai,      // 和了牌 (The winning tile)
        pub machi: Machi,        // 待ち (The wait type)
    }

    /// Represents the two possible outcomes of the raw hand organizer.
    #[derive(Debug, Clone)]
    pub enum HandOrganization {
        /// Standard 4 melds, 1 pair. Ready for standard yaku checking.
        YonmentsuIchiatama(AgariHand), // 四面子一頭 (Standard Hand: 4 Melds, 1 Pair)
        /// An irregular hand (e.g., Chiitoitsu, Kokushi) or an invalid parse.
        /// The yaku checker will determine which, using the raw counts.
        Irregular { // 非標準手 (Irregular Hand)
            /// The raw 14-tile counts of the *entire* hand.
            counts: [u8; 34],
            /// The winning tile, needed for yaku checking.
            agari_hai: Hai,
        }
    }

    #[derive(Debug, Clone)]
    /// Represents the valid, recognized structure of a winning hand.
    /// (This is what the *yaku checker* will ultimately produce)
    pub enum HandStructure {
        /// 四面子一頭 (Standard 4 melds, 1 pair)
        YonmentsuIchiatama(AgariHand), 
        
        /// 七対子 (Seven Pairs)
        Chiitoitsu {
            pairs: [(Hai, Hai); 7],
            agari_hai: Hai,
            machi: Machi, // Will always be Machi::Tanki
        },
        
        /// 国士無双 (Thirteen Orphans)
        KokushiMusou {
            tiles: [Hai; 13], // The 13 unique tiles
            atama: (Hai, Hai),  // The pair
            agari_hai: Hai, // The winning tile
            machi: Machi, // KokushiIchimen (1-sided wait) or KokushiJusanmen (13-sided wait)
        },

        /// 九蓮宝燈 (Nine Gates)
        /// This structure is only noted for calculating the Junsei (True) wait.
        /// The hand is still passed as a YonmentsuIchiatama to the scorer,
        /// but this flag is used to award the double yakuman.
        ChuurenPoutou {
            hand: AgariHand,
            is_junsei: bool, // 純正 (Is it a true 9-sided wait?)
        }
    }
}

/// # Game State and Context
///
/// This module defines the context of the game when a hand is won,
/// which is necessary for calculating certain yaku (e.g., Yakuhai, Haitei).
pub mod game {
    use super::tiles::{Hai, Kaze};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// Represents how the hand was won.
    pub enum AgariType {
        Tsumo, // 自摸 (Self-draw)
        Ron,   // 栄和 (Win off discard)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// Context for the player winning the hand.
    pub struct PlayerContext {
        pub jikaze: Kaze,  // 自風 (Seat Wind)
        pub is_oya: bool,  // 親 (Is player the dealer?)
        pub is_riichi: bool, // 立直 (Was Riichi declared?)
        pub is_daburu_riichi: bool, // ダブル立直 (Double Riichi)
        pub is_ippatsu: bool,       // 一発 (Ippatsu)
        pub is_menzen: bool,      // 門前 (Is the hand fully concealed?)
    }

    #[derive(Debug, Clone)]
    /// Context for the current round of play.
    pub struct GameContext {
        pub bakaze: Kaze,             // 場風 (Prevalent Wind)
        pub kyoku: u8,                // 局 (Round number, e.g., 1 for East 1)
        pub honba: u8,                // 本場 (Honba counter)
        pub riichi_bou: u8,           // リーチ棒 (Riichi sticks on table)
        pub dora_indicators: Vec<Hai>,  // ドラ表示牌 (Dora indicators)
        pub uradora_indicators: Vec<Hai>, // 裏ドラ表示牌 (Ura Dora indicators)
        
        // Special win condition flags
        pub is_tenhou: bool,          // 天和 (Blessing of Heaven)
        pub is_chiihou: bool,         // 地和 (Blessing of Earth)
        pub is_renhou: bool,          // 人和 (Blessing of Man)
        pub is_haitei: bool,          // 海底 (Under the Sea - last draw)
        pub is_houtei: bool,          // 河底 (Under the River - last discard)
        pub is_rinshan: bool,         // 嶺上 (After a Kan)
        pub is_chankan: bool,         // 搶槓 (Robbing a Kan)
    }
}

/// # Yaku (Winning Hands)
///
/// This module defines all possible yaku, including Dora.
pub mod yaku { 
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// Represents a single Yaku (or bonus Dora) awarded to a hand.
    /// The han value is determined by the scoring logic, considering
    /// open/closed state.
    pub enum Yaku { // 役 (Yaku)
        // --- 1 Han Yaku ---
        Riichi,             // 立直 (Riichi)
        Ippatsu,            // 一発 (Ippatsu)
        MenzenTsumo,        // 門前清自摸和 (Fully Concealed Hand)
        Pinfu,              // 平和 (No-Points Hand)
        Iipeikou,           // 一盃口 (Pure Double Sequence)
        HaiteiRaoyue,       // 海底撈月 (Under the Sea)
        HouteiRaoyui,       // 河底撈魚 (Under the River)
        RinshanKaihou,      // 嶺上開花 (After a Kan)
        Chankan,            // 搶槓 (Robbing a Kan)
        Tanyao,             // 断幺九 (All Simples)
        YakuhaiJikaze,      // 役牌: 自風 (Seat Wind)
        YakuhaiBakaze,      // 役牌: 場風 (Prevalent Wind)
        YakuhaiSangenpai,   // 役牌: 三元牌 (Dragon)

        // --- 2 Han Yaku ---
        DaburuRiichi,       // ダブル立直 (Double Riichi)
        Chiitoitsu,         // 七対子 (Seven Pairs)
        SanshokuDoujun,     // 三色同順 (Mixed Triple Sequence) kuisagari
        Ittsu,              // 一気通貫 (Pure Straight) kuisagari
        Chanta,             // 全帯幺九 (Half Outside Hand) kuisagari
        Toitoi,             // 対々和 (All Triplets)
        Sanankou,           // 三暗刻 (Three Concealed Triplets)
        SanshokuDoukou,     // 三色同刻 (Triple Triplets)
        Sankantsu,          // 三槓子 (Three Quads)
        Shousangen,         // 小三元 (Little Three Dragons)
        Honroutou,          // 混老頭 (All Terminals and Honors)
        
        // --- 3 Han Yaku ---
        Ryanpeikou,         // 二盃口 (Twice Pure Double Sequence)
        Junchan,            // 純全帯么 (Fully Outside Hand) kuisagari
        Honitsu,            // 混一色 (Half Flush) kuisagari
        
        // --- 6 Han Yaku ---
        Chinitsu,           // 清一色 (Full Flush) kuisagari
        
        // --- Yakuman (13 Han) ---
        Tenhou,                 // 天和 (Blessing of Heaven)
        Chiihou,                // 地和 (Blessing of Earth)
        Renhou,                 // 人和 (Blessing of Man)
        Daisangen,              // 大三元 (Big Three Dragons)
        Suuankou,               // 四暗刻 (Four Concealed Triplets)
        Daisuushi,              // 大四喜 (Four Big Winds)
        Shousuushi,             // 小四喜 (Four Little Winds)
        Tsuuiisou,              // 字一色 (All Honors)
        Chinroutou,             // 清老頭 (All Terminals)
        Ryuuiisou,              // 緑一色 (All Green)
        Suukantsu,              // 四槓子 (Four Quads)
        KokushiMusou,           // 国士無双 (Thirteen Orphans)
        ChuurenPoutou,          // 九蓮宝燈 (Nine Gates)

        // --- Double Yakuman (26 Han) ---
        SuuankouTanki,          // 四暗刻単騎 (Single Wait Four Concealed)
        KokushiMusouJusanmen,   // 国士無S双13面待ち (13-Sided Wait Kokushi)
        JunseiChuurenPoutou,    // 純正九蓮宝燈 (True Nine Gates)
        
        // --- Dora (Bonus Han, not Yaku) ---
        Dora,                   // ドラ (Dora)
        UraDora,                // 裏ドラ (Ura Dora)
        AkaDora,                // 赤ドラ (Red Five Dora)
    }
}

/// # Scoring Results
///
/// This module defines the final output of a score calculation.
pub mod scoring {
    use super::yaku::Yaku;

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Represents the named point limits for high-scoring hands.
    pub enum HandLimit {
        Mangan,         // 満貫
        Haneman,        // 跳満
        Baiman,         // 倍満
        Sanbaiman,      // 三倍満
        Yakuman,        // 役満 (13han+)
        // Kazoe Yakuman (counted Yakuman) can be represented by 
        // han >= 13 and mapping to HandLimit::Yakuman.
    }

    #[derive(Debug, Clone)]
    /// Represents the complete scoring result for a winning hand.
    pub struct AgariResult {
        pub han: u8,      // 飜 (Han count)
        pub fu: u8,       // 符 (Fu count)
        pub yaku_list: Vec<Yaku>, // List of all yaku and dora achieved
        
        /// The named limit, if one is reached.
        pub limit_name: Option<HandLimit>,
        
        /// Base points. For ron, this is the total.
        /// For tsumo, this is the non-dealer payment.
        pub base_points: u32,
        
        // Payment fields are split for clarity in Tsumo calculations
        
        /// For non-dealer tsumo, this is the dealer's payment.
        /// For dealer tsumo, this is the (identical) payment from each non-dealer.
        pub oya_payment: u32,

        /// For non-dealer tsumo, this is the payment from other non-dealers.
        /// For dealer tsumo, this is 0.
        pub ko_payment: u32,

        /// Total points received by the winner, including honba.
        pub total_payment: u32,
    }
}