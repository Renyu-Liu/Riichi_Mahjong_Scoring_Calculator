mod phase_views;
mod selector_views;
mod utility_views;

use super::messages::Message;
use crate::implements::hand::MentsuType;
use iced::Element;

pub trait View {
    fn view(&self) -> Element<'_, Message>;
    fn view_composition(&self) -> Element<'_, Message>;
    fn view_definition(&self) -> Element<'_, Message>;
    fn view_result(&self) -> Element<'_, Message>;
    #[allow(dead_code)]
    fn view_selecting_winning_tile(&self) -> Element<'_, Message>;
    #[allow(dead_code)]
    fn view_selecting_meld_tile(&self, m_type: MentsuType) -> Element<'_, Message>;
    #[allow(dead_code)]
    fn view_selecting_closed_kan(&self) -> Element<'_, Message>;
    #[allow(dead_code)]
    fn view_selecting_dora(&self, is_ura: bool) -> Element<'_, Message>;
    #[allow(dead_code)]
    fn view_hand_preview(&self) -> Element<'_, Message>;
    #[allow(dead_code)]
    fn view_hand_preview_locked(&self) -> Element<'_, Message>;
    #[allow(dead_code)]
    fn view_tile_pool(&self) -> Element<'_, Message>;
}
