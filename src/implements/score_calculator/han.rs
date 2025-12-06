use crate::implements::types::yaku::Yaku;

pub fn calculate_han(yaku_list: &[Yaku], is_menzen: bool) -> u8 {
    yaku_list
        .iter()
        .map(|yaku| get_han_value(yaku, is_menzen))
        .sum()
}

fn get_han_value(yaku: &Yaku, is_menzen: bool) -> u8 {
    match yaku {
        // 1 Han
        Yaku::Riichi => 1,
        Yaku::Ippatsu => 1,
        Yaku::MenzenTsumo => 1,
        Yaku::Pinfu => 1,
        Yaku::Iipeikou => 1,
        Yaku::HaiteiRaoyue => 1,
        Yaku::HouteiRaoyui => 1,
        Yaku::RinshanKaihou => 1,
        Yaku::Chankan => 1,
        Yaku::Tanyao => 1,
        Yaku::YakuhaiJikaze => 1,
        Yaku::YakuhaiBakaze => 1,
        Yaku::YakuhaiSangenpai => 1,

        // 2 Han
        Yaku::DaburuRiichi => 2,
        Yaku::Chiitoitsu => 2,
        Yaku::Toitoi => 2,
        Yaku::Sanankou => 2,
        Yaku::SanshokuDoukou => 2,
        Yaku::Sankantsu => 2,
        Yaku::Shousangen => 2,
        Yaku::Honroutou => 2,
        // Kuisagari (2 -> 1)
        Yaku::SanshokuDoujun => {
            if is_menzen {
                2
            } else {
                1
            }
        }
        Yaku::Ittsu => {
            if is_menzen {
                2
            } else {
                1
            }
        }
        Yaku::Chanta => {
            if is_menzen {
                2
            } else {
                1
            }
        }

        // 3 Han
        Yaku::Ryanpeikou => 3,
        // Kuisagari (3 -> 2)
        Yaku::Junchan => {
            if is_menzen {
                3
            } else {
                2
            }
        }
        Yaku::Honitsu => {
            if is_menzen {
                3
            } else {
                2
            }
        }

        // 6 Han
        // Kuisagari (6 -> 5)
        Yaku::Chinitsu => {
            if is_menzen {
                6
            } else {
                5
            }
        }

        // Dora
        Yaku::Dora => 1,
        Yaku::UraDora => 1,
        Yaku::AkaDora => 1,

        _ => 0,
    }
}
