// raw_hand_organizer.rs: Organizes a raw hand input into standard melds and pair

use super::types::{
    game::{AgariType, GameContext, PlayerContext},
    hand::{AgariHand, HandOrganization, Machi, Mentsu, MentsuType},
    // Import centralized helper functions
    tiles::{index_to_tile, tile_to_index, Hai, Suhai},
    input::{UserInput},
};
use std::convert::TryInto;

// Input Validation Module
mod input_validator {
    use super::*;

    /// logical conflicts
    fn validate_game_state(
        p: &PlayerContext,
        g: &GameContext,
        a: AgariType,
        input: &UserInput,
    ) -> Result<(), &'static str> {
        // Riichi conflicts
        if p.is_daburu_riichi && p.is_riichi {
            return Err("Invalid state: Cannot be both Riichi and Daburu Riichi.");
        }
        if p.is_ippatsu && !(p.is_riichi || p.is_daburu_riichi) {
            return Err("Invalid state: Ippatsu requires Riichi or Daburu Riichi.");
        }

        // Menzen (Concealed) conflicts
        if p.is_menzen && !input.open_melds.is_empty() {
            return Err("Invalid state: Hand is declared menzen but has open melds.");
        }

        // Tsumo/Ron conflicts
        if g.is_haitei && a == AgariType::Ron {
            return Err("Invalid state: Haitei (last draw) cannot be a Ron win.");
        }
        if g.is_houtei && a == AgariType::Tsumo {
            return Err("Invalid state: Houtei (last discard) cannot be a Tsumo win.");
        }
        if g.is_haitei && g.is_houtei {
            return Err("Invalid state: Cannot be both Haitei and Houtei.");
        }
        if g.is_rinshan && a == AgariType::Ron {
            return Err("Invalid state: Rinshan (kan draw) cannot be a Ron win.");
        }
        if g.is_chankan && a == AgariType::Tsumo {
            return Err("Invalid state: Chankan (robbing kan) cannot be a Tsumo win.");
        }

        // Yakuman state conflicts
        if g.is_tenhou {
            if !p.is_oya {
                return Err("Invalid state: Tenhou requires player to be Oya (dealer).");
            }
            if a != AgariType::Tsumo {
                return Err("Invalid state: Tenhou must be a Tsumo win.");
            }
            if !input.open_melds.is_empty() || !input.closed_kans.is_empty() {
                return Err("Invalid state: Tenhou cannot have any calls (no open melds or kans).");
            }
        }
        if g.is_chiihou {
            if p.is_oya {
                return Err("Invalid state: Chiihou requires player to be non-Oya.");
            }
            if a != AgariType::Tsumo {
                return Err("Invalid state: Chiihou must be a Tsumo win.");
            }
            if !input.open_melds.is_empty() || !input.closed_kans.is_empty() {
                return Err("Invalid state: Chiihou cannot have any calls (no open melds or kans).");
            }
        }
        if g.is_renhou && a != AgariType::Ron {
            return Err("Invalid state: Renhou must be a Ron win.");
        }

        Ok(())
    }

    /// invalid hand composition 
    fn validate_hand_composition(
        input: &UserInput,
        master_counts: &[u8; 34],
    ) -> Result<(), &'static str> {
        // Total Meld Count
        if input.closed_kans.len() + input.open_melds.len() > 4 {
            return Err("Invalid hand: More than 4 total melds (kans + open melds) declared.");
        }

        // Total Tile Count based on melds
        let total_kans = input.closed_kans.len()
            + input
                .open_melds
                .iter()
                .filter(|m| m.mentsu_type == MentsuType::Kantsu)
                .count();

        
        let expected_tiles = (total_kans * 4) + ((4 - total_kans) * 3) + 2;

        let hand_len = input.hand_tiles.len();
        if hand_len == 14 && total_kans == 0 {
        } else if hand_len != expected_tiles {
            let err_msg = "Invalid hand: Tile count does not match declared kans. (Expected 14 for 0 kans, 15 for 1 kan, 16 for 2, 17 for 3, 18 for 4).";
            return Err(err_msg);
        }

        // Winning Tile Presence
        if !input.hand_tiles.contains(&input.winning_tile) {
            return Err("Invalid input: Winning tile is not present in the list of hand tiles.");
        }

        // Max 4 of any tile (checked from master_counts)
        if master_counts.iter().any(|&count| count > 4) {
            return Err("Invalid hand: Contains 5 or more of a single tile type.");
        }

        // Akadora counts (using your new field)
        let num_5m = master_counts[tile_to_index(&Hai::Suhai(5, Suhai::Manzu))];
        let num_5p = master_counts[tile_to_index(&Hai::Suhai(5, Suhai::Pinzu))];
        let num_5s = master_counts[tile_to_index(&Hai::Suhai(5, Suhai::Souzu))];
        let total_fives = num_5m + num_5p + num_5s;

        if input.game_context.num_akadora > total_fives {
            return Err(
                "Invalid input: Number of akadora exceeds the total number of '5' tiles in the hand.",
            );
        }
        
        if input.game_context.num_akadora > 4 {
            return Err("Invalid input: Number of akadora cannot be greater than 4.");
        }

        Ok(())
    }

    pub fn validate_input(input: &UserInput, master_counts: &[u8; 34]) -> Result<(), &'static str> {
        validate_game_state(
            &input.player_context,
            &input.game_context,
            input.agari_type,
            input,
        )?;
        
        validate_hand_composition(input, master_counts)?;
        
        Ok(())
    }
}

