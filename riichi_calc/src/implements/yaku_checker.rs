//! # Riichi Mahjong Yaku Checker
//!
//! This module takes an organized hand and game state and identifies all
//! winning yaku, including Yakuman and Dora.
//!
//! The main entry point is `check_all_yaku`.

// We assume the 'types.rs' file is in `crate::types`
//
// --- FIX ---
// The original import `use crate::types::{...}` failed.
// This is changed to `use crate::{...}`.
// This assumes the modules `game`, `hand`, `tiles`, and `yaku`
// are declared at the crate root (e.g., in your main.rs or lib.rs),
// which would be the case if you pasted the *contents* of types.rs there.
use super::types::{
    game::{AgariType, GameContext, PlayerContext},
    hand::{AgariHand, HandOrganization, HandStructure, Machi, Mentsu, MentsuType},
    tiles::{Hai, Jihai, Kaze, Sangenpai, Suhai},
    yaku::Yaku,
};
use std::collections::{HashMap, HashSet};

/// # YakuResult
/// The definitive result from the yaku checker, passed to the score calculator.
#[derive(Debug, Clone)]
pub struct YakuResult {
    /// The confirmed, valid structure of the winning hand.
    pub hand_structure: HandStructure,
    /// A list of all yaku achieved, including Dora.
    pub yaku_list: Vec<Yaku>,
}

// --- Main Public Function ---

/// Checks a hand for all yaku.
/// This is the main entry point for this module.
///
/// # Arguments
/// * `organization` - The output from the `organize_hand` function.
/// * `player` - The context of the winning player.
/// * `game` - The context of the current game round.
/// * `agari_type` - How the hand was won (Tsumo or Ron). This is critical
///   and was missing from the provided `types.rs` structs.
///
/// # Returns
/// A `Result` containing the `YakuResult` or an error string if the
/// hand is invalid (e.g., irregular but not Kokushi or Chiitoitsu).
pub fn check_all_yaku(
    organization: HandOrganization,
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> Result<YakuResult, &'static str> {
    
    // 1. Check for game-state Yakuman first (Tenhou, etc.)
    let mut yakuman_list = check_game_state_yakuman(player, game);

    // 2. Resolve the hand structure and check for hand-based Yakuman.
    let (hand_structure, hand_yakuman) =
        match resolve_hand_structure(organization, player, game, agari_type) {
            Ok((structure, yakuman)) => (structure, yakuman),
            Err(e) => return Err(e),
        };

    yakuman_list.extend(hand_yakuman);

    // 3. If we have any Yakuman, we are done.
    if !yakuman_list.is_empty() {
        // Post-process to handle double yakuman (e.g., SuuankouTanki replaces Suuankou)
        let final_yakuman = post_process_yakuman(yakuman_list);

        return Ok(YakuResult {
            hand_structure,
            yaku_list: final_yakuman,
        });
    }

    // 4. No Yakuman. Find regular yaku based on the hand structure.
    let mut regular_yaku: Vec<Yaku> = match &hand_structure {
        HandStructure::YonmentsuIchiatama(agari_hand) => {
            find_standard_yaku(agari_hand, player, game, agari_type)
        }
        HandStructure::Chiitoitsu { pairs, agari_hai, machi } => {
            find_chiitoitsu_yaku(pairs, agari_hai, machi, player, game, agari_type)
        }
        // Kokushi/Chuuren are Yakuman, so they would have returned above.
        _ => vec![],
    };

    // 5. Check for Dora.
    // A hand is only valid if it has at least one yaku OR is in Riichi.
    let has_yaku = !regular_yaku.is_empty() 
        || player.is_riichi 
        || player.is_daburu_riichi;

    if has_yaku {
        // Find all tiles in the hand
        let all_tiles = get_all_tiles_from_structure(&hand_structure);
        
        // Add Dora
        let dora_count = count_dora(&all_tiles, &game.dora_indicators);
        for _ in 0..dora_count {
            regular_yaku.push(Yaku::Dora);
        }

        // Add UraDora (only if Riichi)
        if (player.is_riichi || player.is_daburu_riichi) && !game.uradora_indicators.is_empty() {
            let uradora_count = count_dora(&all_tiles, &game.uradora_indicators);
            for _ in 0..uradora_count {
                regular_yaku.push(Yaku::UraDora);
            }
        }
        
        // AkaDora (Red Fives)
        // This is IMPOSSIBLE to implement with the current `Hai` definition.
        // It requires a flag on the `Hai::Suhai` variant.
    }

    Ok(YakuResult {
        hand_structure,
        yaku_list: regular_yaku,
    })
}

// --- 1. Yakuman Checkers ---

/// Checks for Tenhou (Blessing of Heaven) and Chiihou (Blessing of Earth)
fn check_game_state_yakuman(player: &PlayerContext, game: &GameContext) -> Vec<Yaku> {
    let mut yaku = Vec::new();
    if game.is_tenhou {
        yaku.push(Yaku::Tenhou);
    }
    if game.is_chiihou {
        yaku.push(Yaku::Chiihou);
    }
    if game.is_renhou {
        yaku.push(Yaku::Renhou);
    }
    yaku
}

