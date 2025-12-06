use crate::implements::types::yaku::Yaku;

pub fn count_yakuman(yaku_list: &[Yaku]) -> u32 {
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
