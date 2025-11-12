// Import all the new modular types
use super::types::{
    tiles::{Hai, Jihai, Kaze, Sangenpai, Suhai},
    hand::{AgariHand, HandOrganization, Machi, Mentsu, MentsuType},
};
// Used for converting Vec<Mentsu> to [Mentsu; 4]
use std::convert::TryInto;


// === Private Helper Functions ===
pub mod helpers {
    use super::*;

    /// Converts a Hai enum to its corresponding index (0-33).
    pub fn tile_to_index(tile: &Hai) -> usize {
        match tile {
            Hai::Suhai(n, Suhai::Manzu) => (*n - 1) as usize,       // 0-8
            Hai::Suhai(n, Suhai::Pinzu) => (*n - 1) as usize + 9,  // 9-17
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

    /// Converts an index (0-33) back into a Hai.
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

// === Recursive Parsing Logic ===
mod recursive_parser {
    use super::*;

    /// Recursively finds melds from a tile-count array.
    pub fn find_mentsu_recursive(counts: &mut [u8; 34], mentsu: &mut Vec<Mentsu>) -> bool {
        let mut i = 0;
        while i < 34 && counts[i] == 0 {
            i += 1;
        }
        if i == 34 { return true; } // Success: all tiles used up

        // --- Try to form a Triplet (Koutsu) ---
        if counts[i] >= 3 {
            let tile = helpers::index_to_tile(i);
            counts[i] -= 3;
            mentsu.push(Mentsu {
                mentsu_type: MentsuType::Koutsu,
                is_minchou: false, // is_open
                tiles: [tile, tile, tile, tile], // 4th tile is ignored
            });

            if find_mentsu_recursive(counts, mentsu) { return true; }

            // Backtrack
            mentsu.pop();
            counts[i] += 3;
        }

        // --- Try to form a Sequence (Shuntsu) ---
        // i < 27 checks that it's not an honor tile
        // (i % 9) < 7 checks that it's not 8 or 9 (can't start a sequence)
        if i < 27 && (i % 9) < 7 && counts[i] > 0 && counts[i + 1] > 0 && counts[i + 2] > 0 {
            let tile1 = helpers::index_to_tile(i);
            let tile2 = helpers::index_to_tile(i + 1);
            let tile3 = helpers::index_to_tile(i + 2);

            counts[i] -= 1;
            counts[i + 1] -= 1;
            counts[i + 2] -= 1;
            mentsu.push(Mentsu {
                mentsu_type: MentsuType::Shuntsu,
                is_minchou: false,
                // Store sorted, 4th is ignored (using tile3 as placeholder)
                tiles: [tile1, tile2, tile3, tile3], 
            });

            if find_mentsu_recursive(counts, mentsu) { return true; }

            // Backtrack
            mentsu.pop();
            counts[i] += 1;
            counts[i + 1] += 1;
            counts[i + 2] += 1;
        }
        
        // If neither Koutsu nor Shuntsu could be formed from this tile, this branch fails
        false
    }
}

// === Wait Type Analysis Logic ===
mod wait_analyzer {
    use super::*;

    /// Checks if a meld contains a specific tile.
    fn mentsu_contains_tile(mentsu: &Mentsu, tile: &Hai) -> bool {
        match mentsu.mentsu_type {
            MentsuType::Koutsu | MentsuType::Kantsu => mentsu.tiles[0] == *tile,
            MentsuType::Shuntsu => {
                mentsu.tiles[0] == *tile || mentsu.tiles[1] == *tile || mentsu.tiles[2] == *tile
            }
        }
    }

    /// Analyzes the completed hand to determine the wait type.
    pub fn determine_wait_type(
        mentsu: &[Mentsu; 4],
        atama: (Hai, Hai), // pair
        agari_hai: Hai,    // winning_tile
    ) -> Machi { // WaitType
        // 1. Check for Pair Wait (Tanki)
        if agari_hai == atama.0 {
            return Machi::Tanki;
        }

        // Find the meld that the winning tile completes
        let winning_meld = mentsu
            .iter()
            .find(|m| mentsu_contains_tile(m, &agari_hai))
            .expect("Winning tile not in pair or melds. Invalid hand.");

        match winning_meld.mentsu_type {
            // 2. Check for Shanpon wait
            // If the winning tile forms a Koutsu, it must have been a Shanpon wait.
            MentsuType::Koutsu | MentsuType::Kantsu => Machi::Shanpon,
            
            // 3. Check for Ryanmen, Kanchan, or Penchan
            MentsuType::Shuntsu => {
                let t1 = winning_meld.tiles[0];
                let t2 = winning_meld.tiles[1];
                let t3 = winning_meld.tiles[2];
                
                if agari_hai == t2 {
                    // e.g., Hand had 4-6, won on 5. This is Kanchan.
                    Machi::Kanchan
                } else if agari_hai == t1 {
                    // e.g., Hand had t2, t3 (like 3-4, won on 2). This is Ryanmen.
                    // Special case: Hand had 8-9, won on 7. This is Penchan.
                    if helpers::tile_to_index(&t3) % 9 == 8 {
                        Machi::Penchan
                    } else {
                        Machi::Ryanmen
                    }
                } else if agari_hai == t3 {
                    // e.g., Hand had t1, t2 (like 2-3, won on 4). This is Ryanmen.
                    // Special case: Hand had 1-2, won on 3. This is Penchan.
                    if helpers::tile_to_index(&t1) % 9 == 0 {
                        Machi::Penchan
                    } else {
                        Machi::Ryanmen
                    }
                } else {
                    unreachable!("Winning tile in sequence but not t1, t2, or t3");
                }
            }
        }
    }
}


// === Public Function ===

/// Organizes a raw hand into a standard 4-meld, 1-pair structure
/// or flags it as irregular for special yaku checking (Chiitoitsu, Kokushi).
/// 
/// # Arguments
/// * `all_tiles` - A slice containing all 14 tiles of the winning hand.
/// * `agari_hai` - The single winning tile.
/// * `open_mentsu` - A slice of any open melds (pon, chii, kan).
pub fn organize_hand(
    all_tiles: &[Hai],
    agari_hai: Hai,
    open_mentsu: &[Mentsu],
) -> Result<HandOrganization, &'static str> {
    
    // We will add all melds (open and closed) to this Vec
    let mut final_mentsu = open_mentsu.to_vec();
    
    // 1. Create tile counts from ALL 14 tiles
    let mut counts = [0u8; 34];
    for tile in all_tiles {
        counts[helpers::tile_to_index(tile)] += 1;
    }

    // 2. Subtract tiles from open melds to find the remaining *closed* hand.
    for mentsu in open_mentsu {
        match mentsu.mentsu_type {
            MentsuType::Shuntsu | MentsuType::Koutsu => {
                counts[helpers::tile_to_index(&mentsu.tiles[0])] -= 1;
                counts[helpers::tile_to_index(&mentsu.tiles[1])] -= 1;
                counts[helpers::tile_to_index(&mentsu.tiles[2])] -= 1;
            }
            MentsuType::Kantsu => {
                // A Kan uses 4 tiles
                counts[helpers::tile_to_index(&mentsu.tiles[0])] -= 1;
                counts[helpers::tile_to_index(&mentsu.tiles[1])] -= 1;
                counts[helpers::tile_to_index(&mentsu.tiles[2])] -= 1;
                counts[helpers::tile_to_index(&mentsu.tiles[3])] -= 1;
            }
        }
    }

    // 3. Determine how many closed melds we still need to find
    let mentsu_needed = 4 - final_mentsu.len();
    
    // --- Case A: 4 open melds (e.g., Hadaka Tanki / Naked Wait) ---
    if mentsu_needed == 0 {
        // The remaining tiles in `counts` must be the pair
        for i in 0..34 {
            if counts[i] == 2 {
                let pair_tile = helpers::index_to_tile(i);
                let atama = (pair_tile, pair_tile);
                
                // Convert Vec<Mentsu> to [Mentsu; 4]
                let mentsu_array: [Mentsu; 4] = final_mentsu.try_into()
                    .expect("Hand parsing logic error: final_mentsu length not 4");
                
                let agari_hand = AgariHand {
                    mentsu: mentsu_array,
                    atama,
                    agari_hai,
                    machi: Machi::Tanki, // Must be a pair wait
                };

                return Ok(HandOrganization::YonmentsuIchiatama(agari_hand));
            }
        }
        return Err("Invalid hand: 4 open melds but no pair found.");
    }

    // --- Case B: 0-3 open melds (Standard Hand Check) ---
    // Try to find a 4-meld, 1-pair hand by iterating through all possible pairs.
    for i in 0..34 {
        if counts[i] >= 2 {
            // Assume this tile `i` is the pair
            let mut temp_counts = counts; // Copy the closed hand counts
            temp_counts[i] -= 2;
            let atama = (helpers::index_to_tile(i), helpers::index_to_tile(i));
            let mut closed_mentsu: Vec<Mentsu> = Vec::with_capacity(mentsu_needed);

            // 3. Try to find the remaining melds recursively
            if recursive_parser::find_mentsu_recursive(&mut temp_counts, &mut closed_mentsu) {
                // If we found the exact number of melds needed, we have a valid hand
                if closed_mentsu.len() == mentsu_needed {
                    // Success!
                    final_mentsu.append(&mut closed_mentsu);
                    
                    // Convert Vec<Mentsu> to [Mentsu; 4]
                    let mentsu_array: [Mentsu; 4] = final_mentsu.try_into()
                        .expect("Hand parsing logic error: final_mentsu length not 4");

                    let machi = wait_analyzer::determine_wait_type(
                        &mentsu_array,
                        atama,
                        agari_hai
                    );

                    let agari_hand = AgariHand {
                        mentsu: mentsu_array,
                        atama,
                        agari_hai,
                        machi,
                    };
                    
                    return Ok(HandOrganization::YonmentsuIchiatama(agari_hand));
                }
            }
            // If recursion failed or didn't find the right number of melds,
            // we loop and try the next tile as the pair.
        }
    }

    // --- FAILURE ---
    // If we are here, the 4-meld-1-pair parse failed.
    // This means the hand is either irregular (Chiitoitsu, Kokushi) or invalid.
    
    // We must return the *original* 14-tile counts for the yaku checker.
    // The `counts` variable is already mutated, so we re-create it.
    let mut original_counts = [0u8; 34];
    for tile in all_tiles {
        original_counts[helpers::tile_to_index(tile)] += 1;
    }
        
    // Return the irregular structure for the next module to check.
    Ok(HandOrganization::Irregular {
        counts: original_counts,
        agari_hai,
    })

}