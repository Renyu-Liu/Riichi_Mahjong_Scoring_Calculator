// types.rs: basic types and structs

pub mod tiles {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum Suhai {
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
    pub enum Hai {
        // 牌 (Tile)
        Suhai(u8, Suhai), // 数牌 (Number, 1-9)
        Jihai(Jihai),     // 字牌 (Honor)
    }

    impl Hai {
        // simple (2-8)
        pub fn is_simple(&self) -> bool {
            match self {
                Hai::Suhai(n, _) => *n >= 2 && *n <= 8,
                Hai::Jihai(_) => false,
            }
        }

        // terminal (1 or 9)
        pub fn is_terminal(&self) -> bool {
            match self {
                Hai::Suhai(n, _) => *n == 1 || *n == 9,
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
            Hai::Suhai(n, Suhai::Manzu) => (*n - 1) as usize, // 0-8
            Hai::Suhai(n, Suhai::Pinzu) => (*n - 1) as usize + 9, // 9-17
            Hai::Suhai(n, Suhai::Souzu) => (*n - 1) as usize + 18, // 18-26
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
            0..=8 => Hai::Suhai((index + 1) as u8, Suhai::Manzu),
            9..=17 => Hai::Suhai(((index - 9) + 1) as u8, Suhai::Pinzu),
            18..=26 => Hai::Suhai(((index - 18) + 1) as u8, Suhai::Souzu),
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
}

pub mod hand {
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
            // 非標準手 (Irregular Hand)
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
}

pub mod game {
    use super::tiles::{Hai, Kaze};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    // the way the hand was won
    pub enum AgariType {
        Tsumo, // 自摸 (Self-draw)
        Ron,   // 栄和 (Win off discard)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    // Context for winning the hand
    pub struct PlayerContext {
        pub jikaze: Kaze,           // 自風 (Seat Wind)
        pub is_oya: bool,           // 親 (dealer)
        pub is_riichi: bool,        // 立直 (Riichi)
        pub is_daburu_riichi: bool, // ダブル立直 (Double Riichi)
        pub is_ippatsu: bool,       // 一発 (Ippatsu)
        pub is_menzen: bool,        // 門前 (fully concealed)
    }

    #[derive(Debug, Clone)]
    // Context for the current round
    pub struct GameContext {
        pub bakaze: Kaze,                 // 場風 (Prevalent Wind)
        pub honba: u8,                    // 本場 (Honba counter)
        pub dora_indicators: Vec<Hai>,    // ドラ表示牌 (Dora indicators)
        pub uradora_indicators: Vec<Hai>, // 裏ドラ表示牌 (Ura Dora indicators)
        pub num_akadora: u8,              // 赤ドラ (Red Dora)
        // Special win condition flags
        pub is_tenhou: bool,  // 天和 (Blessing of Heaven)
        pub is_chiihou: bool, // 地和 (Blessing of Earth)
        pub is_renhou: bool,  // 人和 (Blessing of Man)
        pub is_haitei: bool,  // 海底 (last draw)
        pub is_houtei: bool,  // 河底 (last discard)
        pub is_rinshan: bool, // 嶺上 (After a Kan)
        pub is_chankan: bool, // 搶槓 (Robbing a Kan)
    }
}

pub mod yaku {
    use std::fmt;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Yaku {
        // 1 Han Yaku
        Riichi,           // 立直 (Riichi)
        Ippatsu,          // 一発 (Ippatsu)
        MenzenTsumo,      // 門前清自摸和 (Fully Concealed Hand)
        Pinfu,            // 平和 (No-Points Hand)
        Iipeikou,         // 一盃口 (Pure Double Sequence)
        HaiteiRaoyue,     // 海底撈月 (Under the Sea)
        HouteiRaoyui,     // 河底撈魚 (Under the River)
        RinshanKaihou,    // 嶺上開花 (After a Kan)
        Chankan,          // 搶槓 (Robbing a Kan)
        Tanyao,           // 断幺九 (All Simples)
        YakuhaiJikaze,    // 役牌: 自風 (Seat Wind)
        YakuhaiBakaze,    // 役牌: 場風 (Prevalent Wind)
        YakuhaiSangenpai, // 役牌: 三元牌 (Dragon)

        // 2 Han Yaku
        DaburuRiichi,   // ダブル立直 (Double Riichi)
        Chiitoitsu,     // 七対子 (Seven Pairs)
        SanshokuDoujun, // 三色同順 (Mixed Triple Sequence) kuisagari
        Ittsu,          // 一気通貫 (Pure Straight) kuisagari
        Chanta,         // 全帯幺九 (Half Outside Hand) kuisagari
        Toitoi,         // 対々和 (All Triplets)
        Sanankou,       // 三暗刻 (Three Concealed Triplets)
        SanshokuDoukou, // 三色同刻 (Triple Triplets)
        Sankantsu,      // 三槓子 (Three Quads)
        Shousangen,     // 小三元 (Little Three Dragons)
        Honroutou,      // 混老頭 (All Terminals and Honors)

        // 3 Han Yaku
        Ryanpeikou, // 二盃口 (Twice Pure Double Sequence)
        Junchan,    // 純全帯么 (Fully Outside Hand) kuisagari
        Honitsu,    // 混一色 (Half Flush) kuisagari

        // 6 Han Yaku
        Chinitsu, // 清一色 (Full Flush) kuisagari

        // Yakuman (13 Han)
        Tenhou,               // 天和 (Blessing of Heaven)
        Chiihou,              // 地和 (Blessing of Earth)
        Renhou,               // 人和 (Blessing of Man)
        Daisangen,            // 大三元 (Big Three Dragons)
        Suuankou,             // 四暗刻 (Four Concealed Triplets)
        Daisuushi,            // 大四喜 (Four Big Winds)
        Shousuushi,           // 小四喜 (Four Little Winds)
        Tsuuiisou,            // 字一色 (All Honors)
        Chinroutou,           // 清老頭 (All Terminals)
        Ryuuiisou,            // 緑一色 (All Green)
        Suukantsu,            // 四槓子 (Four Quads)
        KokushiMusou,         // 国士無双 (Thirteen Orphans)
        ChuurenPoutou,        // 九蓮宝燈 (Nine Gates)
        SuuankouTanki,        // 四暗刻単騎 (Single Wait Four Concealed)
        KokushiMusouJusanmen, // 国士無S双13面待ち (13-Sided Wait Kokushi)
        JunseiChuurenPoutou,  // 純正九蓮宝燈 (True Nine Gates)

        // Dora (not Yaku)
        Dora,    // ドラ (Dora)
        UraDora, // 裏ドラ (Ura Dora)
        AkaDora, // 赤ドラ (Red Five Dora)
    }

    // terminal version of Display for Yaku. Not used in GUI
    impl fmt::Display for Yaku {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}

pub mod scoring {
    use super::game::AgariType;
    use super::yaku::Yaku;
    use std::fmt;

    #[derive(Debug, Clone, PartialEq, Eq)]
    // named point limits
    pub enum HandLimit {
        Mangan,    // 満貫
        Haneman,   // 跳満
        Baiman,    // 倍満
        Sanbaiman, // 三倍満
        Yakuman,   // 役満 (13han+)
    }

    // terminal version of Display for HandLimit. Not used in GUI
    impl fmt::Display for HandLimit {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Debug, Clone)]
    // complete scoring result for a winning hand
    pub struct AgariResult {
        pub han: u8,              // 飜 (Han count)
        pub fu: u8,               // 符 (Fu count)
        pub yaku_list: Vec<Yaku>, // all yaku and dora achieved
        pub num_akadora: u8,      // 赤ドラ (Red Dora)
        pub limit_name: Option<HandLimit>,
        pub oya_payment: u32,
        pub ko_payment: u32,
        pub total_payment: u32,
        pub honba: u8,
        pub agari_type: AgariType,
        pub is_oya: bool,
    }

    // terminal version of Display for AgariResult. No longer used
    impl fmt::Display for AgariResult {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "========== Score Result ==========")?;

            // 1. Total Score
            writeln!(f, "Total Payment: {}", self.total_payment)?;

            // 2. Han / Fu & 4. Limit Name
            if let Some(limit) = &self.limit_name {
                if limit == &HandLimit::Yakuman {
                    let num_yakuman = self.han / 13;
                    if num_yakuman > 1 {
                        writeln!(f, "\n{} x{}", limit, num_yakuman)?;
                    } else {
                        writeln!(f, "\n{}", limit)?;
                    }
                } else {
                    writeln!(f, "\n{} ({} Fu, {} Han)", limit, self.fu, self.han)?;
                }
            } else {
                writeln!(f, "\n{} Fu, {} Han", self.fu, self.han)?;
            }

            // 3. Yaku List
            writeln!(f, "\n--- Yaku List ---")?;
            let mut dora_count = 0;
            let mut uradora_count = 0;
            let mut akadora_count = 0;

            if self.yaku_list.is_empty() {
                writeln!(f, "(No yaku)")?;
            }

            for yaku in &self.yaku_list {
                match yaku {
                    Yaku::Dora => dora_count += 1,
                    Yaku::UraDora => uradora_count += 1,
                    Yaku::AkaDora => akadora_count += 1,
                    _ => writeln!(f, "- {}", yaku)?,
                }
            }
            if dora_count > 0 {
                writeln!(f, "- Dora {}", dora_count)?;
            }
            if uradora_count > 0 {
                writeln!(f, "- UraDora {}", uradora_count)?;
            }
            if akadora_count > 0 {
                writeln!(f, "- AkaDora {}", akadora_count)?;
            }

            // 5. Payment Breakdown
            writeln!(f, "\n--- Payment Breakdown ---")?;
            let tsumo_bonus = self.honba as u32 * 100;
            let ron_bonus = self.honba as u32 * 300;

            match (self.is_oya, self.agari_type) {
                // Oya Tsumo
                (true, AgariType::Tsumo) => {
                    let payment_per_player = self.oya_payment + tsumo_bonus;
                    writeln!(f, "Oya Tsumo win:")?;
                    writeln!(
                        f,
                        "  Each Non-Dealer pays: {} ({} base + {} honba)",
                        payment_per_player, self.oya_payment, tsumo_bonus
                    )?;
                }
                // Ko Tsumo
                (false, AgariType::Tsumo) => {
                    let oya_pay = self.oya_payment + tsumo_bonus;
                    let ko_pay = self.ko_payment + tsumo_bonus;
                    writeln!(f, "Ko Tsumo win:")?;
                    writeln!(
                        f,
                        "  Oya (Dealer) pays: {} ({} base + {} honba)",
                        oya_pay, self.oya_payment, tsumo_bonus
                    )?;
                    writeln!(
                        f,
                        "  Other Non-Dealers pay: {} ({} base + {} honba)",
                        ko_pay, self.ko_payment, tsumo_bonus
                    )?;
                }
                // Oya Ron
                (true, AgariType::Ron) => {
                    let base_payment = self.total_payment - ron_bonus;
                    writeln!(f, "Oya Ron win:")?;
                    writeln!(
                        f,
                        "  Discarder pays: {} ({} base + {} honba)",
                        self.total_payment, base_payment, ron_bonus
                    )?;
                }
                // Ko Ron
                (false, AgariType::Ron) => {
                    let base_payment = self.total_payment - ron_bonus;
                    writeln!(f, "Ko Ron win:")?;
                    writeln!(
                        f,
                        "  Discarder pays: {} ({} base + {} honba)",
                        self.total_payment, base_payment, ron_bonus
                    )?;
                }
            }

            writeln!(f, "==================================")?;
            Ok(())
        }
    }
}

pub mod input {
    use super::game::{AgariType, GameContext, PlayerContext};
    use super::hand::MentsuType;
    use super::tiles::Hai;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct OpenMeldInput {
        // type of meld
        pub mentsu_type: MentsuType,

        // single tile that uniquely identifies the meld
        pub representative_tile: Hai,
    }

    #[derive(Debug, Clone)]
    pub struct UserInput {
        // Hand Composition
        pub hand_tiles: Vec<Hai>,

        pub winning_tile: Hai,

        // list of all open melds
        pub open_melds: Vec<OpenMeldInput>,

        // list of all closed kans
        pub closed_kans: Vec<Hai>,

        pub player_context: PlayerContext,
        pub game_context: GameContext,
        pub agari_type: AgariType,
    }
}
