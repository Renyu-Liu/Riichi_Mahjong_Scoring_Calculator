use crate::implements::types::hand::MentsuType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phase {
    Composition,
    Definition,
    SelectingWinningTile,
    SelectingMeldTile(MentsuType),
    SelectingClosedKan,
    SelectingDora,
    SelectingUraDora,
    Result,
}

impl Default for Phase {
    fn default() -> Self {
        Phase::Composition
    }
}
