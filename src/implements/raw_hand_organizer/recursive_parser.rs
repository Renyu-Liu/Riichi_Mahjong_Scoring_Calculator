use crate::implements::types::{
    hand::{Mentsu, MentsuType},
    tiles::index_to_tile,
};

pub fn find_all_mentsu_recursive(
    counts: &mut [u8; 34],
    mentsu: &mut Vec<Mentsu>,
    results: &mut Vec<Vec<Mentsu>>,
) {
    let mut i = 0;
    while i < 34 && counts[i] == 0 {
        i += 1;
    }
    if i == 34 {
        results.push(mentsu.clone());
        return;
    }

    // Find Koutsu
    if counts[i] >= 3 {
        let tile = index_to_tile(i);
        counts[i] -= 3;
        mentsu.push(Mentsu {
            mentsu_type: MentsuType::Koutsu,
            is_minchou: false,
            tiles: [tile, tile, tile, tile],
        });

        find_all_mentsu_recursive(counts, mentsu, results);

        mentsu.pop();
        counts[i] += 3;
    }

    // Find Shuntsu
    if i < 27 && (i % 9) < 7 && counts[i] > 0 && counts[i + 1] > 0 && counts[i + 2] > 0 {
        let tile1 = index_to_tile(i);
        let tile2 = index_to_tile(i + 1);
        let tile3 = index_to_tile(i + 2);

        counts[i] -= 1;
        counts[i + 1] -= 1;
        counts[i + 2] -= 1;
        mentsu.push(Mentsu {
            mentsu_type: MentsuType::Shuntsu,
            is_minchou: false,
            tiles: [tile1, tile2, tile3, tile3],
        });

        find_all_mentsu_recursive(counts, mentsu, results);

        mentsu.pop();
        counts[i] += 1;
        counts[i + 1] += 1;
        counts[i + 2] += 1;
    }
}
