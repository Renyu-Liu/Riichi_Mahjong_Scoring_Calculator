// Submodules containing phase-specific view logic
mod composition;
mod definition;
mod result;

use super::super::messages::Message;
use super::super::state::{Phase, RiichiGui};
use super::View;
use crate::implements::hand::MentsuType;
use iced::widget::{container, scrollable};
use iced::{Element, Length};

impl View for RiichiGui {
    fn view(&self) -> Element<'_, Message> {
        let content = match &self.phase {
            Phase::Composition => composition::build_composition_view(self),
            Phase::Definition => definition::build_definition_view(self),
            Phase::SelectingWinningTile => self.view_selecting_winning_tile(),
            Phase::SelectingMeldTile(m_type, _) => self.view_selecting_meld_tile(*m_type),
            Phase::SelectingClosedKan { .. } => self.view_selecting_closed_kan(),
            Phase::SelectingDora => self.view_selecting_dora(false),
            Phase::SelectingUraDora => self.view_selecting_dora(true),
            Phase::Result => result::build_result_view(self),
        };

        container(scrollable(container(content).width(Length::Fill).center_x()).width(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }

    fn view_composition(&self) -> Element<'_, Message> {
        composition::build_composition_view(self)
    }

    fn view_definition(&self) -> Element<'_, Message> {
        definition::build_definition_view(self)
    }

    fn view_result(&self) -> Element<'_, Message> {
        result::build_result_view(self)
    }

    fn view_selecting_winning_tile(&self) -> Element<'_, Message> {
        self.view_selecting_winning_tile()
    }

    fn view_selecting_meld_tile(&self, m_type: MentsuType) -> Element<'_, Message> {
        self.view_selecting_meld_tile(m_type)
    }

    fn view_selecting_closed_kan(&self) -> Element<'_, Message> {
        self.view_selecting_closed_kan()
    }

    fn view_selecting_dora(&self, is_ura: bool) -> Element<'_, Message> {
        self.view_selecting_dora(is_ura)
    }

    fn view_hand_preview(&self) -> Element<'_, Message> {
        self.view_hand_preview()
    }

    fn view_hand_preview_locked(&self) -> Element<'_, Message> {
        self.view_hand_preview_locked()
    }

    fn view_tile_pool(&self) -> Element<'_, Message> {
        self.view_tile_pool()
    }
}