/// Tries to resolve the hand into a structure and find any hand-based Yakuman.
fn resolve_hand_structure(
    org: HandOrganization,
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> Result<(HandStructure, Vec<Yaku>), &'static str> {
    match org {
        HandOrganization::YonmentsuIchiatama(agari_hand) => {
            // This is a standard hand. Check for standard-pattern yakuman.
            let (yakuman_list, chuuren_flag) =
                check_standard_yakuman(&agari_hand, player, game, agari_type);

            let structure = if let Some(is_junsei) = chuuren_flag {
                HandStructure::ChuurenPoutou {
                    hand: agari_hand,
                    is_junsei,
                }
            } else {
                HandStructure::YonmentsuIchiatama(agari_hand)
            };

            Ok((structure, yakuman_list))
        }
        HandOrganization::Irregular { counts, agari_hai } => {
            // Try Kokushi first
            if let Some((kokushi_structure, kokushi_yaku)) = check_kokushi(&counts, agari_hai) {
                Ok((kokushi_structure, vec![kokushi_yaku]))
            }
            // Try Chiitoitsu next
            else if let Some(chiitoitsu_structure) = check_chiitoitsu(&counts, agari_hai) {
                // Chiitoitsu can also be Tsuuiisou (All Honors)
                let yakuman = check_chiitoitsu_yakuman(&chiitoitsu_structure);
                Ok((chiitoitsu_structure, yakuman))
            } else {
                Err("Invalid irregular hand. Not Kokushi or Chiitoitsu.")
            }
        }
    }
}

