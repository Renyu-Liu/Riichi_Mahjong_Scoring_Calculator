pub mod recursive_parser;
pub mod wait_analyzer;

use self::{recursive_parser::find_all_mentsu_recursive, wait_analyzer::determine_wait_type};
use crate::implements::types::{
    game::AgariType,
    hand::{AgariHand, HandOrganization, Machi, Mentsu, MentsuType},
    input::UserInput,
    tiles::{index_to_tile, tile_to_index},
};
use std::convert::TryInto;

pub fn organize_hand(input: &UserInput) -> Result<Vec<HandOrganization>, &'static str> {
    let mut master_counts = [0u8; 34];
    for tile in &input.hand_tiles {
        master_counts[tile_to_index(tile)] += 1;
    }

    if input.agari_type == AgariType::Ron {
        master_counts[tile_to_index(&input.winning_tile)] += 1;
    }

    let concealed_counts = master_counts;
    let mut final_results: Vec<HandOrganization> = Vec::new();
    let mut open_mentsu: Vec<Mentsu> = Vec::with_capacity(4);

    // Closed Kans
    for rep_tile in &input.closed_kans {
        let kan_tile = *rep_tile;
        open_mentsu.push(Mentsu {
            mentsu_type: MentsuType::Kantsu,
            is_minchou: false,
            tiles: [kan_tile, kan_tile, kan_tile, kan_tile],
        });
    }

    // Open Melds
    for meld in &input.open_melds {
        let rep_tile = meld.representative_tile;
        let index = tile_to_index(&rep_tile);

        match meld.mentsu_type {
            MentsuType::Koutsu => {
                open_mentsu.push(Mentsu {
                    mentsu_type: MentsuType::Koutsu,
                    is_minchou: true,
                    tiles: [rep_tile, rep_tile, rep_tile, rep_tile],
                });
            }
            MentsuType::Kantsu => {
                open_mentsu.push(Mentsu {
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
                    return Err("Invalid representative tile for Chi (must be 1-7)");
                }

                let t1 = rep_tile;
                let t2 = index_to_tile(index2);
                let t3 = index_to_tile(index3);
                open_mentsu.push(Mentsu {
                    mentsu_type: MentsuType::Shuntsu,
                    is_minchou: true,
                    tiles: [t1, t2, t3, t3],
                });
            }
        }
    }

    let mentsu_needed = 4 - open_mentsu.len();
    let agari_hai = input.winning_tile;

    // 4 known melds
    if mentsu_needed == 0 {
        for i in 0..34 {
            if concealed_counts[i] == 2 {
                let pair_tile = index_to_tile(i);
                let atama = (pair_tile, pair_tile);

                let mentsu_array: [Mentsu; 4] = open_mentsu
                    .clone()
                    .try_into()
                    .map_err(|_| "final_mentsu length not 4")?;

                let agari_hand = AgariHand {
                    mentsu: mentsu_array,
                    atama,
                    agari_hai,
                    machi: Machi::Tanki,
                };

                final_results.push(HandOrganization::YonmentsuIchiatama(agari_hand));
            }
        }
        if input.hand_tiles.len() == 14 {
        } else if final_results.is_empty() {
            return Err("4 open melds but no pair found");
        }
    } else {
        // Standard Hand
        for i in 0..34 {
            if concealed_counts[i] >= 2 {
                let mut temp_counts = concealed_counts;
                temp_counts[i] -= 2;
                let atama = (index_to_tile(i), index_to_tile(i));
                let mut closed_mentsu: Vec<Mentsu> = Vec::with_capacity(mentsu_needed);
                let mut recursive_results: Vec<Vec<Mentsu>> = Vec::new();

                find_all_mentsu_recursive(
                    &mut temp_counts,
                    &mut closed_mentsu,
                    &mut recursive_results,
                );

                for res in recursive_results {
                    if res.len() == mentsu_needed {
                        let mut full_mentsu = open_mentsu.clone();
                        full_mentsu.extend(res);

                        let mentsu_array: [Mentsu; 4] = full_mentsu
                            .try_into()
                            .map_err(|_| "final_mentsu length not 4")?;

                        let possible_waits = determine_wait_type(&mentsu_array, atama, agari_hai);

                        for machi in possible_waits {
                            let agari_hand = AgariHand {
                                mentsu: mentsu_array,
                                atama,
                                agari_hai,
                                machi,
                            };

                            final_results.push(HandOrganization::YonmentsuIchiatama(agari_hand));
                        }
                    }
                }
            }
        }
    }

    final_results.push(HandOrganization::Irregular {
        counts: master_counts,
        agari_hai,
    });

    Ok(final_results)
}
