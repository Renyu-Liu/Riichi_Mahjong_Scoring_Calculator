use crate::implements::types::{
    hand::{HandStructure, Machi},
    tiles::{Hai, index_to_tile, tile_to_index},
    yaku::Yaku,
};

pub fn check_kokushi(counts: &[u8; 34], agari_hai: Hai) -> Option<(HandStructure, Yaku)> {
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
            0 => {}
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