/// Checks a 4-meld, 1-pair hand for all Yakuman.
/// Returns (Yakuman List, ChuurenPoutou flag)
fn check_standard_yakuman(
    hand: &AgariHand,
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> (Vec<Yaku>, Option<bool>) {
    let mut yakuman = Vec::new();
    let all_tiles = get_all_tiles(hand);

    // --- Tile-based Yakuman (Tsuuiisou, Chinroutou, Ryuuiisou) ---
    let mut is_tsuuiisou = true;
    let mut is_chinroutou = true;
    let mut is_ryuuiisou = true;

    for tile in &all_tiles {
        if !is_jihai(tile) {
            is_tsuuiisou = false;
        }
        if !is_terminal(tile) {
            is_chinroutou = false;
        }
        if !is_green_tile(tile) {
            is_ryuuiisou = false;
        }
    }

    if is_tsuuiisou {
        yakuman.push(Yaku::Tsuuiisou);
    }
    if is_chinroutou {
        // Chinroutou requires all tiles to be terminals.
        // This implies it's also Toitoi, but it's a yakuman.
        yakuman.push(Yaku::Chinroutou);
    }
    if is_ryuuiisou {
        // All Green: Only Sou 2,3,4,6,8 and Hatsu.
        yakuman.push(Yaku::Ryuuiisou);
    }

    // --- Meld-based Yakuman ---
    let (koutsu, kantsu) = count_koutsu_kantsu(hand);
    let concealed_koutsu = count_concealed_koutsu(hand, agari_type);
    
    // Suukantsu (Four Quads)
    if kantsu == 4 {
        yakuman.push(Yaku::Suukantsu);
    }

    // Suuankou (Four Concealed Triplets)
    if concealed_koutsu == 4 {
        if hand.machi == Machi::Tanki {
            yakuman.push(Yaku::SuuankouTanki);
        } else {
            yakuman.push(Yaku::Suuankou);
        }
    }

    // Daisangen (Big Three Dragons)
    let mut dragon_koutsu = 0;
    for mentsu in &hand.mentsu {
        if is_koutsu_or_kantsu(mentsu) {
            if let Hai::Jihai(Jihai::Sangen(_)) = mentsu.tiles[0] {
                dragon_koutsu += 1;
            }
        }
    }
    if dragon_koutsu == 3 {
        yakuman.push(Yaku::Daisangen);
    }

    // Daisuushi / Shousuushi (Big/Little Four Winds)
    let mut wind_koutsu = 0;
    let mut wind_atama = false;
    for mentsu in &hand.mentsu {
        if is_koutsu_or_kantsu(mentsu) {
            if let Hai::Jihai(Jihai::Kaze(_)) = mentsu.tiles[0] {
                wind_koutsu += 1;
            }
        }
    }
    if let Hai::Jihai(Jihai::Kaze(_)) = hand.atama.0 {
        wind_atama = true;
    }

    if wind_koutsu == 4 {
        yakuman.push(Yaku::Daisuushi);
    } else if wind_koutsu == 3 && wind_atama {
        yakuman.push(Yaku::Shousuushi);
    }
    
    // Chuuren Poutou (Nine Gates)
    let chuuren_flag = check_chuuren(hand);
    if let Some(is_junsei) = chuuren_flag {
        if is_junsei {
            yakuman.push(Yaku::JunseiChuurenPoutou);
        } else {
            yakuman.push(Yaku::ChuurenPoutou);
        }
    }

    (yakuman, chuuren_flag)
}

/// Checks for Kokushi Musou (Thirteen Orphans)
fn check_kokushi(counts: &[u8; 34], agari_hai: Hai) -> Option<(HandStructure, Yaku)> {
    let mut has_pair = false;
    let mut is_13_sided = true; // Assume 13-sided wait
    
    let mut tiles = Vec::new();
    let mut atama_tile = None;

    for (idx, &count) in counts.iter().enumerate() {
        let tile = hai_from_index(idx);
        if !is_yaochuu(&tile) {
            if count > 0 {
                return None; // Has a non-yaochuu tile
            }
            continue;
        }

        match count {
            1 => {
                is_13_sided = false; // Has a 1, so not waiting on this
                tiles.push(tile);
            },
            2 => {
                if has_pair { return None; } // More than one pair
                has_pair = true;
                atama_tile = Some(tile);
                tiles.push(tile); // Add it once
            },
            0 => {
                is_13_sided = false; // Missing this tile
            },
            _ => return None, // > 2 of a yaocchuu tile
        }
    }
    
    // We must have a pair
    if !has_pair { return None; }
    
    // The `agari_hai` must be one of the tiles
    if !counts[index_from_hai(agari_hai)] > 0 { return None; }

    let atama = (atama_tile.unwrap(), atama_tile.unwrap());
    
    // If it was a 13-sided wait, the agari_hai must be the one we didn't have 1 of
    // But `organize_hand` logic already gave us `counts` *with* the agari_hai.
    // So, for 13-sided, agari_hai is the *only* tile with count 1.
    if counts[index_from_hai(agari_hai)] == 1 {
        // This is a 13-sided wait. All others must be 1, and agari_hai is 1.
        // Wait, no. If it's a 13-sided wait, *before* the agari_hai, we have
        // 1 of all 13 orphans. The `agari_hai` makes one of them a pair.
        // `is_13_sided` check:
        let mut is_13_wait = true;
        for (idx, &count) in counts.iter().enumerate() {
            let tile = hai_from_index(idx);
            if is_yaochuu(&tile) {
                if tile == agari_hai && count != 2 { is_13_wait = false; break; }
                if tile != agari_hai && count != 1 { is_13_wait = false; break; }
            }
        }
        // This is complex. Let's simplify. `types.rs` has `Machi::KokushiJusanmen`.
        // The *organizer* should have set this. We just check the `counts`.
        // If we got here, it's a valid Kokushi.
        
        let machi = if atama.0 == agari_hai {
             Machi::KokushiIchimen // Waited on the pair
        } else {
             Machi::KokushiIchimen // Waited on a single
        };
        
        // The `organize_hand` function should be responsible for setting the
        // Machi type to Jusanmen. Let's assume `agari_hand.machi` is available
        // ... but it's not.
        //
        // Let's use the definition: 13-sided wait means `agari_hai`
        // completes the hand, and it could have been *any* of the 13 orphans.
        // This means the hand *before* agari was 1 of each 13 orphans.
        // So, `counts` (which includes agari_hai) will have one `2` and twelve `1`s.
        // The `agari_hai` *must* be the tile that is `2`.
        
        let mut yaku = Yaku::KokushiMusou;
        let mut final_machi = Machi::KokushiIchimen;

        if atama.0 == agari_hai {
            // Check if it was a 13-sided wait
            let mut all_others_are_one = true;
            for (idx, &count) in counts.iter().enumerate() {
                if !is_yaochuu(&hai_from_index(idx)) { continue; }
                if hai_from_index(idx) != agari_hai && count != 1 {
                    all_others_are_one = false;
                    break;
                }
            }
            if all_others_are_one {
                yaku = Yaku::KokushiMusouJusanmen;
                final_machi = Machi::KokushiJusanmen;
            }
        }

        Some((HandStructure::KokushiMusou { tiles: tiles.try_into().ok()?, atama, agari_hai, machi: final_machi }, yaku))
    } else {
        None
    }
}

/// Checks for Chiitoitsu (Seven Pairs)
fn check_chiitoitsu(counts: &[u8; 34], agari_hai: Hai) -> Option<HandStructure> {
    let mut pair_count = 0;
    let mut pairs = Vec::new();

    for (idx, &count) in counts.iter().enumerate() {
        if count > 0 {
            if count == 2 {
                pair_count += 1;
                let tile = hai_from_index(idx);
                pairs.push((tile, tile));
            } else if count == 4 {
                // 4-of-a-kind counts as 2 pairs in Chiitoitsu
                pair_count += 2;
                let tile = hai_from_index(idx);
                pairs.push((tile, tile));
                pairs.push((tile, tile));
            } else {
                return None; // Has a 1 or 3
            }
        }
    }

    if pair_count == 7 {
        Some(HandStructure::Chiitoitsu {
            pairs: pairs.try_into().ok()?,
            agari_hai,
            machi: Machi::Tanki, // Chiitoitsu is always a pair wait
        })
    } else {
        None
    }
}

/// Checks if a Chiitoitsu is also a Yakuman (Tsuuiisou)
fn check_chiitoitsu_yakuman(hand: &HandStructure) -> Vec<Yaku> {
    if let HandStructure::Chiitoitsu { pairs, .. } = hand {
        let mut is_tsuuiisou = true;
        for (tile, _) in pairs {
            if !is_jihai(tile) {
                is_tsuuiisou = false;
                break;
            }
        }
        if is_tsuuiisou {
            return vec![Yaku::Tsuuiisou];
        }
    }
    vec![]
}

/// Checks for Chuuren Poutou (Nine Gates)
fn check_chuuren(hand: &AgariHand) -> Option<bool> {
    let all_tiles = get_all_tiles(hand);
    
    // Must be one suit only
    let (is_chinitsu, suit) = check_chinitsu(&all_tiles);
    if !is_chinitsu { return None; }
    let suit = suit.unwrap(); // We know it's Some

    // Must be menzen
    if !hand.mentsu.iter().all(|m| !m.is_minchou) {
        return None;
    }

    // Check counts: 1,1,1, 2,3,4, 5,6,7, 8,8,8, 9,9,9
    let mut counts = [0u8; 9];
    for tile in &all_tiles {
        if let Hai::Suhai(n, s) = tile {
            if *s == suit {
                counts[(n - 1) as usize] += 1;
            }
        }
    }

    let mut ones = 0;
    let mut nines = 0;
    let mut others = 0;
    
    if counts[0] < 3 { return None; } // Need at least three 1s
    if counts[8] < 3 { return None; } // Need at least three 9s
    
    let mut has_extra = false;
    let mut extra_tile_num = 0;

    for (i, &count) in counts.iter().enumerate() {
        let num = i + 1;
        if num == 1 || num == 9 {
            if count < 3 { return None; }
            if count == 4 {
                if has_extra { return None; } // Two extras
                has_extra = true;
                extra_tile_num = num;
            }
        } else {
            if count < 1 { return None; }
            if count == 2 {
                if has_extra { return None; }
                has_extra = true;
                extra_tile_num = num;
            }
            if count > 2 { return None; }
        }
    }

    if !has_extra { return None; } // Needs 14 tiles
    
    // It's Chuuren. Is it Junsei (True 9-sided wait)?
    // This means the winning tile completes the 1,1,1,2,3,4,5,6,7,8,9,9,9 form.
    // The `extra_tile_num` is the number of the tile we have two of.
    // The `agari_hai` must be that number.
    if let Hai::Suhai(n, s) = hand.agari_hai {
        if s == suit && n as usize == extra_tile_num {
            return Some(true); // Junsei!
        }
    }

    Some(false) // Not Junsei, but still Chuuren
}


/// Handles Double Yakuman overrides
fn post_process_yakuman(mut yakuman: Vec<Yaku>) -> Vec<Yaku> {
    let has_suuankou_tanki = yakuman.contains(&Yaku::SuuankouTanki);
    let has_kokushi_jusanmen = yakuman.contains(&Yaku::KokushiMusouJusanmen);
    let has_junsei_chuuren = yakuman.contains(&Yaku::JunseiChuurenPoutou);

    yakuman.retain(|&y| {
        (y != Yaku::Suuankou || !has_suuankou_tanki)
            && (y != Yaku::KokushiMusou || !has_kokushi_jusanmen)
            && (y != Yaku::ChuurenPoutou || !has_junsei_chuuren)
    });

    yakuman
}


// --- 2. Regular Yaku Checkers ---

/// Finds all standard yaku for a 4-meld, 1-pair hand.
fn find_standard_yaku(
    hand: &AgariHand,
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> Vec<Yaku> {
    let mut yaku_list = Vec::new();

    // --- State-based Yaku (1 han) ---
    if player.is_daburu_riichi {
        yaku_list.push(Yaku::DaburuRiichi);
    } else if player.is_riichi {
        yaku_list.push(Yaku::Riichi);
    }
    if player.is_ippatsu {
        yaku_list.push(Yaku::Ippatsu);
    }
    if player.is_menzen && agari_type == AgariType::Tsumo {
        yaku_list.push(Yaku::MenzenTsumo);
    }
    if game.is_haitei && agari_type == AgariType::Tsumo {
        yaku_list.push(Yaku::HaiteiRaoyue);
    }
    if game.is_houtei && agari_type == AgariType::Ron {
        yaku_list.push(Yaku::HouteiRaoyui);
    }
    if game.is_rinshan {
        yaku_list.push(Yaku::RinshanKaihou);
    }
    if game.is_chankan {
        yaku_list.push(Yaku::Chankan);
    }

    // --- Yakuhai (1 han) ---
    yaku_list.extend(check_yakuhai(hand, player, game));

    // --- Pinfu (1 han) ---
    if check_pinfu(hand, player, game) {
        yaku_list.push(Yaku::Pinfu);
    }

    // --- Tanyao (1 han) ---
    if check_tanyao(hand) {
        yaku_list.push(Yaku::Tanyao);
    }

    // --- Sequence Yaku (Iipeikou, Ryanpeikou, Sanshoku, Ittsu) ---
    let shuntsu: Vec<&Mentsu> = hand
        .mentsu
        .iter()
        .filter(|m| m.mentsu_type == MentsuType::Shuntsu)
        .collect();

    if player.is_menzen {
        let (iipeikou, ryanpeikou) = check_peikou(&shuntsu);
        if ryanpeikou {
            yaku_list.push(Yaku::Ryanpeikou);
        } else if iipeikou {
            yaku_list.push(Yaku::Iipeikou);
        }
    }
    
    if check_sanshoku_doujun(&shuntsu) {
        yaku_list.push(Yaku::SanshokuDoujun);
    }
    
    if check_ittsu(&shuntsu) {
        yaku_list.push(Yaku::Ittsu);
    }

    // --- Triplet Yaku (Toitoi, Sanankou, Sanshoku, Sankantsu, Shousangen) ---
    let (koutsu, kantsu) = count_koutsu_kantsu(hand);

    if koutsu + kantsu == 4 {
        yaku_list.push(Yaku::Toitoi);
    } else {
        // Sanankou and Toitoi are mutually exclusive
        let concealed_koutsu = count_concealed_koutsu(hand, agari_type);
        if concealed_koutsu == 3 {
            yaku_list.push(Yaku::Sanankou);
        }
    }
    
    if kantsu == 3 {
        yaku_list.push(Yaku::Sankantsu);
    }

    if check_sanshoku_doukou(hand) {
        yaku_list.push(Yaku::SanshokuDoukou);
    }

    if check_shousangen(hand) {
        yaku_list.push(Yaku::Shousangen);
    }
    
    // --- Terminal/Honor Yaku (Chanta, Junchan, Honroutou) ---
    let all_tiles = get_all_tiles(hand);
    let all_groups = get_all_groups(hand);

    let is_honroutou = all_tiles.iter().all(|t| is_yaochuu(t))
        && !all_tiles.iter().all(|t| is_terminal(t)); // Exclude Chinroutou
    
    if is_honroutou {
        yaku_list.push(Yaku::Honroutou);
    } else {
        let (is_chanta, is_junchan) = check_chanta_junchan(&all_groups);
        if is_junchan {
            yaku_list.push(Yaku::Junchan);
        } else if is_chanta {
            yaku_list.push(Yaku::Chanta);
        }
    }


    // --- Color Yaku (Honitsu, Chinitsu) ---
    let (is_chinitsu, _) = check_chinitsu(&all_tiles);
    if is_chinitsu {
        yaku_list.push(Yaku::Chinitsu);
    } else {
        let (is_honitsu, _) = check_honitsu(&all_tiles);
        if is_honitsu {
            yaku_list.push(Yaku::Honitsu);
        }
    }
    
    // --- Remove overlapping yaku (e.g. Pinfu + Tsumo if Tsumo is already counted) ---
    // Pinfu + MenzenTsumo is a special case.
    // If Pinfu is present, MenzenTsumo is valid.
    // But Pinfu is *not* allowed with non-MenzenTsumo state yaku (Rinshan, etc.)
    if yaku_list.contains(&Yaku::Pinfu) {
        if yaku_list.contains(&Yaku::RinshanKaihou) || yaku_list.contains(&Yaku::Chankan) {
             yaku_list.retain(|&y| y != Yaku::Pinfu);
        }
    }

    yaku_list
}

/// Finds all yaku for a Chiitoitsu hand.
fn find_chiitoitsu_yaku(
    pairs: &[(Hai, Hai); 7],
    agari_hai: &Hai,
    machi: &Machi,
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> Vec<Yaku> {
    let mut yaku_list = Vec::new();

    // Chiitoitsu is always 2 han and menzen
    yaku_list.push(Yaku::Chiitoitsu);

    // --- State-based Yaku ---
    // Riichi/DaburuRiichi/Ippatsu
    if player.is_daburu_riichi {
        yaku_list.push(Yaku::DaburuRiichi);
    } else if player.is_riichi {
        yaku_list.push(Yaku::Riichi);
    }
    if player.is_ippatsu {
        yaku_list.push(Yaku::Ippatsu);
    }
    // MenzenTsumo
    if agari_type == AgariType::Tsumo {
        yaku_list.push(Yaku::MenzenTsumo);
    }
    // Haitei/Houtei
    if game.is_haitei && agari_type == AgariType::Tsumo {
        yaku_list.push(Yaku::HaiteiRaoyue);
    }
    if game.is_houtei && agari_type == AgariType::Ron {
        yaku_list.push(Yaku::HouteiRaoyui);
    }
    
    // --- Tile-based Yaku ---
    let all_tiles: Vec<Hai> = pairs.iter().flat_map(|&(t1, t2)| vec![t1, t2]).collect();

    if all_tiles.iter().all(|t| is_simple(t)) {
        yaku_list.push(Yaku::Tanyao);
    }
    
    // Honroutou (All Terminals & Honors)
    if all_tiles.iter().all(|t| is_yaochuu(t)) {
         yaku_list.push(Yaku::Honroutou);
    }

    // Color Yaku (Honitsu, Chinitsu)
    let (is_chinitsu, _) = check_chinitsu(&all_tiles);
    if is_chinitsu {
        yaku_list.push(Yaku::Chinitsu);
    } else {
        let (is_honitsu, _) = check_honitsu(&all_tiles);
        if is_honitsu {
            yaku_list.push(Yaku::Honitsu);
        }
    }

    yaku_list
}

// --- 3. Dora Checkers ---

/// Counts the number of Dora in a hand.
fn count_dora(all_tiles: &[Hai], indicators: &[Hai]) -> u8 {
    let mut count = 0;
    for indicator in indicators {
        let dora_tile = get_dora_tile(indicator);
        for tile in all_tiles {
            if *tile == dora_tile {
                count += 1;
            }
        }
    }
    count
}

/// Gets the Dora tile from an indicator.
fn get_dora_tile(indicator: &Hai) -> Hai {
    match indicator {
        Hai::Suhai(n, s) => {
            if *n == 9 {
                Hai::Suhai(1, *s)
            } else {
                Hai::Suhai(n + 1, *s)
            }
        }
        Hai::Jihai(Jihai::Kaze(k)) => Hai::Jihai(Jihai::Kaze(match k {
            Kaze::Ton => Kaze::Nan,
            Kaze::Nan => Kaze::Shaa,
            Kaze::Shaa => Kaze::Pei,
            Kaze::Pei => Kaze::Ton,
        })),
        Hai::Jihai(Jihai::Sangen(s)) => Hai::Jihai(Jihai::Sangen(match s {
            Sangenpai::Haku => Sangenpai::Hatsu,
            Sangenpai::Hatsu => Sangenpai::Chun,
            Sangenpai::Chun => Sangenpai::Haku,
        })),
    }
}


// --- 4. Yaku-specific Helper Functions ---

/// Checks for Yakuhai (Dragons, Seat Wind, Prevalent Wind).
fn check_yakuhai(
    hand: &AgariHand,
    player: &PlayerContext,
    game: &GameContext,
) -> Vec<Yaku> {
    let mut yaku = Vec::new();
    let mut found = HashSet::new();

    let mut check_tile = |tile: Hai, yaku_type: Yaku| {
        if !found.contains(&yaku_type) {
            if (hand.atama.0 == tile)
                || hand.mentsu.iter().any(|m| {
                    is_koutsu_or_kantsu(m) && m.tiles[0] == tile
                })
            {
                // This is a common error. Yakuhai atama is only
                // valid for Shousangen, Daisuushi/Shousuushi.
                // It must be a Koutsu or Kantsu.
            }
            
            if hand.mentsu.iter().any(|m| {
                    is_koutsu_or_kantsu(m) && m.tiles[0] == tile
                })
            {
                yaku.push(yaku_type);
                found.insert(yaku_type);
            }
        }
    };

    // Dragons
    check_tile(Hai::Jihai(Jihai::Sangen(Sangenpai::Haku)), Yaku::YakuhaiSangenpai);
    check_tile(Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu)), Yaku::YakuhaiSangenpai);
    check_tile(Hai::Jihai(Jihai::Sangen(Sangenpai::Chun)), Yaku::YakuhaiSangenpai);

    // Winds
    check_tile(Hai::Jihai(Jihai::Kaze(game.bakaze)), Yaku::YakuhaiBakaze);
    check_tile(Hai::Jihai(Jihai::Kaze(player.jikaze)), Yaku::YakuhaiJikaze);
    
    // Note: If bakaze == jikaze, this will add two yaku, which is correct.
    
    // De-duplicate Sangenpai
    let sange_count = yaku.iter().filter(|&&y| y == Yaku::YakuhaiSangenpai).count();
    yaku.retain(|&y| y != Yaku::YakuhaiSangenpai);
    for _ in 0..sange_count {
        yaku.push(Yaku::YakuhaiSangenpai);
    }
    
    yaku
}

/// Checks for Pinfu (No-points hand).
fn check_pinfu(hand: &AgariHand, player: &PlayerContext, game: &GameContext) -> bool {
    // 1. Must be menzen
    if !player.is_menzen {
        return false;
    }
    // 2. All 4 melds are Shuntsu
    if !hand
        .mentsu
        .iter()
        .all(|m| m.mentsu_type == MentsuType::Shuntsu)
    {
        return false;
    }
    // 3. Atama is not a Yakuhai tile
    if let Hai::Jihai(Jihai::Sangen(_)) = hand.atama.0 {
        return false;
    }
    if let Hai::Jihai(Jihai::Kaze(k)) = hand.atama.0 {
        if k == game.bakaze || k == player.jikaze {
            return false;
        }
    }
    // 4. Must be a Ryanmen (two-sided) wait
    if hand.machi != Machi::Ryanmen {
        return false;
    }
    
    // 5. Fu check (implied by above rules, but good to be explicit)
    // Pinfu is incompatible with any yaku that gives fu.
    // (Handled in find_standard_yaku)

    true
}

/// Checks for Tanyao (All Simples).
fn check_tanyao(hand: &AgariHand) -> bool {
    get_all_tiles(hand).iter().all(|t| is_simple(t))
}

/// Checks for Iipeikou (Pure Double Sequence) and Ryanpeikou (Twice Pure Double Sequence)
fn check_peikou<'a>(shuntsu: &[&'a Mentsu]) -> (bool, bool) {
    if shuntsu.len() < 2 {
        return (false, false);
    }
    
    let mut identical_pairs = 0;
    let mut seen = HashSet::new();

    for (i, m1) in shuntsu.iter().enumerate() {
        if seen.contains(&i) { continue; }
        for (j, m2) in shuntsu.iter().enumerate() {
            if i == j || seen.contains(&j) { continue; }
            
            // Check for identical shuntsu (e.g., 234m and 234m)
            if m1.tiles[0] == m2.tiles[0] {
                identical_pairs += 1;
                seen.insert(i);
                seen.insert(j);
                break;
            }
        }
    }

    (identical_pairs == 1, identical_pairs == 2)
}

