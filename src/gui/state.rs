use crate::implements::game::AgariType;
use crate::implements::hand::MentsuType;
use crate::implements::input::OpenMeldInput;
use crate::implements::tiles::{Hai, Kaze};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phase {
    Composition,
    Definition,
    SelectingWinningTile,
    SelectingMeldTile(MentsuType, Option<usize>),
    SelectingClosedKan { editing_index: Option<usize> },
    SelectingDora,
    SelectingUraDora,
    Result,
}

pub struct RiichiGui {
    pub phase: Phase,

    // Phase 1 State
    pub hand_tiles: Vec<Hai>,
    pub tile_counts: [u8; 34],

    // Phase 2 State
    pub winning_tile: Option<Hai>,
    pub open_melds: Vec<OpenMeldInput>,
    pub closed_kans: Vec<Hai>,

    // Phase 3 State
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
}

impl RiichiGui {
    pub fn new() -> Self {
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

    pub fn count_five_tiles(&self) -> u8 {
        let mut count = 0;

        for tile in &self.hand_tiles {
            if matches!(tile, Hai::Suhai(5, _)) {
                count += 1;
            }
        }

        if let Some(tile) = &self.winning_tile {
            if matches!(tile, Hai::Suhai(5, _)) {
                count += 1;
            }
        }

        for meld in &self.open_melds {
            let tiles = self.get_meld_tiles(meld);
            for tile in tiles {
                if matches!(tile, Hai::Suhai(5, _)) {
                    count += 1;
                }
            }
        }

        for tile in &self.closed_kans {
            if matches!(tile, Hai::Suhai(5, _)) {
                count += 4;
            }
        }

        count
    }

    pub fn can_form_meld(&self, meld: &OpenMeldInput, editing_idx: Option<usize>) -> bool {
        let mut hand_counts = [0u8; 34];
        for tile in &self.hand_tiles {
            hand_counts[crate::implements::tiles::tile_to_index(tile)] += 1;
        }

        for (i, existing_meld) in self.open_melds.iter().enumerate() {
            if Some(i) == editing_idx {
                continue;
            }
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

    pub fn get_all_possible_pons(&self, editing_idx: Option<usize>) -> Vec<OpenMeldInput> {
        let mut available_counts = [0u8; 34];
        for tile in &self.hand_tiles {
            available_counts[crate::implements::tiles::tile_to_index(tile)] += 1;
        }

        for (i, existing_meld) in self.open_melds.iter().enumerate() {
            if Some(i) == editing_idx {
                continue;
            }
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

    pub fn get_all_possible_chiis(&self, editing_idx: Option<usize>) -> Vec<OpenMeldInput> {
        let mut available_counts = [0u8; 34];
        for tile in &self.hand_tiles {
            available_counts[crate::implements::tiles::tile_to_index(tile)] += 1;
        }

        for (i, existing_meld) in self.open_melds.iter().enumerate() {
            if Some(i) == editing_idx {
                continue;
            }
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
}
