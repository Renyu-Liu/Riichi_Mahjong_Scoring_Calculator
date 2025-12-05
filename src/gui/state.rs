use crate::implements::game::AgariType;
use crate::implements::hand::MentsuType;
use crate::implements::input::OpenMeldInput;
use crate::implements::tiles::{Hai, Kaze, Suhai};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phase {
    Composition,
    Definition,
    SelectingWinningTile,
    SelectingMeldTile(MentsuType),
    SelectingClosedKan,
    SelectingDora,
    SelectingUraDora,
    Result,
}

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
    pub score_result: Option<Result<crate::implements::scoring::AgariResult, String>>,
    pub show_rules: bool,
    pub tile_images: std::collections::HashMap<Hai, iced::widget::image::Handle>,
    pub rules_image: Option<iced::widget::image::Handle>,
}

impl RiichiGui {
    pub fn new() -> Self {
        let mut tile_images: std::collections::HashMap<
            crate::implements::tiles::Hai,
            iced::widget::image::Handle,
        > = std::collections::HashMap::new();
        for i in 0..34 {
            let tile = crate::implements::tiles::index_to_tile(i);
            let path = crate::gui::components::get_tile_image_path(&tile);

            // fast rendering
            if let Ok(img) = image::open(&path) {
                let resized = img.resize(256, 256, image::imageops::FilterType::Nearest);
                let rgba = resized.to_rgba8();
                let width = rgba.width();
                let height = rgba.height();
                let pixels = rgba.into_raw();

                let handle = iced::widget::image::Handle::from_pixels(width, height, pixels);
                tile_images.insert(tile, handle);
            } else {
                eprintln!("Failed to load image: {}", path);
            }
        }

        let rules_image = if let Ok(img) = image::open("assets/riichi_rule.png") {
            let rgba = img.to_rgba8();
            let width = rgba.width();
            let height = rgba.height();
            let pixels = rgba.into_raw();
            Some(iced::widget::image::Handle::from_pixels(
                width, height, pixels,
            ))
        } else {
            eprintln!("Failed to load rules image");
            None
        };

        Self {
            phase: Phase::Composition,
            hand_tiles: Vec::new(),
            tile_counts: [4; 34],
            winning_tile: None,
            open_melds: Vec::new(),
            closed_kans: Vec::new(),
            agari_type: AgariType::Ron,
            bakaze: Kaze::Ton,
            jikaze: Kaze::Ton,
            is_riichi: false,
            is_daburu_riichi: false,
            is_ippatsu: false,
            is_rinshan: false,
            is_chankan: false,
            is_haitei: false,
            is_houtei: false,
            is_tenhou: false,
            is_chiihou: false,
            is_renhou: false,
            honba: 0,
            num_akadora: 0,
            dora_indicators: Vec::new(),
            uradora_indicators: Vec::new(),
            score_result: None,
            show_rules: false,
            tile_images,
            rules_image,
        }
    }

    pub fn get_meld_tiles(&self, meld: &OpenMeldInput) -> Vec<Hai> {
        let mut tiles = Vec::new();
        match meld.mentsu_type {
            MentsuType::Shuntsu => {
                let start_idx = crate::implements::tiles::tile_to_index(&meld.representative_tile);
                if start_idx < 27 {
                    let suit_base = (start_idx / 9) * 9;

                    for i in 0..3 {
                        let idx = start_idx + i;
                        if idx < suit_base + 9 {
                            tiles.push(crate::implements::tiles::index_to_tile(idx));
                        }
                    }
                } else {
                    tiles.push(meld.representative_tile);
                }
            }
            MentsuType::Koutsu => {
                for _ in 0..3 {
                    tiles.push(meld.representative_tile);
                }
            }
            MentsuType::Kantsu => {
                for _ in 0..4 {
                    tiles.push(meld.representative_tile);
                }
            }
        }
        tiles
    }

    pub fn get_max_akadora_count(&self) -> u8 {
        let mut count_5m = 0;
        let mut count_5p = 0;
        let mut count_5s = 0;

        let check_tile = |tile: &Hai, c_m: &mut u8, c_p: &mut u8, c_s: &mut u8| {
            if let Hai::Suhai(Suhai { number: 5, suit }) = tile {
                match suit {
                    crate::implements::tiles::Suit::Manzu => *c_m += 1,
                    crate::implements::tiles::Suit::Pinzu => *c_p += 1,
                    crate::implements::tiles::Suit::Souzu => *c_s += 1,
                }
            }
        };

        for tile in &self.hand_tiles {
            check_tile(tile, &mut count_5m, &mut count_5p, &mut count_5s);
        }

        if let Some(tile) = &self.winning_tile {
            check_tile(tile, &mut count_5m, &mut count_5p, &mut count_5s);
        }

        for meld in &self.open_melds {
            for tile in self.get_meld_tiles(meld) {
                check_tile(&tile, &mut count_5m, &mut count_5p, &mut count_5s);
            }
        }

        for tile in &self.closed_kans {
            for _ in 0..4 {
                check_tile(tile, &mut count_5m, &mut count_5p, &mut count_5s);
            }
        }

        // 1 red 5-man, 2 red 5-pin, 1 red 5-sou
        let max_m = if count_5m > 0 { 1 } else { 0 };
        let max_p = if count_5p >= 2 { 2 } else { count_5p };
        let max_s = if count_5s > 0 { 1 } else { 0 };

        max_m + max_p + max_s
    }