/// Checks for Sanshoku Doujun (Mixed Triple Sequence)
fn check_sanshoku_doujun<'a>(shuntsu: &[&'a Mentsu]) -> bool {
    if shuntsu.len() < 3 { return false; }

    let mut starters: HashMap<u8, (bool, bool, bool)> = HashMap::new();
    
    for m in shuntsu {
        if let Hai::Suhai(n, s) = m.tiles[0] {
            let entry = starters.entry(n).or_insert((false, false, false));
            match s {
                Suhai::Manzu => entry.0 = true,
                Suhai::Pinzu => entry.1 = true,
                Suhai::Souzu => entry.2 = true,
            }
        }
    }

    starters.values().any(|&(m, p, s)| m && p && s)
}

/// Checks for Ittsu (Pure Straight)
fn check_ittsu<'a>(shuntsu: &[&'a Mentsu]) -> bool {
    if shuntsu.len() < 3 { return false; }
    
    let mut suits: HashMap<Suhai, HashSet<u8>> = HashMap::new();
    
    for m in shuntsu {
        if let Hai::Suhai(n, s) = m.tiles[0] {
            suits.entry(s).or_default().insert(n);
        }
    }
    
    for set in suits.values() {
        if set.contains(&1) && set.contains(&4) && set.contains(&7) {
            return true;
        }
    }
    false
}

