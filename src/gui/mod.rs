pub mod components;
pub mod messages;
pub mod state;
pub mod styles;
pub mod update;
pub mod view;

use self::messages::Message;
use self::state::RiichiGui;
use self::update::Update;
use self::view::View;
use iced::{Element, Sandbox, Settings};

pub fn run() -> iced::Result {
    let mut settings = Settings::default();
    settings.fonts = vec![
        include_bytes!("../../assets/font/Arimo.ttf")
            .as_slice()
            .into(),
        include_bytes!("../../assets/font/Arimo-Bold.ttf")
            .as_slice()
            .into(),
    ];
    settings.default_font = iced::Font::with_name("Arimo");
    RiichiGui::run(settings)
}

impl Sandbox for RiichiGui {
    type Message = Message;

    fn new() -> Self {
        Self::new()
    }

    fn title(&self) -> String {
        String::from("Riichi Mahjong Calculator")
    }

    fn update(&mut self, message: Message) {
        Update::update(self, message)
    }

    fn view(&self) -> Element<'_, Message> {
        View::view(self)
    }
}
