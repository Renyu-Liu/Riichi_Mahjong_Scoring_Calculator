// yaku_checker.rs: Check for yaku

use super::types::{
    game::{AgariType, GameContext, PlayerContext},
    hand::{AgariHand, HandOrganization, HandStructure, Machi, Mentsu, MentsuType},
    tiles::{index_to_tile, tile_to_index, Hai, Jihai, Kaze, Sangenpai, Suhai},
    yaku::Yaku,
};
use std::collections::{HashMap, HashSet};

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
            num_akadora: 0, // Akadora don't count for Yakuman
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
    let has_yaku =
        !regular_yaku.is_empty() || player.is_riichi || player.is_daburu_riichi;

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

fn check_game_state_yakuman(_player: &PlayerContext, game: &GameContext) -> Vec<Yaku> {
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

/// Find hand-based Yakuman
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
                Err("Invalid irregular hand. Not Kokushi or Chiitoitsu.")
            }
        }
    }
}

fn check_standard_yakuman(
    hand: &AgariHand,
    _player: &PlayerContext,
    _game: &GameContext, 
    agari_type: AgariType,
) -> (Vec<Yaku>, Option<bool>) {
    let mut yakuman = Vec::new();
    let all_tiles = get_all_tiles(hand);

    // Tile-based Yakuman 
    let mut is_tsuuiisou = true;
    let mut is_chinroutou = true;
    let mut is_ryuuiisou = true;

    for tile in &all_tiles {
        if !tile.is_jihai() {
            is_tsuuiisou = false;
        }
        if !tile.is_terminal() {
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
        yakuman.push(Yaku::Chinroutou);
    }
    if is_ryuuiisou {
        yakuman.push(Yaku::Ryuuiisou);
    }

    //  Meld-based Yakuman 
    let (_koutsu, kantsu) = count_koutsu_kantsu(hand);
    let concealed_koutsu = count_concealed_koutsu(hand, agari_type);

    // Suukantsu 
    if kantsu == 4 {
        yakuman.push(Yaku::Suukantsu);
    }

    // Suuankou 
    if concealed_koutsu == 4 {
        if hand.machi == Machi::Tanki {
            yakuman.push(Yaku::SuuankouTanki);
        } else {
            yakuman.push(Yaku::Suuankou);
        }
    }

    // Daisangen
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

    // Daisuushi / Shousuushi 
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

    // Chuuren Poutou 
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

/// Kokushi Musou 
fn check_kokushi(counts: &[u8; 34], agari_hai: Hai) -> Option<(HandStructure, Yaku)> {
    let mut has_pair = false;
    let mut tiles = Vec::new();
    let mut atama_tile = None;

    for (idx, &count) in counts.iter().enumerate() {
        let tile = index_to_tile(idx);
        if !tile.is_yaochuu() {
            if count > 0 {
                return None; 
            }
            continue;
        }

        match count {
            1 => {
                tiles.push(tile);
            }
            2 => {
                if has_pair {
                    return None;
                } 
                has_pair = true;
                atama_tile = Some(tile);
                tiles.push(tile); 
            }
            0 => {
            }
            _ => return None, 
        }
    }

    if !has_pair {
        return None;
    }

    let agari_hai_index = tile_to_index(&agari_hai);
    if counts[agari_hai_index] == 0 {
        return None; 
    }

    let atama = (atama_tile.unwrap(), atama_tile.unwrap());

    // 13-sided wait 
    let mut yaku = Yaku::KokushiMusou;
    let mut final_machi = Machi::KokushiIchimen;

    if atama.0 == agari_hai {
        if tiles.len() == 13 {
            yaku = Yaku::KokushiMusouJusanmen;
            final_machi = Machi::KokushiJusanmen;
        }
    }

    Some((
        HandStructure::KokushiMusou {
            tiles: tiles.try_into().ok()?,
            atama,
            _agari_hai: agari_hai,
            _machi: final_machi,
        },
        yaku,
    ))
}

fn check_chiitoitsu(counts: &[u8; 34], agari_hai: Hai) -> Option<HandStructure> {
    let mut pair_count = 0;
    let mut pairs = Vec::new();

    for (idx, &count) in counts.iter().enumerate() {
        if count > 0 {
            if count == 2 {
                pair_count += 1;
                let tile = index_to_tile(idx);
                pairs.push((tile, tile));
            } else if count == 4 {
                pair_count += 2;
                let tile = index_to_tile(idx);
                pairs.push((tile, tile));
                pairs.push((tile, tile));
            } else {
                return None; 
            }
        }
    }

    if pair_count == 7 {
        Some(HandStructure::Chiitoitsu {
            pairs: pairs.try_into().ok()?,
            agari_hai,
            machi: Machi::Tanki, 
        })
    } else {
        None
    }
}

/// Tsuuiisou
fn check_chiitoitsu_yakuman(hand: &HandStructure) -> Vec<Yaku> {
    if let HandStructure::Chiitoitsu { pairs, .. } = hand {
        let mut is_tsuuiisou = true;
        for (tile, _) in pairs {
            if !tile.is_jihai() {
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

/// Chuuren Poutou 
fn check_chuuren(hand: &AgariHand) -> Option<bool> {
    let all_tiles = get_all_tiles(hand);

    let (is_chinitsu, suit) = check_chinitsu(&all_tiles);
    if !is_chinitsu {
        return None;
    }
    let suit = suit.unwrap(); 

    if !hand.mentsu.iter().all(|m| !m.is_minchou) {
        return None;
    }

    let mut counts = [0u8; 9];
    for tile in &all_tiles {
        if let Hai::Suhai(n, s) = tile {
            if *s == suit {
                counts[(n - 1) as usize] += 1;
            }
        }
    }

    let mut has_extra = false;
    let mut extra_tile_num = 0;

    for (i, &count) in counts.iter().enumerate() {
        let num = i + 1;
        let required_count = if num == 1 || num == 9 { 3 } else { 1 };

        if count < required_count {
            return None;
        }
        if count == required_count + 1 {
            if has_extra {
                return None;
            } 
            has_extra = true;
            extra_tile_num = num;
        } else if count > required_count + 1 {
            return None; 
        }
    }

    if !has_extra {
        return None;
    } 
    if let Hai::Suhai(n, s) = hand.agari_hai {
        if s == suit && n as usize == extra_tile_num {
            return Some(true);
        }
    }

    Some(false) 
}

/// Double Yakuman overrides
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

// Regular Yaku

fn find_standard_yaku(
    hand: &AgariHand,
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> Vec<Yaku> {
    let mut yaku_list = Vec::new();

    // State-based Yaku 
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

    // Yakuhai 
    yaku_list.extend(check_yakuhai(hand, player, game));

    // Pinfu 
    if check_pinfu(hand, player, game) {
        yaku_list.push(Yaku::Pinfu);
    }

    // Tanyao
    if check_tanyao(hand) {
        yaku_list.push(Yaku::Tanyao);
    }

    //  Sequence Yaku 
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

    //  Triplet Yaku
    let (koutsu, kantsu) = count_koutsu_kantsu(hand);

    if koutsu + kantsu == 4 {
        yaku_list.push(Yaku::Toitoi);
    } else {
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

    //  Terminal/Honor Yaku 
    let all_tiles = get_all_tiles(hand);
    let all_groups = get_all_groups(hand);

    let is_honroutou = all_tiles.iter().all(|t| t.is_yaochuu())
        && !all_tiles.iter().all(|t| t.is_terminal()); // Exclude Chinroutou

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

    //  Color Yaku 
    let (is_chinitsu, _) = check_chinitsu(&all_tiles);
    if is_chinitsu {
        yaku_list.push(Yaku::Chinitsu);
    } else {
        let (is_honitsu, _) = check_honitsu(&all_tiles);
        if is_honitsu {
            yaku_list.push(Yaku::Honitsu);
        }
    }

    if yaku_list.contains(&Yaku::Pinfu) {
        if yaku_list.contains(&Yaku::RinshanKaihou)
            || yaku_list.contains(&Yaku::Chankan)
        {
            yaku_list.retain(|&y| y != Yaku::Pinfu);
        }
    }

    yaku_list
}

fn find_chiitoitsu_yaku(
    pairs: &[(Hai, Hai); 7],
    _agari_hai: &Hai,
    _machi: &Machi,
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> Vec<Yaku> {
    let mut yaku_list = Vec::new();

    yaku_list.push(Yaku::Chiitoitsu);

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

    // Tile-based Yaku 
    let all_tiles: Vec<Hai> = pairs.iter().flat_map(|&(t1, t2)| vec![t1, t2]).collect();

    if all_tiles.iter().all(|t| t.is_simple()) {
        yaku_list.push(Yaku::Tanyao);
    }

    // Honroutou 
    if all_tiles.iter().all(|t| t.is_yaochuu()) {
        yaku_list.push(Yaku::Honroutou);
    }

    // Color Yaku 
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

//  Dora

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

//  Helper Functions 

fn check_yakuhai(
    hand: &AgariHand,
    player: &PlayerContext,
    game: &GameContext,
) -> Vec<Yaku> {
    let mut yaku = Vec::new();

    let koutsu_tiles: HashSet<Hai> = hand
        .mentsu
        .iter()
        .filter(|m| is_koutsu_or_kantsu(m))
        .map(|m| m.tiles[0])
        .collect();

    // Dragons
    if koutsu_tiles.contains(&Hai::Jihai(Jihai::Sangen(Sangenpai::Haku))) {
        yaku.push(Yaku::YakuhaiSangenpai);
    }
    if koutsu_tiles.contains(&Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu))) {
        yaku.push(Yaku::YakuhaiSangenpai);
    }
    if koutsu_tiles.contains(&Hai::Jihai(Jihai::Sangen(Sangenpai::Chun))) {
        yaku.push(Yaku::YakuhaiSangenpai);
    }

    // Winds
    let bakaze_hai = Hai::Jihai(Jihai::Kaze(game.bakaze));
    if koutsu_tiles.contains(&bakaze_hai) {
        yaku.push(Yaku::YakuhaiBakaze);
    }

    let jikaze_hai = Hai::Jihai(Jihai::Kaze(player.jikaze));
    if koutsu_tiles.contains(&jikaze_hai) && jikaze_hai != bakaze_hai {
        yaku.push(Yaku::YakuhaiJikaze);
    } else if koutsu_tiles.contains(&jikaze_hai) && jikaze_hai == bakaze_hai {
        // This is correct, as it counts for 2 han.
        yaku.push(Yaku::YakuhaiJikaze);
    }

    yaku
}

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

    true
}

fn check_tanyao(hand: &AgariHand) -> bool {
    get_all_tiles(hand).iter().all(|t| t.is_simple())
}

fn check_peikou<'a>(shuntsu: &[&'a Mentsu]) -> (bool, bool) {
    if shuntsu.len() < 2 {
        return (false, false);
    }

    let mut identical_pairs = 0;
    let mut seen = HashSet::new();

    for (i, m1) in shuntsu.iter().enumerate() {
        if seen.contains(&i) {
            continue;
        }
        for (j, m2) in shuntsu.iter().enumerate() {
            if i == j || seen.contains(&j) {
                continue;
            }

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

fn check_sanshoku_doujun<'a>(shuntsu: &[&'a Mentsu]) -> bool {
    if shuntsu.len() < 3 {
        return false;
    }

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

fn check_ittsu<'a>(shuntsu: &[&'a Mentsu]) -> bool {
    if shuntsu.len() < 3 {
        return false;
    }

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

fn check_sanshoku_doukou(hand: &AgariHand) -> bool {
    let koutsu: Vec<&Mentsu> = hand
        .mentsu
        .iter()
        .filter(|m| is_koutsu_or_kantsu(m))
        .collect();

    if koutsu.len() < 3 {
        return false;
    }

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

fn check_chanta_junchan(groups: &[Vec<Hai>]) -> (bool, bool) {
    let mut is_chanta = true;
    let mut is_junchan = true;

    for group in groups {
        let mut has_terminal = false;
        let mut has_jihai = false;

        for tile in group {
            if tile.is_jihai() {
                has_jihai = true;
            }
            if tile.is_terminal() {
                has_terminal = true;
            }
        }

        if !has_terminal && !has_jihai {
            is_chanta = false;
            is_junchan = false;
            break;
        }

        if has_jihai {
            is_junchan = false; 
        }
    }

    (is_chanta, is_junchan && is_chanta)
}

fn check_color(all_tiles: &[Hai]) -> (bool, bool, Option<Suhai>) {
    let mut suit = None;
    let mut has_jihai = false;
    let mut is_honitsu = true;
    let mut is_chinitsu = true;

    for tile in all_tiles {
        match tile {
            Hai::Suhai(_n, s) => {
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

    if !has_jihai && suit.is_none() {
        is_honitsu = false;
        is_chinitsu = false;
    }

    if !has_jihai {
        is_honitsu = false;
    } 

    (is_honitsu, is_chinitsu, suit)
}

fn check_honitsu(all_tiles: &[Hai]) -> (bool, Option<Suhai>) {
    let (hon, _chin, suit) = check_color(all_tiles);
    (hon, suit)
}
fn check_chinitsu(all_tiles: &[Hai]) -> (bool, Option<Suhai>) {
    let (_hon, chin, suit) = check_color(all_tiles);
    (chin, suit)
}

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

fn get_all_tiles_from_structure(structure: &HandStructure) -> Vec<Hai> {
    match structure {
        HandStructure::YonmentsuIchiatama(hand) => get_all_tiles(hand),
        HandStructure::Chiitoitsu { pairs, .. } => {
            pairs.iter().flat_map(|&(t1, t2)| vec![t1, t2]).collect()
        }
        HandStructure::KokushiMusou { tiles, atama, .. } => {
            let mut v = tiles.to_vec();
            v.push(atama.0); 
            v
        }
        HandStructure::ChuurenPoutou { hand, .. } => get_all_tiles(hand),
    }
}

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

fn count_concealed_koutsu(hand: &AgariHand, agari_type: AgariType) -> u8 {
    let mut count = 0;
    for m in &hand.mentsu {
        if m.is_minchou {
            continue;
        } 

        if m.mentsu_type == MentsuType::Koutsu {
            if agari_type == AgariType::Ron {
                if m.tiles[0] == hand.agari_hai {
                    continue; // This triplet was completed by Ron, not concealed.
                }
            }
            count += 1;
        } else if m.mentsu_type == MentsuType::Kantsu {
            count += 1; 
        }
    }
    count
}

fn is_koutsu_or_kantsu(mentsu: &Mentsu) -> bool {
    mentsu.mentsu_type == MentsuType::Koutsu || mentsu.mentsu_type == MentsuType::Kantsu
}

fn is_green_tile(tile: &Hai) -> bool {
    match tile {
        Hai::Suhai(n, Suhai::Souzu) => {
            *n == 2 || *n == 3 || *n == 4 || *n == 6 || *n == 8
        }
        Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu)) => true,
        _ => false,
    }
}