    /// Checks for meld
    pub fn can_form_meld(&self, meld: &OpenMeldInput) -> bool {
        let mut hand_counts = [0u8; 34];
        for tile in &self.hand_tiles {
            hand_counts[crate::implements::tiles::tile_to_index(tile)] += 1;
        }

        for existing_meld in &self.open_melds {
            for tile in self.get_meld_tiles(existing_meld) {
                let idx = crate::implements::tiles::tile_to_index(&tile);
                if hand_counts[idx] > 0 {
                    hand_counts[idx] -= 1;
                } else {
                    return false;
                }
            }
        }

        for tile in self.get_meld_tiles(meld) {
            let idx = crate::implements::tiles::tile_to_index(&tile);
            if hand_counts[idx] > 0 {
                hand_counts[idx] -= 1;
            } else {
                return false;
            }
        }

        true
    }

    pub fn get_all_possible_pons(&self) -> Vec<OpenMeldInput> {
        let mut available_counts = [0u8; 34];
        for tile in &self.hand_tiles {
            available_counts[crate::implements::tiles::tile_to_index(tile)] += 1;
        }

        for existing_meld in &self.open_melds {
            for tile in self.get_meld_tiles(existing_meld) {
                let idx = crate::implements::tiles::tile_to_index(&tile);
                if available_counts[idx] > 0 {
                    available_counts[idx] -= 1;
                }
            }
        }

        let mut pons = Vec::new();
        for i in 0..34 {
            if available_counts[i] >= 3 {
                let tile = crate::implements::tiles::index_to_tile(i);
                pons.push(OpenMeldInput {
                    mentsu_type: MentsuType::Koutsu,
                    representative_tile: tile,
                });
            }
        }
        pons
    }

    pub fn get_all_possible_chiis(&self) -> Vec<OpenMeldInput> {
        let mut available_counts = [0u8; 34];
        for tile in &self.hand_tiles {
            available_counts[crate::implements::tiles::tile_to_index(tile)] += 1;
        }

        for existing_meld in &self.open_melds {
            for tile in self.get_meld_tiles(existing_meld) {
                let idx = crate::implements::tiles::tile_to_index(&tile);
                if available_counts[idx] > 0 {
                    available_counts[idx] -= 1;
                }
            }
        }

        let mut chiis = Vec::new();
        for suit_offset in [0, 9, 18] {
            for start_num in 0..7 {
                let idx1 = suit_offset + start_num;
                let idx2 = idx1 + 1;
                let idx3 = idx1 + 2;

                if available_counts[idx1] > 0
                    && available_counts[idx2] > 0
                    && available_counts[idx3] > 0
                {
                    let tile = crate::implements::tiles::index_to_tile(idx1);
                    chiis.push(OpenMeldInput {
                        mentsu_type: MentsuType::Shuntsu,
                        representative_tile: tile,
                    });
                }
            }
        }
        chiis
    }

    pub fn get_all_possible_kans(&self) -> Vec<Hai> {
        let mut available_counts = [0u8; 34];
        for tile in &self.hand_tiles {
            available_counts[crate::implements::tiles::tile_to_index(tile)] += 1;
        }

        let mut kans = Vec::new();
        for i in 0..34 {
            if available_counts[i] == 4 {
                let tile = crate::implements::tiles::index_to_tile(i);
                kans.push(tile);
            }
        }
        kans
    }

    pub fn reset(&mut self) {
        self.phase = Phase::Composition;
        self.hand_tiles.clear();
        self.tile_counts = [4; 34];
        self.winning_tile = None;
        self.open_melds.clear();
        self.closed_kans.clear();
        self.agari_type = AgariType::Ron;
        self.bakaze = Kaze::Ton;
        self.jikaze = Kaze::Ton;
        self.is_riichi = false;
        self.is_daburu_riichi = false;
        self.is_ippatsu = false;
        self.is_rinshan = false;
        self.is_chankan = false;
        self.is_haitei = false;
        self.is_houtei = false;
        self.is_tenhou = false;
        self.is_chiihou = false;
        self.is_renhou = false;
        self.honba = 0;
        self.num_akadora = 0;
        self.dora_indicators.clear();
        self.uradora_indicators.clear();
        self.score_result = None;
        self.show_rules = false;
    }
}