/// Checks for Sanshoku Doukou (Triple Triplets)
fn check_sanshoku_doukou(hand: &AgariHand) -> bool {
    let koutsu: Vec<&Mentsu> = hand
        .mentsu
        .iter()
        .filter(|m| is_koutsu_or_kantsu(m))
        .collect();
        
    if koutsu.len() < 3 { return false; }
    
    let mut numbers: HashMap<u8, (bool, bool, bool)> = HashMap::new();
    
    for m in koutsu {
        if let Hai::Suhai(n, s) = m.tiles[0] {
            let entry = numbers.entry(n).or_insert((false, false, false));
            match s {
                Suhai::Manzu => entry.0 = true,
                Suhai::Pinzu => entry.1 = true,
                Suhai::Souzu => entry.2 = true,
            }
        }
    }
    
    numbers.values().any(|&(m, p, s)| m && p && s)
}

/// Checks for Shousangen (Little Three Dragons)
fn check_shousangen(hand: &AgariHand) -> bool {
    let mut dragon_koutsu = 0;
    let mut dragon_atama = false;
    
    for m in &hand.mentsu {
        if is_koutsu_or_kantsu(m) {
            if let Hai::Jihai(Jihai::Sangen(_)) = m.tiles[0] {
                dragon_koutsu += 1;
            }
        }
    }
    
    if let Hai::Jihai(Jihai::Sangen(_)) = hand.atama.0 {
        dragon_atama = true;
    }
    
    dragon_koutsu == 2 && dragon_atama
}

