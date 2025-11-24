use crate::gui::messages::Message;
use crate::gui::state::RiichiGui;
use crate::gui::styles::ColoredButtonStyle;
use iced::widget::{button, column, text};
use iced::{Color, Element, Length, theme};

pub fn build_composition_view(gui: &RiichiGui) -> Element<'_, Message> {
    let hand_preview = gui.view_hand_preview();
    let tile_pool = gui.view_tile_pool();

    let tile_count = gui.hand_tiles.len();
    let counter_color = if tile_count < 14 {
        Color::from_rgb(0.8, 0.0, 0.0)
    } else {
        Color::from_rgb(0.0, 0.5, 0.0)
    };

    let counter_text = text(format!("Your Hand ({}/18 tiles)", tile_count))
        .size(20)
        .style(counter_color);

    let confirm_btn = button(text("Confirm Hand"))
        .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
            background_color: Color::from_rgb(0.0, 0.6, 0.0),
            text_color: Color::WHITE,
        })))
        .on_press_maybe(if tile_count >= 14 {
            Some(Message::ConfirmHand)
        } else {
            None
        });

    column![
        counter_text,
        hand_preview,
        confirm_btn,
        tile_pool,
        iced::widget::Space::with_height(Length::Fixed(100.0))
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center)
    .into()
}
