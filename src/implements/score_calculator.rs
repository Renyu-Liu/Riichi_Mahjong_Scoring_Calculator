// score_calculator.rs: Calculates the final score

use super::types::{
    game::{AgariType, GameContext, PlayerContext},
    hand::{HandStructure, Machi, MentsuType},
    scoring::{AgariResult, HandLimit},
    tiles::{Hai, Jihai},
    yaku::Yaku,
};
use super::yaku_checker::YakuResult;


pub fn calculate_score(
    yaku_result: YakuResult,
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> AgariResult {
    let tsumo_bonus = game.honba as u32 * 100;
    let ron_bonus = game.honba as u32 * 300;
    let yaku_list = yaku_result.yaku_list;
    let num_akadora = yaku_result.num_akadora;

    // Check for Yakuman
    let num_yakuman = count_yakuman(&yaku_list);

    if num_yakuman > 0 {
        let han = 13 * num_yakuman as u8; 
        let fu = 0; 
        let limit_name = Some(HandLimit::Yakuman);
        let base_yakuman_points = (8000 * num_yakuman) as u32;

        let (base_points, oya_payment, ko_payment, total_payment) = match (player.is_oya, agari_type) {
            // Oya Tsumo
            (true, AgariType::Tsumo) => {
                let p = round_up_100(base_yakuman_points * 2);
                let total = (p + tsumo_bonus) * 3;
                (p, p, 0, total)
            }
            // Ko Tsumo
            (false, AgariType::Tsumo) => {
                let oya_p = round_up_100(base_yakuman_points * 2);
                let ko_p = round_up_100(base_yakuman_points * 1);
                let total = (oya_p + tsumo_bonus) + (ko_p + tsumo_bonus) * 2;
                (ko_p, oya_p, ko_p, total)
            }
            // Oya Ron
            (true, AgariType::Ron) => {
                let total = round_up_100(base_yakuman_points * 6) + ron_bonus;
                (total, 0, 0, total)
            }
            // Ko Ron
            (false, AgariType::Ron) => {
                let total = round_up_100(base_yakuman_points * 4) + ron_bonus;
                (total, 0, 0, total)
            }
        };

        return AgariResult {
            han,
            fu,
            yaku_list,
            num_akadora: 0, 
            limit_name,
            base_points,
            oya_payment,
            ko_payment,
            total_payment,
            honba: game.honba,
            agari_type,
            is_oya: player.is_oya,
        };
    }

    // Regular Hand Path 
    let han = calculate_han(&yaku_list, player.is_menzen);
    let fu = calculate_fu(
        &yaku_result.hand_structure,
        &yaku_list,
        player,
        game,
        agari_type,
    );

    let (basic_points, limit_name) = calculate_basic_points(han, fu);

    // Calculate Final Payments
    let (base_points, oya_payment, ko_payment, total_payment) = match (player.is_oya, agari_type) {
        // Oya Tsumo
        (true, AgariType::Tsumo) => {
            let p = round_up_100(basic_points * 2);
            let total = (p + tsumo_bonus) * 3;
            (p, p, 0, total)
        }
        // Ko Tsumo
        (false, AgariType::Tsumo) => {
            let oya_p = round_up_100(basic_points * 2);
            let ko_p = round_up_100(basic_points * 1);
            let total = (oya_p + tsumo_bonus) + (ko_p + tsumo_bonus) * 2;
            (ko_p, oya_p, ko_p, total)
        }
        // Oya Ron
        (true, AgariType::Ron) => {
            let total = round_up_100(basic_points * 6) + ron_bonus;
            (total, 0, 0, total)
        }
        // Ko Ron
        (false, AgariType::Ron) => {
            let total = round_up_100(basic_points * 4) + ron_bonus;
            (total, 0, 0, total)
        }
    };

    AgariResult {
        han,
        fu,
        yaku_list,
        num_akadora, 
        limit_name,
        base_points,
        oya_payment,
        ko_payment,
        total_payment,
        honba: game.honba,
        agari_type,
        is_oya: player.is_oya,
    }
}

// Helper Functions

fn calculate_han(yaku_list: &[Yaku], is_menzen: bool) -> u8 {
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
        Yaku::SanshokuDoujun => if is_menzen { 2 } else { 1 },
        Yaku::Ittsu => if is_menzen { 2 } else { 1 },
        Yaku::Chanta => if is_menzen { 2 } else { 1 },

        // 3 Han 
        Yaku::Ryanpeikou => 3,
        // Kuisagari (3 -> 2)
        Yaku::Junchan => if is_menzen { 3 } else { 2 },
        Yaku::Honitsu => if is_menzen { 3 } else { 2 },

        // 6 Han
        // Kuisagari (6 -> 5)
        Yaku::Chinitsu => if is_menzen { 6 } else { 5 },

        // Dora
        Yaku::Dora => 1,
        Yaku::UraDora => 1,
        Yaku::AkaDora => 1,

        _ => 0,
    }
}

fn calculate_fu(
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
        return if agari_type == AgariType::Tsumo { 20 } else { 30 };
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
                    (true, false) => 8,  // Open simple quad
                    (true, true) => 16,  // Open terminal/honor quad
                    (false, false) => 16, // Concealed simple quad
                    (false, true) => 32, // Concealed terminal/honor quad
                };
            }
            MentsuType::Shuntsu => {} // Sequences are 0 fu
        }
    }

    // Pair 
    fu += get_pair_fu(&hand.atama.0, player, game);

    // Wait 
    match hand.machi {
        Machi::Kanchan | Machi::Penchan | Machi::Tanki => fu += 2,
        _ => {} // Ryanmen and Shanpon are 0 fu
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

fn calculate_basic_points(han: u8, fu: u8) -> (u32, Option<HandLimit>) {
    // Kazoe Yakuman
    if han >= 13 {
        return (8000, Some(HandLimit::Yakuman));
    }
    // Sanbaiman
    if han >= 11 {
        return (6000, Some(HandLimit::Sanbaiman));
    }
    // Baiman
    if han >= 8 {
        return (4000, Some(HandLimit::Baiman));
    }
    // Haneman
    if han >= 6 {
        return (3000, Some(HandLimit::Haneman));
    }
    // Mangan
    if han == 5 {
        return (2000, Some(HandLimit::Mangan));
    }

    // Below Mangan
    let basic_points = (fu as u32) * (1 << (han + 2));

    // kiriage Mangan (cap at 2000)
    if basic_points >= 2000 {
        (2000, Some(HandLimit::Mangan))
    } else {
        (basic_points, None)
    }
}

fn count_yakuman(yaku_list: &[Yaku]) -> u32 {
    yaku_list
        .iter()
        .map(|yaku| match yaku {
            // Double Yakuman
            Yaku::SuuankouTanki => 2,
            Yaku::KokushiMusouJusanmen => 2,
            Yaku::JunseiChuurenPoutou => 2,
            // Single Yakuman
            Yaku::Tenhou => 1,
            Yaku::Chiihou => 1,
            Yaku::Renhou => 1,
            Yaku::Daisangen => 1,
            Yaku::Suuankou => 1,
            Yaku::Daisuushi => 1,
            Yaku::Shousuushi => 1,
            Yaku::Tsuuiisou => 1,
            Yaku::Chinroutou => 1,
            Yaku::Ryuuiisou => 1,
            Yaku::Suukantsu => 1,
            Yaku::KokushiMusou => 1,
            Yaku::ChuurenPoutou => 1,
            _ => 0,
        })
        .sum()
}

fn round_up_100(n: u32) -> u32 {
    (n + 99) / 100 * 100
}