// Recursive Parsing
mod recursive_parser {
    use super::*;

    /// Find melds
    pub fn find_mentsu_recursive(counts: &mut [u8; 34], mentsu: &mut Vec<Mentsu>) -> bool {
        let mut i = 0;
        while i < 34 && counts[i] == 0 {
            i += 1;
        }
        if i == 34 {
            return true;
        }

        // Find Koutsu
        if counts[i] >= 3 {
            let tile = index_to_tile(i);
            counts[i] -= 3;
            mentsu.push(Mentsu {
                mentsu_type: MentsuType::Koutsu,
                is_minchou: false, // is_open
                tiles: [tile, tile, tile, tile], 
            });

            if find_mentsu_recursive(counts, mentsu) {
                return true;
            }

            mentsu.pop();
            counts[i] += 3;
        }

        // Find Shuntsu
        if i < 27 && (i % 9) < 7 && counts[i] > 0 && counts[i + 1] > 0 && counts[i + 2] > 0 {
            let tile1 = index_to_tile(i);
            let tile2 = index_to_tile(i + 1);
            let tile3 = index_to_tile(i + 2);

            counts[i] -= 1;
            counts[i + 1] -= 1;
            counts[i + 2] -= 1;
            mentsu.push(Mentsu {
                mentsu_type: MentsuType::Shuntsu,
                is_minchou: false,
                tiles: [tile1, tile2, tile3, tile3],
            });

            if find_mentsu_recursive(counts, mentsu) {
                return true;
            }

            mentsu.pop();
            counts[i] += 1;
            counts[i + 1] += 1;
            counts[i + 2] += 1;
        }

        false
    }
}

// Wait Type Analysis
mod wait_analyzer {
    use super::*;

    fn mentsu_contains_tile(mentsu: &Mentsu, tile: &Hai) -> bool {
        match mentsu.mentsu_type {
            MentsuType::Koutsu | MentsuType::Kantsu => mentsu.tiles[0] == *tile,
            MentsuType::Shuntsu => {
                mentsu.tiles[0] == *tile || mentsu.tiles[1] == *tile || mentsu.tiles[2] == *tile
            }
        }
    }

