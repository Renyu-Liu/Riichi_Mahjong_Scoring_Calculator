pub mod calculation;
pub mod initialize;
pub mod melds;
pub mod phase;

use crate::implements::types::{
    game::AgariType,
    input::OpenMeldInput,
    tiles::{Hai, Kaze},
};
pub use phase::Phase;

pub struct RiichiGui {
    pub phase: Phase,

    // --- Composition Phase ---
    pub hand_tiles: Vec<Hai>,
    pub tile_counts: [u8; 34],

    // --- Definition Phase ---
    pub winning_tile: Option<Hai>,
    pub open_melds: Vec<OpenMeldInput>,
    pub closed_kans: Vec<Hai>,

    // --- Result Phase ---
    pub agari_type: AgariType,
    pub bakaze: Kaze,
    pub jikaze: Kaze,
    pub is_riichi: bool,
    pub is_daburu_riichi: bool,
    pub is_ippatsu: bool,
    pub is_rinshan: bool,
    pub is_chankan: bool,
    pub is_haitei: bool,
    pub is_houtei: bool,
    pub is_tenhou: bool,
    pub is_chiihou: bool,
    pub is_renhou: bool,
    pub honba: u8,
    pub num_akadora: u8,
    pub dora_indicators: Vec<Hai>,
    pub uradora_indicators: Vec<Hai>,
    pub score_result: Option<Result<crate::implements::types::scoring::AgariResult, String>>,
    pub show_rules: bool,
    pub tile_images: std::collections::HashMap<Hai, iced::widget::image::Handle>,
    pub rules_image: Option<iced::widget::image::Handle>,
}
