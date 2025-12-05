use crate::gui::components::action_button;
use crate::gui::messages::Message;
use crate::gui::state::RiichiGui;
use crate::gui::styles::ColoredButtonStyle;
use crate::implements::game::AgariType;
use crate::implements::hand::MentsuType;
use crate::implements::tiles::Kaze;
use iced::widget::{button, checkbox, column, radio, row, text};
use iced::{Color, Element, theme};

pub fn build_definition_view(gui: &RiichiGui) -> Element<'_, Message> {
    let hand_preview = gui.view_hand_preview_locked();
    let modify_btn = action_button("Modify Hand", Message::ModifyHand, ColoredButtonStyle::INFO);

    // Winning Tile Section
    let winning_tile_section = column![
        text("Winning Tile").size(20).font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..iced::Font::with_name("Arimo")
        }),
        match &gui.winning_tile {
            Some(t) => {
                {
                    let e: Element<Message> = row![{
                        let handle = gui
                            .tile_images
                            .get(t)
                            .expect("Tile image not found")
                            .clone();
                        button(iced::widget::Image::new(handle).width(40))
                            .on_press(Message::StartSelectWinningTile)
                            .style(theme::Button::Text)
                    }]
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .into();
                    e
                }
            }
            None => row![action_button(
                "Select",
                Message::StartSelectWinningTile,
                ColoredButtonStyle::INFO,
            )]
            .align_items(iced::Alignment::Center)
            .into(),
        }
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center);

    // Melds Section
    let melds_section = column![
        text("Open Melds").size(20).font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..iced::Font::with_name("Arimo")
        }),
        // Display existing open melds
        column(
            gui.open_melds
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let tiles = gui.get_meld_tiles(m);
                    let tile_images = row(tiles
                        .iter()
                        .map(|t| {
                            let handle = gui
                                .tile_images
                                .get(t)
                                .expect("Tile image not found")
                                .clone();
                            iced::widget::Image::new(handle).width(40).into()
                        })
                        .collect::<Vec<Element<Message>>>())
                    .spacing(2);

                    row![
                        button(tile_images)
                            .on_press(Message::RemoveOpenMeld(i))
                            .style(theme::Button::Text)
                    ]
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .into()
                })
                .collect::<Vec<Element<Message>>>()
        )
        .align_items(iced::Alignment::Center)
        .spacing(10),
        // Display existing closed kans
        column(
            gui.closed_kans
                .iter()
                .enumerate()
                .map(|(i, k)| {
                    let tiles = vec![*k; 4];
                    let tile_images = row(tiles
                        .iter()
                        .map(|t| {
                            let handle = gui
                                .tile_images
                                .get(t)
                                .expect("Tile image not found")
                                .clone();
                            iced::widget::Image::new(handle).width(40).into()
                        })
                        .collect::<Vec<Element<Message>>>())
                    .spacing(2);
                    row![
                        button(tile_images)
                            .on_press(Message::RemoveClosedKan(i))
                            .style(theme::Button::Text)
                    ]
                    .align_items(iced::Alignment::Center)
                    .into()
                })
                .collect::<Vec<Element<Message>>>()
        )
        .spacing(10),
        // Buttons to add new melds
        row![
            action_button(
                "Add Pon",
                Message::SelectMeldType(MentsuType::Koutsu),
                ColoredButtonStyle::INFO,
            ),
            action_button(
                "Add Chii",
                Message::SelectMeldType(MentsuType::Shuntsu),
                ColoredButtonStyle::INFO,
            ),
            action_button(
                "Add Kan",
                Message::StartAddClosedKan,
                ColoredButtonStyle::INFO,
            )
        ]
        .spacing(10)
        .align_items(iced::Alignment::Center)
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center);

    // Game Context Section
    let is_oya = gui.jikaze == Kaze::Ton;
    let is_ron = gui.agari_type == AgariType::Ron;
    let is_tsumo = gui.agari_type == AgariType::Tsumo;
    let is_menzen = gui.open_melds.is_empty();

    // Incompatible yaku checkbox
    let checkbox_with_conflict = |label: &str,
                                  is_checked: bool,
                                  msg: fn(bool) -> Message,
                                  is_enabled: bool|
     -> Element<Message> {
        if is_enabled {
            checkbox(label, is_checked).on_toggle(msg).into()
        } else {
            row![
                checkbox("", is_checked),
                text(label).style(Color::from_rgb(0.5, 0.5, 0.5))
            ]
            .spacing(0)
            .align_items(iced::Alignment::Center)
            .into()
        }
    };

    let context_section = column![
        text("Game Info").size(20).font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..iced::Font::with_name("Arimo")
        }),
        // Agari Type
        row![
            text("Win Type:"),
            radio(
                "Ron",
                AgariType::Ron,
                Some(gui.agari_type),
                Message::ToggleAgariType
            ),
            radio(
                "Tsumo",
                AgariType::Tsumo,
                Some(gui.agari_type),
                Message::ToggleAgariType
            ),
        ]
        .spacing(20),
        // Bakaze
        row![
            text("Prevalent Wind:"),
            radio("East", Kaze::Ton, Some(gui.bakaze), Message::SetBakaze),
            radio("South", Kaze::Nan, Some(gui.bakaze), Message::SetBakaze),
            radio("West", Kaze::Shaa, Some(gui.bakaze), Message::SetBakaze),
            radio("North", Kaze::Pei, Some(gui.bakaze), Message::SetBakaze),
        ]
        .spacing(10),
        // Jikaze
        row![
            text("Seat Wind:"),
            radio("East", Kaze::Ton, Some(gui.jikaze), Message::SetJikaze),
            radio("South", Kaze::Nan, Some(gui.jikaze), Message::SetJikaze),
            radio("West", Kaze::Shaa, Some(gui.jikaze), Message::SetJikaze),
            radio("North", Kaze::Pei, Some(gui.jikaze), Message::SetJikaze),
        ]
        .spacing(10),
        // Honba Counter
        row![
            text(format!("Honba: {}", gui.honba)),
            action_button("+", Message::IncrementHonba, ColoredButtonStyle::INFO,),
            button(text("-"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.6, 0.0, 0.0),
                    text_color: Color::WHITE,
                })))
                .on_press_maybe(if gui.honba > 0 {
                    Some(Message::DecrementHonba)
                } else {
                    None
                }),
        ]
        .spacing(10)
        .align_items(iced::Alignment::Center),
        iced::widget::rule::Rule::horizontal(30),
        // Special Yaku
        column![
            text("Special Yaku").size(20).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..iced::Font::with_name("Arimo")
            }),
            // Riichi & Status
            row![
                checkbox_with_conflict("Riichi", gui.is_riichi, Message::ToggleRiichi, is_menzen),
                checkbox_with_conflict(
                    "Double Riichi",
                    gui.is_daburu_riichi,
                    Message::ToggleDoubleRiichi,
                    is_menzen
                ),
                checkbox_with_conflict(
                    "Ippatsu",
                    gui.is_ippatsu,
                    Message::ToggleIppatsu,
                    gui.is_riichi || gui.is_daburu_riichi
                ),
            ]
            .spacing(10),
            row![
                checkbox_with_conflict(
                    "Tenhou",
                    gui.is_tenhou,
                    Message::ToggleTenhou,
                    is_tsumo && is_oya && is_menzen
                ),
                checkbox_with_conflict(
                    "Chiihou",
                    gui.is_chiihou,
                    Message::ToggleChiihou,
                    is_tsumo && !is_oya && is_menzen
                ),
                checkbox_with_conflict(
                    "Renhou",
                    gui.is_renhou,
                    Message::ToggleRenhou,
                    is_ron && is_menzen
                ),
            ]
            .spacing(10),
            row![
                checkbox_with_conflict("Haitei", gui.is_haitei, Message::ToggleHaitei, is_tsumo),
                checkbox_with_conflict("Houtei", gui.is_houtei, Message::ToggleHoutei, is_ron),
                checkbox_with_conflict("Rinshan", gui.is_rinshan, Message::ToggleRinshan, is_tsumo),
                checkbox_with_conflict("Chankan", gui.is_chankan, Message::ToggleChankan, is_ron),
            ]
            .spacing(10),
            iced::widget::rule::Rule::horizontal(30),
        ]
        .spacing(5)
        .align_items(iced::Alignment::Center),
        text("Dora").size(20).font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..iced::Font::with_name("Arimo")
        }),
        // Dora Indicators
        column![
            text("Dora:"),
            row(gui
                .dora_indicators
                .iter()
                .enumerate()
                .map(|(i, t)| {
                    let handle = gui
                        .tile_images
                        .get(t)
                        .expect("Tile image not found")
                        .clone();
                    button(iced::widget::Image::new(handle).width(30))
                        .on_press(Message::RemoveDora(i))
                        .style(theme::Button::Text)
                        .into()
                })
                .collect::<Vec<Element<Message>>>())
            .spacing(5),
            action_button("Add", Message::StartAddDora, ColoredButtonStyle::INFO,),
            if gui.is_riichi {
                column![
                    text("Ura Dora:"),
                    row(gui
                        .uradora_indicators
                        .iter()
                        .enumerate()
                        .map(|(i, t)| {
                            let handle = gui
                                .tile_images
                                .get(t)
                                .expect("Tile image not found")
                                .clone();
                            button(iced::widget::Image::new(handle).width(30))
                                .on_press(Message::RemoveUraDora(i))
                                .style(theme::Button::Text)
                                .into()
                        })
                        .collect::<Vec<Element<Message>>>())
                    .spacing(5),
                    action_button("Add", Message::StartAddUraDora, ColoredButtonStyle::INFO,),
                ]
                .spacing(5)
                .align_items(iced::Alignment::Center)
            } else {
                column![]
            }
        ]
        .spacing(5)
        .align_items(iced::Alignment::Center),
        // Akadora Counter
        {
            let max_akadora = gui.get_max_akadora_count();

            if max_akadora > 0 {
                row![
                    text(format!("Akadora: {}", gui.num_akadora)),
                    button(text("+"))
                        .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                            background_color: Color::from_rgb(0.0, 0.0, 0.6),
                            text_color: Color::WHITE,
                        })))
                        .on_press_maybe(if gui.num_akadora < max_akadora && gui.num_akadora < 4 {
                            Some(Message::IncrementAkadora)
                        } else {
                            None
                        }),
                    button(text("-"))
                        .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                            background_color: Color::from_rgb(0.6, 0.0, 0.0),
                            text_color: Color::WHITE,
                        })))
                        .on_press_maybe(if gui.num_akadora > 0 {
                            Some(Message::DecrementAkadora)
                        } else {
                            None
                        }),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center)
            } else {
                row![]
            }
        },
    ]
    .spacing(15)
    .align_items(iced::Alignment::Center);

    let calculate_btn = button(text("Calculate Score"))
        .style(theme::Button::Custom(Box::new(ColoredButtonStyle::PRIMARY)))
        .on_press_maybe(if gui.winning_tile.is_some() {
            Some(Message::CalculateScore)
        } else {
            None
        });

    let mut content = column![
        hand_preview,
        modify_btn,
        iced::widget::rule::Rule::horizontal(30),
        winning_tile_section,
        iced::widget::rule::Rule::horizontal(30),
        melds_section,
        iced::widget::rule::Rule::horizontal(30),
        context_section,
        calculate_btn
    ];

    if gui.winning_tile.is_none() {
        content = content.push(
            text("You must select a winning tile")
                .size(12)
                .style(Color::from_rgb(0.8, 0.0, 0.0)),
        );
    }

    content
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
}
