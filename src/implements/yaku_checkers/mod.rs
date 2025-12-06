use crate::implements::types::{
    game::{AgariType, GameContext, PlayerContext},
    hand::{HandOrganization, HandStructure},
    yaku::Yaku,
};

pub mod standard;
pub mod utils;
pub mod yakuman;

use standard::*;
use utils::*;
use yakuman::*;

#[derive(Debug, Clone)]
pub struct YakuResult {
    pub hand_structure: HandStructure,
    pub yaku_list: Vec<Yaku>,
    pub num_akadora: u8,
}

pub fn check_all_yaku(
    organization: HandOrganization,
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> Result<YakuResult, &'static str> {
    // game-state Yakuman
    let mut yakuman_list = check_game_state_yakuman(player, game);

    // hand-based Yakuman
    let (hand_structure, hand_yakuman) =
        match resolve_hand_structure(organization, player, game, agari_type) {
            Ok((structure, yakuman)) => (structure, yakuman),
            Err(e) => return Err(e),
        };

    yakuman_list.extend(hand_yakuman);

    if !yakuman_list.is_empty() {
        let final_yakuman = post_process_yakuman(yakuman_list);

        return Ok(YakuResult {
            hand_structure,
            yaku_list: final_yakuman,
            num_akadora: 0,
        });
    }

    // regular yaku
    let mut regular_yaku: Vec<Yaku> = match &hand_structure {
        HandStructure::YonmentsuIchiatama(agari_hand) => {
            find_standard_yaku(agari_hand, player, game, agari_type)
        }
        HandStructure::Chiitoitsu {
            pairs,
            agari_hai,
            machi,
        } => find_chiitoitsu_yaku(pairs, agari_hai, machi, player, game, agari_type),
        _ => vec![],
    };

    // Dora
    let has_yaku = !regular_yaku.is_empty() || player.is_riichi || player.is_daburu_riichi;

    let mut num_akadora_to_add = 0;

    if has_yaku {
        let all_tiles = get_all_tiles_from_structure(&hand_structure);

        let dora_count = count_dora(&all_tiles, &game.dora_indicators);
        for _ in 0..dora_count {
            regular_yaku.push(Yaku::Dora);
        }

        if (player.is_riichi || player.is_daburu_riichi) && !game.uradora_indicators.is_empty() {
            let uradora_count = count_dora(&all_tiles, &game.uradora_indicators);
            for _ in 0..uradora_count {
                regular_yaku.push(Yaku::UraDora);
            }
        }

        if game.num_akadora > 0 {
            num_akadora_to_add = game.num_akadora;
            for _ in 0..game.num_akadora {
                regular_yaku.push(Yaku::AkaDora);
            }
        }
    }

    Ok(YakuResult {
        hand_structure,
        yaku_list: regular_yaku,
        num_akadora: num_akadora_to_add,
    })
}

// unwrap raw_hand_organizer.rs
fn resolve_hand_structure(
    org: HandOrganization,
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> Result<(HandStructure, Vec<Yaku>), &'static str> {
    match org {
        HandOrganization::YonmentsuIchiatama(agari_hand) => {
            let (yakuman_list, chuuren_flag) =
                check_standard_yakuman(&agari_hand, player, game, agari_type);

            let structure = if let Some(is_junsei) = chuuren_flag {
                HandStructure::ChuurenPoutou {
                    hand: agari_hand,
                    _is_junsei: is_junsei,
                }
            } else {
                HandStructure::YonmentsuIchiatama(agari_hand)
            };

            Ok((structure, yakuman_list))
        }
        HandOrganization::Irregular { counts, agari_hai } => {
            // Kokushi
            if let Some((kokushi_structure, kokushi_yaku)) = check_kokushi(&counts, agari_hai) {
                Ok((kokushi_structure, vec![kokushi_yaku]))
            }
            // Chiitoitsu
            else if let Some(chiitoitsu_structure) = check_chiitoitsu(&counts, agari_hai) {
                let yakuman = check_chiitoitsu_yakuman(&chiitoitsu_structure);
                Ok((chiitoitsu_structure, yakuman))
            } else {
                Err("No Yaku Found")
            }
        }
    }
}