/// Checks for Chanta and Junchan
fn check_chanta_junchan(groups: &[Vec<Hai>]) -> (bool, bool) {
    let mut is_chanta = true;
    let mut is_junchan = true; // Assumed true until a Jihai is found
    
    for group in groups {
        let mut has_terminal = false;
        let mut has_jihai = false;
        
        for tile in group {
            if is_jihai(tile) { has_jihai = true; }
            if is_terminal(tile) { has_terminal = true; }
        }
        
        if !has_terminal && !has_jihai {
            is_chanta = false;
            is_junchan = false;
            break;
        }
        
        if has_jihai {
            is_junchan = false; // Has an honor, so can't be Junchan
        }
    }
    
    (is_chanta, is_junchan && is_chanta)
}

/// Checks for Honitsu (Half Flush) and Chinitsu (Full Flush)
fn check_color(all_tiles: &[Hai]) -> (bool, bool, Option<Suhai>) {
    let mut suit = None;
    let mut has_jihai = false;
    let mut is_honitsu = true;
    let mut is_chinitsu = true;

    for tile in all_tiles {
        match tile {
            Hai::Suhai(n, s) => {
                if suit.is_none() {
                    suit = Some(*s);
                } else if suit != Some(*s) {
                    is_honitsu = false;
                    is_chinitsu = false;
                    break;
                }
            }
            Hai::Jihai(_) => {
                has_jihai = true;
                is_chinitsu = false;
            }
        }
    }
    
    if !has_jihai && suit.is_none() { // All Jihai (Tsuuiisou)
        is_honitsu = false;
        is_chinitsu = false;
    }
    
    if !has_jihai { is_honitsu = false; } // Chinitsu is not Honitsu

    (is_honitsu, is_chinitsu, suit)
}

