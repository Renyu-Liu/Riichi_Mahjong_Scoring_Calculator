use crate::implements::game::AgariType;
use crate::implements::hand::MentsuType;
use crate::implements::input::OpenMeldInput;
use crate::implements::tiles::{Hai, Kaze};

#[derive(Debug, Clone)]
pub enum Message {
    // --- Composition Phase ---
    AddTile(Hai),
    RemoveTile(usize),
    ConfirmHand,
    CancelSelection,

    // --- Definition Phase ---
    ModifyHand,
    StartSelectWinningTile,
    SelectWinningTile(Hai),
    SelectMeldType(MentsuType),
    SelectCompleteMeld(OpenMeldInput),
    StartAddClosedKan,
    SelectClosedKan(Hai),
    RemoveOpenMeld(usize),
    RemoveClosedKan(usize),
    IncrementAkadora,
    DecrementAkadora,
    ToggleAgariType(AgariType),
    SetBakaze(Kaze),
    SetJikaze(Kaze),
    ToggleRiichi(bool),
    ToggleDoubleRiichi(bool),
    ToggleIppatsu(bool),
    ToggleRinshan(bool),
    ToggleChankan(bool),
    ToggleHaitei(bool),
    ToggleHoutei(bool),
    ToggleTenhou(bool),
    ToggleChiihou(bool),
    ToggleRenhou(bool),
    IncrementHonba,
    DecrementHonba,
    StartAddDora,
    SelectDora(Hai),
    StartAddUraDora,
    SelectUraDora(Hai),
    RemoveDora(usize),
    RemoveUraDora(usize),
    CalculateScore,

    // --- Result Phase ---
    StartOver,
    ShowRules,
    HideRules,
}
