use super::messages::Message;
use crate::implements::tiles::{Hai, Jihai, Kaze, Sangenpai, Suhai};
use iced::Element;
use iced::widget::{button, column, row};

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
        Hai::Suhai(n, Suhai::Manzu) => format!("Man{}.png", n),
        Hai::Suhai(n, Suhai::Pinzu) => format!("Pin{}.png", n),
        Hai::Suhai(n, Suhai::Souzu) => format!("Sou{}.png", n),
        Hai::Jihai(Jihai::Kaze(Kaze::Ton)) => "Ton.png".to_string(),
        Hai::Jihai(Jihai::Kaze(Kaze::Nan)) => "Nan.png".to_string(),
        Hai::Jihai(Jihai::Kaze(Kaze::Shaa)) => "Shaa.png".to_string(),
        Hai::Jihai(Jihai::Kaze(Kaze::Pei)) => "Pei.png".to_string(),
        Hai::Jihai(Jihai::Sangen(Sangenpai::Haku)) => "Haku.png".to_string(),
        Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu)) => "Hatsu.png".to_string(),
        Hai::Jihai(Jihai::Sangen(Sangenpai::Chun)) => "Chun.png".to_string(),
    };
    format!("lib/{}", filename)
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
        Hai::Suhai(n, Suhai::Manzu) => (0, *n),
        Hai::Suhai(n, Suhai::Pinzu) => (1, *n),
        Hai::Suhai(n, Suhai::Souzu) => (2, *n),
        Hai::Jihai(Jihai::Kaze(Kaze::Ton)) => (3, 0),
        Hai::Jihai(Jihai::Kaze(Kaze::Nan)) => (3, 1),
        Hai::Jihai(Jihai::Kaze(Kaze::Shaa)) => (3, 2),
        Hai::Jihai(Jihai::Kaze(Kaze::Pei)) => (3, 3),
        Hai::Jihai(Jihai::Sangen(Sangenpai::Haku)) => (4, 0),
        Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu)) => (4, 1),
        Hai::Jihai(Jihai::Sangen(Sangenpai::Chun)) => (4, 2),
    }
}
