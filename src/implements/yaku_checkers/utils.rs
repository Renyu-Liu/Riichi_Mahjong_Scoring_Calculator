use crate::implements::types::{
    game::AgariType,
    hand::{AgariHand, HandStructure, Machi, Mentsu, MentsuType},
    tiles::{Hai, Jihai, Kaze, Sangenpai, Suhai, index_to_tile},
};

pub fn check_chiitoitsu(counts: &[u8; 34], agari_hai: Hai) -> Option<HandStructure> {
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

pub fn count_dora(all_tiles: &[Hai], indicators: &[Hai]) -> u8 {
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

pub fn get_dora_tile(indicator: &Hai) -> Hai {
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

pub fn get_all_tiles(hand: &AgariHand) -> Vec<Hai> {
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

pub fn get_all_tiles_from_structure(structure: &HandStructure) -> Vec<Hai> {
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

pub fn get_all_groups(hand: &AgariHand) -> Vec<Vec<Hai>> {
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

pub fn count_koutsu_kantsu(hand: &AgariHand) -> (u8, u8) {
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

pub fn count_concealed_koutsu(hand: &AgariHand, agari_type: AgariType) -> u8 {
    let mut count = 0;
    for m in &hand.mentsu {
        if m.is_minchou {
            continue;
        }

        if m.mentsu_type == MentsuType::Koutsu {
            if agari_type == AgariType::Ron {
                if m.tiles[0] == hand.agari_hai {
                    continue;
                }
            }
            count += 1;
        } else if m.mentsu_type == MentsuType::Kantsu {
            count += 1;
        }
    }
    count
}

pub fn is_koutsu_or_kantsu(mentsu: &Mentsu) -> bool {
    mentsu.mentsu_type == MentsuType::Koutsu || mentsu.mentsu_type == MentsuType::Kantsu
}

pub fn is_green_tile(tile: &Hai) -> bool {
    match tile {
        Hai::Suhai(n, Suhai::Souzu) => *n == 2 || *n == 3 || *n == 4 || *n == 6 || *n == 8,
        Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu)) => true,
        _ => false,
    }
}
