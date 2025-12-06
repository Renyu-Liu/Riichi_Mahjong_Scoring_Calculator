use crate::implements::types::scoring::HandLimit;

pub fn calculate_basic_points(han: u8, fu: u8) -> (u32, Option<HandLimit>) {
    if han >= 13 {
        return (8000, Some(HandLimit::Yakuman));
    }
    if han >= 11 {
        return (6000, Some(HandLimit::Sanbaiman));
    }
    if han >= 8 {
        return (4000, Some(HandLimit::Baiman));
    }
    if han >= 6 {
        return (3000, Some(HandLimit::Haneman));
    }
    if han == 5 {
        return (2000, Some(HandLimit::Mangan));
    }

    // Below Mangan
    let basic_points = (fu as u32) * (1 << (han + 2));

    // kiriage Mangan
    if basic_points >= 2000 {
        (2000, Some(HandLimit::Mangan))
    } else {
        (basic_points, None)
    }
}

pub fn round_up_100(n: u32) -> u32 {
    (n + 99) / 100 * 100
}
