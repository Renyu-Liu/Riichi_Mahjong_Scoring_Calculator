use super::messages::Message;
use super::styles::ColoredButtonStyle;
use crate::implements::tiles::{Hai, Jihai, Kaze, Sangenpai, Suhai, Suit};
use iced::Element;
use iced::theme;
use iced::widget::{button, column, row, text};

#[allow(dead_code)]
pub trait OnPressMaybe {
    fn on_press_maybe(self, msg: Option<Message>) -> Self;
}

impl<'a> OnPressMaybe for button::Button<'a, Message> {
    fn on_press_maybe(self, msg: Option<Message>) -> Self {
        match msg {
            Some(m) => self.on_press(m),
            None => self,
        }
    }
}

pub fn get_tile_image_path(tile: &Hai) -> String {
    let filename = match tile {
        Hai::Suhai(Suhai { number, suit }) => {
            let suit_prefix = match suit {
                Suit::Manzu => "Man",
                Suit::Pinzu => "Pin",
                Suit::Souzu => "Sou",
            };
            format!("{}{}.png", suit_prefix, number)
        }
        Hai::Jihai(jihai) => {
            let name = match jihai {
                Jihai::Kaze(Kaze::Ton) => "Ton",
                Jihai::Kaze(Kaze::Nan) => "Nan",
                Jihai::Kaze(Kaze::Shaa) => "Shaa",
                Jihai::Kaze(Kaze::Pei) => "Pei",
                Jihai::Sangen(Sangenpai::Haku) => "Haku",
                Jihai::Sangen(Sangenpai::Hatsu) => "Hatsu",
                Jihai::Sangen(Sangenpai::Chun) => "Chun",
            };
            format!("{}.png", name)
        }
    };
    format!("assets/{}", filename)
}

pub fn create_grid(elements: Vec<Element<Message>>, columns: usize) -> Element<Message> {
    let mut rows = column![].spacing(10);
    let mut current_row = row![].spacing(10);
    let mut count_in_row = 0;

    for element in elements {
        current_row = current_row.push(element);
        count_in_row += 1;

        if count_in_row >= columns {
            rows = rows.push(current_row);
            current_row = row![].spacing(10);
            count_in_row = 0;
        }
    }

    if count_in_row > 0 {
        rows = rows.push(current_row);
    }

    rows.into()
}

pub fn sort_tiles_by_type(tile: &Hai) -> (u8, u8) {
    match tile {
        Hai::Suhai(Suhai {
            number: n,
            suit: Suit::Manzu,
        }) => (0, *n),
        Hai::Suhai(Suhai {
            number: n,
            suit: Suit::Pinzu,
        }) => (1, *n),
        Hai::Suhai(Suhai {
            number: n,
            suit: Suit::Souzu,
        }) => (2, *n),
        Hai::Jihai(Jihai::Kaze(Kaze::Ton)) => (3, 0),
        Hai::Jihai(Jihai::Kaze(Kaze::Nan)) => (3, 1),
        Hai::Jihai(Jihai::Kaze(Kaze::Shaa)) => (3, 2),
        Hai::Jihai(Jihai::Kaze(Kaze::Pei)) => (3, 3),
        Hai::Jihai(Jihai::Sangen(Sangenpai::Haku)) => (4, 0),
        Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu)) => (4, 1),
        Hai::Jihai(Jihai::Sangen(Sangenpai::Chun)) => (4, 2),
    }
}

pub fn tile_button<'a>(
    content: Element<'a, Message>,
    msg: Message,
    style: ColoredButtonStyle,
) -> Element<'a, Message> {
    button(content)
        .style(theme::Button::Custom(Box::new(style)))
        .on_press(msg)
        .padding(5)
        .into()
}

pub fn action_button<'a>(
    label: &str,
    msg: Message,
    style: ColoredButtonStyle,
) -> Element<'a, Message> {
    button(text(label))
        .style(theme::Button::Custom(Box::new(style)))
        .on_press(msg)
        .into()
}

pub fn cancel_button<'a>() -> Element<'a, Message> {
    action_button(
        "Cancel",
        Message::CancelSelection,
        ColoredButtonStyle::DANGER,
    )
}
