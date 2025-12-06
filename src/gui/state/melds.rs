use super::RiichiGui;
use crate::implements::types::{hand::MentsuType, input::OpenMeldInput, tiles::Hai};

impl RiichiGui {
    pub fn get_meld_tiles(&self, meld: &OpenMeldInput) -> Vec<Hai> {
        let mut tiles = Vec::new();
        match meld.mentsu_type {
            MentsuType::Shuntsu => {
                let start_idx =
                    crate::implements::types::tiles::tile_to_index(&meld.representative_tile);
                if start_idx < 27 {
                    let suit_base = (start_idx / 9) * 9;

                    for i in 0..3 {
                        let idx = start_idx + i;
                        if idx < suit_base + 9 {
                            tiles.push(crate::implements::types::tiles::index_to_tile(idx));
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

    /// Checks for meld possibility
    pub fn can_form_meld(&self, meld: &OpenMeldInput) -> bool {
        let mut hand_counts = [0u8; 34];
        for tile in &self.hand_tiles {
            hand_counts[crate::implements::types::tiles::tile_to_index(tile)] += 1;
        }

        for existing_meld in &self.open_melds {
            for tile in self.get_meld_tiles(existing_meld) {
                let idx = crate::implements::types::tiles::tile_to_index(&tile);
                if hand_counts[idx] > 0 {
                    hand_counts[idx] -= 1;
                } else {
                    return false;
                }
            }
        }

        for tile in self.get_meld_tiles(meld) {
            let idx = crate::implements::types::tiles::tile_to_index(&tile);
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
            available_counts[crate::implements::types::tiles::tile_to_index(tile)] += 1;
        }

        for existing_meld in &self.open_melds {
            for tile in self.get_meld_tiles(existing_meld) {
                let idx = crate::implements::types::tiles::tile_to_index(&tile);
                if available_counts[idx] > 0 {
                    available_counts[idx] -= 1;
                }
            }
        }

        let mut pons = Vec::new();
        for i in 0..34 {
            if available_counts[i] >= 3 {
                let tile = crate::implements::types::tiles::index_to_tile(i);
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
            available_counts[crate::implements::types::tiles::tile_to_index(tile)] += 1;
        }

        for existing_meld in &self.open_melds {
            for tile in self.get_meld_tiles(existing_meld) {
                let idx = crate::implements::types::tiles::tile_to_index(&tile);
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
                    let tile = crate::implements::types::tiles::index_to_tile(idx1);
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
            available_counts[crate::implements::types::tiles::tile_to_index(tile)] += 1;
        }

        let mut kans = Vec::new();
        for i in 0..34 {
            if available_counts[i] == 4 {
                let tile = crate::implements::types::tiles::index_to_tile(i);
                kans.push(tile);
            }
        }
        kans
    }
}
