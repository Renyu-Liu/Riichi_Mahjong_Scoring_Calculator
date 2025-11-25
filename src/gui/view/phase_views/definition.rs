use crate::gui::components::get_tile_image_path;
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
    let modify_btn = button(text("Modify Hand"))
        .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
            background_color: Color::from_rgb(0.0, 0.0, 0.6),
            text_color: Color::WHITE,
        })))
        .on_press(Message::ModifyHand);

    let winning_tile_section = column![
        text("Winning Tile:"),
        match &gui.winning_tile {
            Some(t) => {
                {
                    let e: Element<Message> = row![
                        button(
                            iced::widget::Image::<iced::widget::image::Handle>::new(
                                get_tile_image_path(t)
                            )
                            .width(40)
                        )
                        .on_press(Message::StartSelectWinningTile)
                        .style(theme::Button::Text)
                    ]
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .into();
                    e
                }
            }
            None => row![
                button(text("Select"))
                    .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                        background_color: Color::from_rgb(0.0, 0.0, 0.6),
                        text_color: Color::WHITE,
                    })))
                    .on_press(Message::StartSelectWinningTile)
            ]
            .align_items(iced::Alignment::Center)
            .into(),
        }
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center);

    let melds_section = column![
        text("Open Melds:"),
        column(
            gui.open_melds
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let tiles = gui.get_meld_tiles(m);
                    let tile_images = row(tiles
                        .iter()
                        .map(|t| {
                            iced::widget::Image::<iced::widget::image::Handle>::new(
                                get_tile_image_path(t),
                            )
                            .width(40)
                            .into()
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
        column(
            gui.closed_kans
                .iter()
                .enumerate()
                .map(|(i, k)| {
                    let tiles = vec![*k; 4];
                    let tile_images = row(tiles
                        .iter()
                        .map(|t| {
                            iced::widget::Image::<iced::widget::image::Handle>::new(
                                get_tile_image_path(t),
                            )
                            .width(40)
                            .into()
                        })
                        .collect::<Vec<Element<Message>>>())
                    .spacing(2);

                    row![
                        tile_images,
                        button(text("Change"))
                            .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                                background_color: Color::from_rgb(0.0, 0.0, 0.6),
                                text_color: Color::WHITE,
                            })))
                            .on_press(Message::EditClosedKan(i)),
                        button(text("Remove"))
                            .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                                background_color: Color::from_rgb(0.6, 0.0, 0.0),
                                text_color: Color::WHITE,
                            })))
                            .on_press(Message::RemoveClosedKan(i))
                    ]
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .into()
                })
                .collect::<Vec<Element<Message>>>()
        )
        .spacing(10),
        row![
            button(text("Add Pon"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.0, 0.0, 0.6),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::SelectMeldType(MentsuType::Koutsu)),
            button(text("Add Chii"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.0, 0.0, 0.6),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::SelectMeldType(MentsuType::Shuntsu)),
            button(text("Add Kan"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.0, 0.0, 0.6),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::StartAddClosedKan)
        ]
        .spacing(10)
        .align_items(iced::Alignment::Center)
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center);

    let is_oya = gui.jikaze == Kaze::Ton;
    let is_ron = gui.agari_type == AgariType::Ron;
    let is_tsumo = gui.agari_type == AgariType::Tsumo;
    let is_menzen = gui.open_melds.is_empty();

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
        text("Game Context:").size(18),
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
        // Special Yaku
        column![
            text("Special Yaku:"),
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
        ]
        .spacing(5),
        row![
            text(format!("Honba: {}", gui.honba)),
            button(text("+"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.0, 0.0, 0.6),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::IncrementHonba),
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
        {
            let five_tile_count = gui.count_five_tiles();

            if five_tile_count > 0 {
                row![
                    text(format!("Akadora: {}", gui.num_akadora)),
                    button(text("+"))
                        .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                            background_color: Color::from_rgb(0.0, 0.0, 0.6),
                            text_color: Color::WHITE,
                        })))
                        .on_press_maybe(
                            if gui.num_akadora < five_tile_count && gui.num_akadora < 4 {
                                Some(Message::IncrementAkadora)
                            } else {
                                None
                            }
                        ),
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
        column![
            text("Dora:"),
            row(gui
                .dora_indicators
                .iter()
                .enumerate()
                .map(|(i, t)| {
                    button(
                        iced::widget::Image::<iced::widget::image::Handle>::new(
                            get_tile_image_path(t),
                        )
                        .width(30),
                    )
                    .on_press(Message::RemoveDora(i))
                    .style(theme::Button::Text)
                    .into()
                })
                .collect::<Vec<Element<Message>>>())
            .spacing(5),
            button(text("Add"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.0, 0.0, 0.6),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::StartAddDora),
            if gui.is_riichi {
                column![
                    text("Ura Dora:"),
                    row(gui
                        .uradora_indicators
                        .iter()
                        .enumerate()
                        .map(|(i, t)| {
                            button(
                                iced::widget::Image::<iced::widget::image::Handle>::new(
                                    get_tile_image_path(t),
                                )
                                .width(30),
                            )
                            .on_press(Message::RemoveUraDora(i))
                            .style(theme::Button::Text)
                            .into()
                        })
                        .collect::<Vec<Element<Message>>>())
                    .spacing(5),
                    button(text("Add"))
                        .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                            background_color: Color::from_rgb(0.0, 0.0, 0.6),
                            text_color: Color::WHITE,
                        })))
                        .on_press(Message::StartAddUraDora),
                ]
                .spacing(5)
            } else {
                column![]
            }
        ]
        .spacing(5)
    ]
    .spacing(10);

    let calculate_btn = button(text("Calculate Score"))
        .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
            background_color: Color::from_rgb(0.0, 0.6, 0.0),
            text_color: Color::WHITE,
        })))
        .on_press_maybe(if gui.winning_tile.is_some() {
            Some(Message::CalculateScore)
        } else {
            None
        });

    column![
        hand_preview,
        modify_btn,
        winning_tile_section,
        melds_section,
        context_section,
        calculate_btn
    ]
    .spacing(20)
    .align_items(iced::Alignment::Center)
    .into()
}
