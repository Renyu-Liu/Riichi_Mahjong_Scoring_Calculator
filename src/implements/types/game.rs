use super::tiles::{Hai, Kaze};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// win type
pub enum AgariType {
    Tsumo, // 自摸 (Self-draw)
    Ron,   // 栄和 (Win off discard)
}

impl Default for AgariType {
    fn default() -> Self {
        AgariType::Ron
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Context winning hand
pub struct PlayerContext {
    pub jikaze: Kaze,           // 自風 (Seat Wind)
    pub is_oya: bool,           // 親 (dealer)
    pub is_riichi: bool,        // 立直 (Riichi)
    pub is_daburu_riichi: bool, // ダブル立直 (Double Riichi)
    pub is_ippatsu: bool,       // 一発 (Ippatsu)
    pub is_menzen: bool,        // 門前 (fully concealed)
}

#[derive(Debug, Clone)]
// Context current round
pub struct GameContext {
    pub bakaze: Kaze,                 // 場風 (Prevalent Wind)
    pub honba: u8,                    // 本場 (Honba counter)
    pub dora_indicators: Vec<Hai>,    // ドラ表示牌 (Dora indicators)
    pub uradora_indicators: Vec<Hai>, // 裏ドラ表示牌 (Ura Dora indicators)
    pub num_akadora: u8,              // 赤ドラ (Red Dora)
    // Special yaku flags
    pub is_tenhou: bool,  // 天和 (Blessing of Heaven)
    pub is_chiihou: bool, // 地和 (Blessing of Earth)
    pub is_renhou: bool,  // 人和 (Blessing of Man)
    pub is_haitei: bool,  // 海底 (last draw)
    pub is_houtei: bool,  // 河底 (last discard)
    pub is_rinshan: bool, // 嶺上 (After a Kan)
    pub is_chankan: bool, // 搶槓 (Robbing a Kan)
}
