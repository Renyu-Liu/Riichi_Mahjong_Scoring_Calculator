// terminal_ver.rs: Not used anymore

mod implements;
use implements::game::{AgariType, GameContext, PlayerContext};
use implements::input::UserInput;
use implements::scoring::AgariResult;
use implements::tiles::{Hai, Kaze, Suhai};
use implements::*;

fn create_example_hand_input() -> UserInput {
    let hand_tiles = vec![
        Hai::Suhai(2, Suhai::Manzu),
        Hai::Suhai(3, Suhai::Manzu),
        Hai::Suhai(4, Suhai::Manzu),
        Hai::Suhai(5, Suhai::Manzu),
        Hai::Suhai(6, Suhai::Manzu),
        Hai::Suhai(7, Suhai::Manzu), 
        Hai::Suhai(3, Suhai::Pinzu), 
        Hai::Suhai(4, Suhai::Pinzu),
        Hai::Suhai(5, Suhai::Pinzu), 
        Hai::Suhai(6, Suhai::Pinzu),
        Hai::Suhai(7, Suhai::Pinzu),
        Hai::Suhai(8, Suhai::Pinzu), 
        Hai::Suhai(4, Suhai::Souzu),
        Hai::Suhai(4, Suhai::Souzu),
    ];

    let winning_tile = Hai::Suhai(8, Suhai::Pinzu);

    let open_melds = Vec::new();
    let closed_kans = Vec::new();

    let player_context = PlayerContext {
        jikaze: Kaze::Nan, 
        is_oya: false,     
        is_riichi: true,   
        is_daburu_riichi: false,
        is_ippatsu: false,
        is_menzen: true, 
    };

    let game_context = GameContext {
        bakaze: Kaze::Ton,                                    
        kyoku: 1,                                             
        honba: 1,                                              
        riichi_bou: 1,                                        
        dora_indicators: vec![Hai::Suhai(2, Suhai::Pinzu)],   
        uradora_indicators: vec![Hai::Suhai(6, Suhai::Manzu)], 
        num_akadora: 1,                                       

        is_tenhou: false,
        is_chiihou: false,
        is_renhou: false,
        is_haitei: false,
        is_houtei: false,
        is_rinshan: false,
        is_chankan: false,
    };

    let agari_type = AgariType::Tsumo;

    UserInput {
        hand_tiles,
        winning_tile,
        open_melds,
        closed_kans,
        player_context,
        game_context,
        agari_type,
    }
}

pub fn calculate_agari(input: &UserInput) -> Result<AgariResult, &'static str> {
    let player = &input.player_context;
    let game = &input.game_context;
    let agari_type = input.agari_type;

    let organization = organize_hand(input)?;

    let yaku_result = check_all_yaku(organization, player, game, agari_type)?;

    let final_score = calculate_score(yaku_result, player, game, agari_type);

    Ok(final_score)
}

fn main() {
    println!("--- Riichi Mahjong Score Calculator Example ---");

    let user_input = create_example_hand_input();

    let calculation_result = calculate_agari(&user_input);

    println!("\nCalculating score for hand...\n");

    match calculation_result {
        Ok(final_score) => {
            println!("{}", final_score);
        }
        Err(error_message) => {
            println!("!!! Error calculating score: {} !!!", error_message);
        }
    }
}
