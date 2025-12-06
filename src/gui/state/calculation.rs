use super::RiichiGui;
use crate::implements::types::tiles::{Hai, Suhai};

impl RiichiGui {
    pub fn get_max_akadora_count(&self) -> u8 {
        let mut count_5m = 0;
        let mut count_5p = 0;
        let mut count_5s = 0;

        let check_tile = |tile: &Hai, c_m: &mut u8, c_p: &mut u8, c_s: &mut u8| {
            if let Hai::Suhai(Suhai { number: 5, suit }) = tile {
                match suit {
                    crate::implements::types::tiles::Suit::Manzu => *c_m += 1,
                    crate::implements::types::tiles::Suit::Pinzu => *c_p += 1,
                    crate::implements::types::tiles::Suit::Souzu => *c_s += 1,
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
}
