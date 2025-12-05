// standard.rs: standard yaku checkers

use super::utils::*;
use crate::implements::types::{
    game::{AgariType, GameContext, PlayerContext},
    hand::{AgariHand, Machi, Mentsu, MentsuType},
    tiles::{Hai, Jihai, Sangenpai, Suhai, Suit},
    yaku::Yaku,
};
use std::collections::{HashMap, HashSet};

pub fn find_standard_yaku(
    hand: &AgariHand,
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> Vec<Yaku> {
    let mut yaku_list = Vec::new();

    // context-based Yaku
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

    // Sequence Yaku
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

    // Triplet Yaku
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

    // Terminal/Honor Yaku
    let all_tiles = get_all_tiles(hand);
    let all_groups = get_all_groups(hand);

    let is_honroutou =
        all_tiles.iter().all(|t| t.is_yaochuu()) && !all_tiles.iter().all(|t| t.is_terminal()); // Exclude Chinroutou

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

    if yaku_list.contains(&Yaku::Pinfu) {
        if yaku_list.contains(&Yaku::RinshanKaihou) || yaku_list.contains(&Yaku::Chankan) {
            yaku_list.retain(|&y| y != Yaku::Pinfu);
        }
    }

    yaku_list
}

pub fn find_chiitoitsu_yaku(
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

fn check_yakuhai(hand: &AgariHand, player: &PlayerContext, game: &GameContext) -> Vec<Yaku> {
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
        yaku.push(Yaku::YakuhaiJikaze);
    }

    yaku
}

fn check_pinfu(hand: &AgariHand, player: &PlayerContext, game: &GameContext) -> bool {
    // Kantsu check
    if hand
        .mentsu
        .iter()
        .any(|m| m.mentsu_type == MentsuType::Kantsu)
    {
        return false;
    }

    // menzen check
    if !player.is_menzen {
        return false;
    }
    // Shuntsu check
    if !hand
        .mentsu
        .iter()
        .all(|m| m.mentsu_type == MentsuType::Shuntsu)
    {
        return false;
    }
    // Yakuhai check
    if let Hai::Jihai(Jihai::Sangen(_)) = hand.atama.0 {
        return false;
    }
    if let Hai::Jihai(Jihai::Kaze(k)) = hand.atama.0 {
        if k == game.bakaze || k == player.jikaze {
            return false;
        }
    }
    // Ryanmen check
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

    let mut counts = HashMap::new();
    for m in shuntsu {
        *counts.entry(m.tiles[0]).or_insert(0) += 1;
    }

    let mut pairs = 0;
    for count in counts.values() {
        pairs += count / 2;
    }

    // (iipeikou, ryanpeikou)
    (pairs == 1, pairs == 2)
}

fn check_sanshoku_generic(mentsu_list: &[&Mentsu]) -> bool {
    if mentsu_list.len() < 3 {
        return false;
    }

    let mut starters: HashMap<u8, (bool, bool, bool)> = HashMap::new();

    for m in mentsu_list {
        if let Hai::Suhai(Suhai { number: n, suit: s }) = m.tiles[0] {
            let entry = starters.entry(n).or_insert((false, false, false));
            match s {
                Suit::Manzu => entry.0 = true,
                Suit::Pinzu => entry.1 = true,
                Suit::Souzu => entry.2 = true,
            }
        }
    }

    starters.values().any(|&(m, p, s)| m && p && s)
}

fn check_sanshoku_doujun<'a>(shuntsu: &[&'a Mentsu]) -> bool {
    check_sanshoku_generic(shuntsu)
}

fn check_ittsu<'a>(shuntsu: &[&'a Mentsu]) -> bool {
    if shuntsu.len() < 3 {
        return false;
    }

    let mut suits: HashMap<Suit, HashSet<u8>> = HashMap::new();

    for m in shuntsu {
        if let Hai::Suhai(Suhai { number: n, suit: s }) = m.tiles[0] {
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

    check_sanshoku_generic(&koutsu)
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

fn check_color(all_tiles: &[Hai]) -> (bool, bool, Option<Suit>) {
    let mut suit = None;
    let mut has_jihai = false;
    let mut is_honitsu = true;
    let mut is_chinitsu = true;

    for tile in all_tiles {
        match tile {
            Hai::Suhai(Suhai { suit: s, .. }) => {
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

pub fn check_honitsu(all_tiles: &[Hai]) -> (bool, Option<Suit>) {
    let (hon, _chin, suit) = check_color(all_tiles);
    (hon, suit)
}

pub fn check_chinitsu(all_tiles: &[Hai]) -> (bool, Option<Suit>) {
    let (_hon, chin, suit) = check_color(all_tiles);
    (chin, suit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::implements::hand::{Mentsu, MentsuType};
    use crate::implements::tiles::{Hai, Suhai, Suit};

    fn make_shuntsu(suit: Suit, start_num: u8) -> Mentsu {
        Mentsu {
            mentsu_type: MentsuType::Shuntsu,
            is_minchou: false,
            tiles: [
                Hai::Suhai(Suhai {
                    number: start_num,
                    suit,
                }),
                Hai::Suhai(Suhai {
                    number: start_num + 1,
                    suit,
                }),
                Hai::Suhai(Suhai {
                    number: start_num + 2,
                    suit,
                }),
                Hai::Suhai(Suhai {
                    number: 0,
                    suit: Suit::Manzu,
                }), // padding
            ],
        }
    }

    #[test]
    fn test_ryanpeikou_four_identical() {
        // 4 identical sequences -> 2 pairs -> Ryanpeikou
        let m1 = make_shuntsu(Suit::Manzu, 2);
        let m2 = make_shuntsu(Suit::Manzu, 2);
        let m3 = make_shuntsu(Suit::Manzu, 2);
        let m4 = make_shuntsu(Suit::Manzu, 2);

        let shuntsu = vec![&m1, &m2, &m3, &m4];
        let (iipeikou, ryanpeikou) = check_peikou(&shuntsu);

        assert_eq!(ryanpeikou, true, "Should be Ryanpeikou (4 identical)");
        assert_eq!(iipeikou, false, "Should not be just Iipeikou");
    }

    #[test]
    fn test_iipeikou_three_identical() {
        // 3 identical sequences -> 1 pair -> Iipeikou
        let m1 = make_shuntsu(Suit::Manzu, 2);
        let m2 = make_shuntsu(Suit::Manzu, 2);
        let m3 = make_shuntsu(Suit::Manzu, 2);

        let shuntsu = vec![&m1, &m2, &m3];
        let (iipeikou, ryanpeikou) = check_peikou(&shuntsu);

        assert_eq!(iipeikou, true, "Should be Iipeikou (3 identical)");
        assert_eq!(ryanpeikou, false);
    }

    #[test]
    fn test_ryanpeikou_two_pairs() {
        // 2 identical + 2 identical -> 2 pairs -> Ryanpeikou
        let m1 = make_shuntsu(Suit::Manzu, 2);
        let m2 = make_shuntsu(Suit::Manzu, 2);
        let m3 = make_shuntsu(Suit::Pinzu, 5);
        let m4 = make_shuntsu(Suit::Pinzu, 5);

        let shuntsu = vec![&m1, &m2, &m3, &m4];
        let (iipeikou, ryanpeikou) = check_peikou(&shuntsu);

        assert_eq!(ryanpeikou, true, "Should be Ryanpeikou (2 different pairs)");
        assert_eq!(iipeikou, false);
    }
}