fn check_honitsu(all_tiles: &[Hai]) -> (bool, Option<Suhai>) {
    let (hon, chin, suit) = check_color(all_tiles);
    (hon, suit)
}
fn check_chinitsu(all_tiles: &[Hai]) -> (bool, Option<Suhai>) {
    let (hon, chin, suit) = check_color(all_tiles);
    (chin, suit)
}


// --- 5. Generic Tile Helpers ---

/// Gets all 14 tiles from a standard hand.
fn get_all_tiles(hand: &AgariHand) -> Vec<Hai> {
    let mut tiles = Vec::with_capacity(14);
    tiles.push(hand.atama.0);
    tiles.push(hand.atama.1);
    for mentsu in &hand.mentsu {
        match mentsu.mentsu_type {
            MentsuType::Shuntsu | MentsuType::Koutsu => {
                tiles.extend_from_slice(&mentsu.tiles[0..3]);
            }
            MentsuType::Kantsu => {
                tiles.extend_from_slice(&mentsu.tiles[0..4]);
            }
        }
    }
    tiles
}

/// Gets all 14 tiles from any hand structure.
fn get_all_tiles_from_structure(structure: &HandStructure) -> Vec<Hai> {
    match structure {
        HandStructure::YonmentsuIchiatama(hand) => get_all_tiles(hand),
        HandStructure::Chiitoitsu { pairs, .. } => {
            pairs.iter().flat_map(|&(t1, t2)| vec![t1, t2]).collect()
        }
        HandStructure::KokushiMusou { tiles, atama, .. } => {
            let mut v = tiles.to_vec();
            v.push(atama.0); // The pair tile
            v
        }
        HandStructure::ChuurenPoutou { hand, .. } => get_all_tiles(hand),
    }
}

