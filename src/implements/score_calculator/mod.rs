pub mod fu;
pub mod han;
pub mod points;
pub mod yakuman;

use self::{
    fu::calculate_fu,
    han::calculate_han,
    points::{calculate_basic_points, round_up_100},
    yakuman::count_yakuman,
};
use super::YakuResult;
use crate::implements::types::{
    game::{AgariType, GameContext, PlayerContext},
    scoring::{AgariResult, HandLimit},
};

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

    // Check Yakuman
    let num_yakuman = count_yakuman(&yaku_list);

    if num_yakuman > 0 {
        let han = 13 * num_yakuman as u8;
        let fu = 0;
        let limit_name = Some(HandLimit::Yakuman);
        let base_yakuman_points = (8000 * num_yakuman) as u32;

        let (oya_payment, ko_payment, total_payment) = match (player.is_oya, agari_type) {
            // Oya Tsumo
            (true, AgariType::Tsumo) => {
                let p = round_up_100(base_yakuman_points * 2);
                let total = (p + tsumo_bonus) * 3;
                (p, 0, total)
            }
            // Ko Tsumo
            (false, AgariType::Tsumo) => {
                let oya_p = round_up_100(base_yakuman_points * 2);
                let ko_p = round_up_100(base_yakuman_points * 1);
                let total = (oya_p + tsumo_bonus) + (ko_p + tsumo_bonus) * 2;
                (oya_p, ko_p, total)
            }
            // Oya Ron
            (true, AgariType::Ron) => {
                let total = round_up_100(base_yakuman_points * 6) + ron_bonus;
                (0, 0, total)
            }
            // Ko Ron
            (false, AgariType::Ron) => {
                let total = round_up_100(base_yakuman_points * 4) + ron_bonus;
                (0, 0, total)
            }
        };

        return AgariResult {
            han,
            fu,
            yaku_list,
            num_akadora: 0,
            limit_name,
            oya_payment,
            ko_payment,
            total_payment,
            honba: game.honba,
            agari_type,
            is_oya: player.is_oya,
        };
    }

    // Regular Hand
    let han = calculate_han(&yaku_list, player.is_menzen);
    let fu = calculate_fu(
        &yaku_result.hand_structure,
        &yaku_list,
        player,
        game,
        agari_type,
    );

    let (basic_points, limit_name) = calculate_basic_points(han, fu);

    let (oya_payment, ko_payment, total_payment) = match (player.is_oya, agari_type) {
        // Oya Tsumo
        (true, AgariType::Tsumo) => {
            let p = round_up_100(basic_points * 2);
            let total = (p + tsumo_bonus) * 3;
            (p, 0, total)
        }
        // Ko Tsumo
        (false, AgariType::Tsumo) => {
            let oya_p = round_up_100(basic_points * 2);
            let ko_p = round_up_100(basic_points * 1);
            let total = (oya_p + tsumo_bonus) + (ko_p + tsumo_bonus) * 2;
            (oya_p, ko_p, total)
        }
        // Oya Ron
        (true, AgariType::Ron) => {
            let total = round_up_100(basic_points * 6) + ron_bonus;
            (0, 0, total)
        }
        // Ko Ron
        (false, AgariType::Ron) => {
            let total = round_up_100(basic_points * 4) + ron_bonus;
            (0, 0, total)
        }
    };

    AgariResult {
        han,
        fu,
        yaku_list,
        num_akadora,
        limit_name,
        oya_payment,
        ko_payment,
        total_payment,
        honba: game.honba,
        agari_type,
        is_oya: player.is_oya,
    }
}
