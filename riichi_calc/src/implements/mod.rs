pub mod types;
pub use types::*;
pub mod raw_hand_organizer;
pub use raw_hand_organizer::*;
pub mod yaku_checker;
pub use yaku_checker::*;
pub mod score_calculator;
pub use score_calculator::*;

use crate::implements::input::UserInput;
use crate::implements::scoring::AgariResult;

pub fn calculate_agari(input: &UserInput) -> Result<AgariResult, &'static str> {
    // Get input
    let player = &input.player_context;
    let game = &input.game_context;
    let agari_type = input.agari_type;

    // Organize Hand
    let organization = organize_hand(input)?;

    // Check Yaku
    let yaku_result = check_all_yaku(organization, player, game, agari_type)?;

    // Calculate Final Score
    let final_score = calculate_score(yaku_result, player, game, agari_type);

    Ok(final_score)
}
