use super::super::components::get_tile_image_path;
use super::super::messages::Message;
use super::super::state::RiichiGui;
use super::super::styles::ColoredButtonStyle;
use iced::widget::{button, column, container, image, row, text};
use iced::{Color, Element, Length, theme};

impl RiichiGui {
    pub fn view_hand_preview(&self) -> Element<'_, Message> {
        let tiles: Vec<Element<Message>> = self
            .hand_tiles
            .iter()
            .enumerate()
            .map(|(i, tile)| {
                let image_path = get_tile_image_path(tile);
                button(
                    iced::widget::Image::<iced::widget::image::Handle>::new(image_path).width(40),
                )
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::WHITE,
                    text_color: Color::BLACK,
                })))
                .on_press(Message::RemoveTile(i))
                .padding(0)
                .into()
            })
            .collect();

        container(row(tiles).spacing(5))
            .height(Length::Fixed(80.0))
            .align_y(iced::alignment::Vertical::Center)
            .into()
    }

    pub fn view_hand_preview_locked(&self) -> Element<'_, Message> {
        let tiles: Vec<Element<Message>> = self
            .hand_tiles
            .iter()
            .enumerate()
            .map(|(_, tile)| {
                let image_path = get_tile_image_path(tile);

                let btn = button(
                    iced::widget::Image::<iced::widget::image::Handle>::new(image_path).width(40),
                )
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::WHITE,
                    text_color: Color::BLACK,
                })))
                .padding(0);

                btn.into()
            })
            .collect();

        row(tiles).spacing(5).into()
    }

    pub fn view_tile_pool(&self) -> Element<'_, Message> {
        let mut tiles = Vec::new();

        for i in 0..34 {
            let tile = crate::implements::tiles::index_to_tile(i);
            let count = self.tile_counts[i];
            let image_path = get_tile_image_path(&tile);

            let tile_image = image(image_path).width(50);

            let count_text = text(format!("({})", count)).size(12).style(if count > 0 {
                Color::BLACK
            } else {
                Color::from_rgb(0.5, 0.5, 0.5)
            });

            let button_bg_color = if count > 0 {
                Color::WHITE
            } else {
                Color::from_rgb(0.85, 0.85, 0.85)
            };

            let btn = button(column![tile_image, count_text].align_items(iced::Alignment::Center))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: button_bg_color,
                    text_color: Color::BLACK,
                })))
                .on_press_maybe(if count > 0 {
                    Some(Message::AddTile(tile))
                } else {
                    None
                })
                .padding(5)
                .into();

            tiles.push(btn);
        }

        super::super::components::create_grid(tiles, 9)
    }
}
