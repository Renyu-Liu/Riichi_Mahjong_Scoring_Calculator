use super::game::{AgariType, GameContext, PlayerContext};
use super::hand::MentsuType;
use super::tiles::Hai;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenMeldInput {
    // type of meld
    pub mentsu_type: MentsuType,

    // single tile that uniquely represents the meld
    pub representative_tile: Hai,
}

#[derive(Debug, Clone)]
pub struct UserInput {
    // Hand Composition
    pub hand_tiles: Vec<Hai>,

    pub winning_tile: Hai,

    // list of all open melds
    pub open_melds: Vec<OpenMeldInput>,

    // list of all closed kans
    pub closed_kans: Vec<Hai>,

    pub player_context: PlayerContext,
    pub game_context: GameContext,
    pub agari_type: AgariType,
}
