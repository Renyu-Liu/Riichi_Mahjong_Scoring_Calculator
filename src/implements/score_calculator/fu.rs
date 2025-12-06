use crate::implements::types::{
    game::{AgariType, GameContext, PlayerContext},
    hand::{HandStructure, Machi, MentsuType},
    tiles::{Hai, Jihai},
    yaku::Yaku,
};

pub fn calculate_fu(
    hand_structure: &HandStructure,
    yaku_list: &[Yaku],
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> u8 {
    // Chiitoitsu
    if yaku_list.contains(&Yaku::Chiitoitsu) {
        return 25;
    }

    // Pinfu
    if yaku_list.contains(&Yaku::Pinfu) {
        return if agari_type == AgariType::Tsumo {
            20
        } else {
            30
        };
    }

    // Standard Fu
    let mut fu = 20;

    let hand = match hand_structure {
        HandStructure::YonmentsuIchiatama(h) => h,
        HandStructure::ChuurenPoutou { hand, .. } => hand,
        HandStructure::Chiitoitsu { .. } => return 25,
        HandStructure::KokushiMusou { .. } => return 0,
    };

    // Agari Type
    if agari_type == AgariType::Tsumo {
        fu += 2; // Tsumo fu
    } else if player.is_menzen {
        fu += 10; // Menzen Ron fu
    }

    // Melds
    for mentsu in &hand.mentsu {
        let is_open = mentsu.is_minchou;
        let is_yaochuu = mentsu.tiles[0].is_yaochuu();

        match mentsu.mentsu_type {
            MentsuType::Koutsu => {
                fu += match (is_open, is_yaochuu) {
                    (true, false) => 2,  // Open simple triplet
                    (true, true) => 4,   // Open terminal/honor triplet
                    (false, false) => 4, // Concealed simple triplet
                    (false, true) => 8,  // Concealed terminal/honor triplet
                };
            }
            MentsuType::Kantsu => {
                fu += match (is_open, is_yaochuu) {
                    (true, false) => 8,   // Open simple quad
                    (true, true) => 16,   // Open terminal/honor quad
                    (false, false) => 16, // Concealed simple quad
                    (false, true) => 32,  // Concealed terminal/honor quad
                };
            }
            MentsuType::Shuntsu => {}
        }
    }

    // Pair
    fu += get_pair_fu(&hand.atama.0, player, game);

    // Wait
    match hand.machi {
        Machi::Kanchan | Machi::Penchan | Machi::Tanki => fu += 2,
        _ => {}
    }

    (((fu + 9) / 10) * 10) as u8
}

fn get_pair_fu(tile: &Hai, player: &PlayerContext, game: &GameContext) -> u32 {
    match tile {
        // Dragon Pair
        Hai::Jihai(Jihai::Sangen(_)) => 2,
        // Wind Pair
        Hai::Jihai(Jihai::Kaze(k)) => {
            let mut fu = 0;
            if *k == game.bakaze {
                fu += 2; // Prevalent Wind
            }
            if *k == player.jikaze {
                fu += 2; // Seat Wind
            }
            fu
        }
        _ => 0,
    }
}