    pub fn determine_wait_type(
        mentsu: &[Mentsu; 4],
        atama: (Hai, Hai), 
        agari_hai: Hai,    
    ) -> Machi {
        if agari_hai == atama.0 {
            return Machi::Tanki;
        }

        let winning_meld = mentsu
            .iter()
            .find(|m| mentsu_contains_tile(m, &agari_hai))
            .expect("Winning tile not in pair or melds. Invalid hand.");

        match winning_meld.mentsu_type {
            MentsuType::Koutsu | MentsuType::Kantsu => Machi::Shanpon,
            MentsuType::Shuntsu => {
                let t1 = winning_meld.tiles[0];
                let t2 = winning_meld.tiles[1];
                let t3 = winning_meld.tiles[2];

                if agari_hai == t2 {
                    Machi::Kanchan
                } else if agari_hai == t1 {
                    if tile_to_index(&t3) % 9 == 8 {
                        Machi::Penchan
                    } else {
                        Machi::Ryanmen
                    }
                } else if agari_hai == t3 {
                    if tile_to_index(&t1) % 9 == 0 {
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


pub fn organize_hand(input: &UserInput) -> Result<HandOrganization, &'static str> {
    
    let mut master_counts = [0u8; 34];
    for tile in &input.hand_tiles {
        master_counts[tile_to_index(tile)] += 1;
    }
    input_validator::validate_input(input, &master_counts)?;

    let mut concealed_counts = master_counts;
    let mut final_mentsu: Vec<Mentsu> = Vec::with_capacity(4);

    for rep_tile in &input.closed_kans {
        let kan_tile = *rep_tile;
        let index = tile_to_index(&kan_tile);

        if concealed_counts[index] < 4 {
            return Err("Invalid input: declared closed kan not present in hand tiles.");
        }
        concealed_counts[index] -= 4;
        final_mentsu.push(Mentsu {
            mentsu_type: MentsuType::Kantsu,
            is_minchou: false, 
            tiles: [kan_tile, kan_tile, kan_tile, kan_tile],
        });
    }

    for meld in &input.open_melds {
        let rep_tile = meld.representative_tile;
        let index = tile_to_index(&rep_tile);

        match meld.mentsu_type {
            MentsuType::Koutsu => {
                if concealed_counts[index] < 3 {
                    return Err("Invalid input: declared Pon not present in hand tiles.");
                }
                concealed_counts[index] -= 3;
                final_mentsu.push(Mentsu {
                    mentsu_type: MentsuType::Koutsu,
                    is_minchou: true, 
                    tiles: [rep_tile, rep_tile, rep_tile, rep_tile], 
                });
            }
            MentsuType::Kantsu => {
                if concealed_counts[index] < 4 {
                    return Err("Invalid input: declared open Kan not present in hand tiles.");
                }
                concealed_counts[index] -= 4;
                final_mentsu.push(Mentsu {
                    mentsu_type: MentsuType::Kantsu,
                    is_minchou: true, 
                    tiles: [rep_tile, rep_tile, rep_tile, rep_tile],
                });
            }
            MentsuType::Shuntsu => {
                let index1 = index;
                let index2 = index1 + 1;
                let index3 = index1 + 2;

                if index1 >= 27 || (index1 % 9) >= 7 {
                    return Err("Invalid representative tile for Chi (must be 1-7 of a suit).");
                }
                if concealed_counts[index1] < 1
                    || concealed_counts[index2] < 1
                    || concealed_counts[index3] < 1
                {
                    return Err("Invalid input: declared Chi not present in hand tiles.");
                }

                concealed_counts[index1] -= 1;
                concealed_counts[index2] -= 1;
                concealed_counts[index3] -= 1;

                let t1 = rep_tile;
                let t2 = index_to_tile(index2);
                let t3 = index_to_tile(index3);
                final_mentsu.push(Mentsu {
                    mentsu_type: MentsuType::Shuntsu,
                    is_minchou: true, 
                    tiles: [t1, t2, t3, t3], 
                });
            }
        }
    }

    let mentsu_needed = 4 - final_mentsu.len();
    let agari_hai = input.winning_tile;

    // 4 known melds
    if mentsu_needed == 0 {
        for i in 0..34 {
            if concealed_counts[i] == 2 {
                let pair_tile = index_to_tile(i);
                let atama = (pair_tile, pair_tile);

                let mentsu_array: [Mentsu; 4] = final_mentsu
                    .try_into()
                    .expect("Hand parsing logic error: final_mentsu length not 4");

                let agari_hand = AgariHand {
                    mentsu: mentsu_array,
                    atama,
                    agari_hai,
                    machi: Machi::Tanki, 
                };

                return Ok(HandOrganization::YonmentsuIchiatama(agari_hand));
            }
        }
        if input.hand_tiles.len() == 14 {
             // placeholder
        } else {
            return Err("Invalid hand: 4 open melds but no pair found.");
        }
    }

    // Standard Hand
    for i in 0..34 {
        if concealed_counts[i] >= 2 {
            let mut temp_counts = concealed_counts; 
            temp_counts[i] -= 2;
            let atama = (index_to_tile(i), index_to_tile(i));
            let mut closed_mentsu: Vec<Mentsu> = Vec::with_capacity(mentsu_needed);

            if recursive_parser::find_mentsu_recursive(&mut temp_counts, &mut closed_mentsu) {
                if closed_mentsu.len() == mentsu_needed {
                    final_mentsu.append(&mut closed_mentsu);

                    let mentsu_array: [Mentsu; 4] = final_mentsu
                        .try_into()
                        .expect("Hand parsing logic error: final_mentsu length not 4");

                    let machi =
                        wait_analyzer::determine_wait_type(&mentsu_array, atama, agari_hai);

                    let agari_hand = AgariHand {
                        mentsu: mentsu_array,
                        atama,
                        agari_hai,
                        machi,
                    };

                    return Ok(HandOrganization::YonmentsuIchiatama(agari_hand));
                }
            }
        }
    }
    // Irregular Hand
    Ok(HandOrganization::Irregular {
        counts: master_counts,
        agari_hai,
    })
}