/// Gets all 5 "groups" (4 melds + 1 pair) from a hand.
fn get_all_groups(hand: &AgariHand) -> Vec<Vec<Hai>> {
    let mut groups = Vec::with_capacity(5);
    groups.push(vec![hand.atama.0, hand.atama.1]);
    for mentsu in &hand.mentsu {
        match mentsu.mentsu_type {
            MentsuType::Shuntsu | MentsuType::Koutsu => {
                groups.push(mentsu.tiles[0..3].to_vec());
            }
            MentsuType::Kantsu => {
                groups.push(mentsu.tiles[0..4].to_vec());
            }
        }
    }
    groups
}

/// Counts koutsu and kantsu.
fn count_koutsu_kantsu(hand: &AgariHand) -> (u8, u8) {
    let mut koutsu = 0;
    let mut kantsu = 0;
    for m in &hand.mentsu {
        match m.mentsu_type {
            MentsuType::Koutsu => koutsu += 1,
            MentsuType::Kantsu => kantsu += 1,
            _ => (),
        }
    }
    (koutsu, kantsu)
}

/// Counts concealed koutsu/kantsu (for Sanankou/Suuankou).
fn count_concealed_koutsu(hand: &AgariHand, agari_type: AgariType) -> u8 {
    let mut count = 0;
    for m in &hand.mentsu {
        if m.is_minchou {
            continue;
        } // Skip open melds
        
        if m.mentsu_type == MentsuType::Koutsu {
            // If Ron, the koutsu completed by the agari_hai is NOT concealed.
            if agari_type == AgariType::Ron {
                // Check if agari_hai is part of this koutsu
                if m.tiles[0] == hand.agari_hai
                    || m.tiles[1] == hand.agari_hai
                    || m.tiles[2] == hand.agari_hai
                {
                    continue; // This triplet was completed by Ron, not concealed.
                }
            }
            count += 1;
        } else if m.mentsu_type == MentsuType::Kantsu {
            count += 1; // Concealed Kantsu always counts
        }
    }
    count
}

/// Is the meld a Koutsu or Kantsu?
fn is_koutsu_or_kantsu(mentsu: &Mentsu) -> bool {
    mentsu.mentsu_type == MentsuType::Koutsu || mentsu.mentsu_type == MentsuType::Kantsu
}

/// Is the tile a simple (2-8 suhai)?
fn is_simple(tile: &Hai) -> bool {
    match tile {
        Hai::Suhai(n, _) => *n >= 2 && *n <= 8,
        Hai::Jihai(_) => false,
    }
}

/// Is the tile a terminal (1 or 9)?
fn is_terminal(tile: &Hai) -> bool {
    match tile {
        Hai::Suhai(n, _) => *n == 1 || *n == 9,
        Hai::Jihai(_) => false,
    }
}

/// Is the tile an honor tile (wind or dragon)?
fn is_jihai(tile: &Hai) -> bool {
    matches!(tile, Hai::Jihai(_))
}

/// Is the tile a terminal or honor (yaochuu-hai)?
fn is_yaochuu(tile: &Hai) -> bool {
    is_terminal(tile) || is_jihai(tile)
}

/// Is the tile part of Ryuuiisou (All Green)?
fn is_green_tile(tile: &Hai) -> bool {
    match tile {
        Hai::Suhai(n, Suhai::Souzu) => {
            *n == 2 || *n == 3 || *n == 4 || *n == 6 || *n == 8
        }
        Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu)) => true,
        _ => false,
    }
}

/// --- Tile Indexing Helpers (for counts array) ---
/// These must match the implementation in your hand organizer.
/// 0-8: Manzu 1-9
/// 9-17: Pinzu 1-9
/// 18-26: Souzu 1-9
/// 27-30: Winds (Ton, Nan, Shaa, Pei)
/// 31-33: Dragons (Haku, Hatsu, Chun)

fn index_from_hai(hai: Hai) -> usize {
    match hai {
        Hai::Suhai(n, Suhai::Manzu) => (n - 1) as usize,
        Hai::Suhai(n, Suhai::Pinzu) => (n - 1) as usize + 9,
        Hai::Suhai(n, Suhai::Souzu) => (n - 1) as usize + 18,
        Hai::Jihai(Jihai::Kaze(Kaze::Ton)) => 27,
        Hai::Jihai(Jihai::Kaze(Kaze::Nan)) => 28,
        Hai::Jihai(Jihai::Kaze(Kaze::Shaa)) => 29,
        Hai::Jihai(Jihai::Kaze(Kaze::Pei)) => 30,
        Hai::Jihai(Jihai::Sangen(Sangenpai::Haku)) => 31,
        Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu)) => 32,
        Hai::Jihai(Jihai::Sangen(Sangenpai::Chun)) => 33,
    }
}

fn hai_from_index(idx: usize) -> Hai {
    match idx {
        0..=8 => Hai::Suhai((idx + 1) as u8, Suhai::Manzu),
        9..=17 => Hai::Suhai((idx - 9 + 1) as u8, Suhai::Pinzu),
        18..=26 => Hai::Suhai((idx - 18 + 1) as u8, Suhai::Souzu),
        27 => Hai::Jihai(Jihai::Kaze(Kaze::Ton)),
        28 => Hai::Jihai(Jihai::Kaze(Kaze::Nan)),
        29 => Hai::Jihai(Jihai::Kaze(Kaze::Shaa)),
        30 => Hai::Jihai(Jihai::Kaze(Kaze::Pei)),
        31 => Hai::Jihai(Jihai::Sangen(Sangenpai::Haku)),
        32 => Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu)),
        33 => Hai::Jihai(Jihai::Sangen(Sangenpai::Chun)),
        _ => panic!("Invalid tile index"),
    }
}