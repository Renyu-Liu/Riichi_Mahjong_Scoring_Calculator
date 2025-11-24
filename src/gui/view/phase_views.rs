use super::super::components::get_tile_image_path;
use super::super::messages::Message;
use super::super::state::{Phase, RiichiGui};
use super::super::styles::ColoredButtonStyle;
use super::View;
use crate::implements::game::AgariType;
use crate::implements::hand::MentsuType;
use crate::implements::tiles::Kaze;
use iced::widget::{button, checkbox, column, container, radio, row, scrollable, text};
use iced::{Color, Element, Length, theme};

impl View for RiichiGui {
    fn view(&self) -> Element<'_, Message> {
        let content = match &self.phase {
            Phase::Composition => self.view_composition(),
            Phase::Definition => self.view_definition(),
            Phase::SelectingWinningTile => self.view_selecting_winning_tile(),
            Phase::SelectingMeldTile(m_type, _) => self.view_selecting_meld_tile(*m_type),
            Phase::SelectingClosedKan { .. } => self.view_selecting_closed_kan(),
            Phase::SelectingDora => self.view_selecting_dora(false),
            Phase::SelectingUraDora => self.view_selecting_dora(true),
            Phase::Result => self.view_result(),
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
        let hand_preview = self.view_hand_preview();
        let tile_pool = self.view_tile_pool();

        let tile_count = self.hand_tiles.len();
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

    fn view_definition(&self) -> Element<'_, Message> {
        let hand_preview = self.view_hand_preview_locked();
        let modify_btn = button(text("Modify Hand"))
            .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                background_color: Color::from_rgb(0.0, 0.0, 0.6),
                text_color: Color::WHITE,
            })))
            .on_press(Message::ModifyHand);

        let winning_tile_section = column![
            text("Winning Tile:"),
            match &self.winning_tile {
                Some(t) => {
                    {
                        let e: Element<Message> = row![
                            iced::widget::Image::<iced::widget::image::Handle>::new(
                                get_tile_image_path(t, false)
                            )
                            .width(40),
                            button(text("Change"))
                                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                                    background_color: Color::from_rgb(0.0, 0.0, 0.6),
                                    text_color: Color::WHITE,
                                })))
                                .on_press(Message::StartSelectWinningTile)
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
                self.open_melds
                    .iter()
                    .enumerate()
                    .map(|(i, m)| {
                        let tiles = self.get_meld_tiles(m);
                        let tile_images = row(tiles
                            .iter()
                            .map(|t| {
                                iced::widget::Image::<iced::widget::image::Handle>::new(
                                    get_tile_image_path(t, false),
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
                                .on_press(Message::EditOpenMeld(i)),
                            button(text("Remove"))
                                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                                    background_color: Color::from_rgb(0.6, 0.0, 0.0),
                                    text_color: Color::WHITE,
                                })))
                                .on_press(Message::RemoveOpenMeld(i))
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
                self.closed_kans
                    .iter()
                    .enumerate()
                    .map(|(i, k)| {
                        let tiles = vec![*k; 4];
                        let tile_images = row(tiles
                            .iter()
                            .map(|t| {
                                iced::widget::Image::<iced::widget::image::Handle>::new(
                                    get_tile_image_path(t, false),
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

        let is_oya = self.jikaze == Kaze::Ton;
        let is_ron = self.agari_type == AgariType::Ron;
        let is_tsumo = self.agari_type == AgariType::Tsumo;
        let is_menzen = self.open_melds.is_empty();

        // conflicts
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
                    Some(self.agari_type),
                    Message::ToggleAgariType
                ),
                radio(
                    "Tsumo",
                    AgariType::Tsumo,
                    Some(self.agari_type),
                    Message::ToggleAgariType
                ),
            ]
            .spacing(20),
            // Bakaze
            row![
                text("Prevalent Wind (Bakaze):"),
                radio("East", Kaze::Ton, Some(self.bakaze), Message::SetBakaze),
                radio("South", Kaze::Nan, Some(self.bakaze), Message::SetBakaze),
                radio("West", Kaze::Shaa, Some(self.bakaze), Message::SetBakaze),
                radio("North", Kaze::Pei, Some(self.bakaze), Message::SetBakaze),
            ]
            .spacing(10),
            // Jikaze
            row![
                text("Player Wind (Jikaze):"),
                radio("East", Kaze::Ton, Some(self.jikaze), Message::SetJikaze),
                radio("South", Kaze::Nan, Some(self.jikaze), Message::SetJikaze),
                radio("West", Kaze::Shaa, Some(self.jikaze), Message::SetJikaze),
                radio("North", Kaze::Pei, Some(self.jikaze), Message::SetJikaze),
            ]
            .spacing(10),
            // Riichi & Status
            row![
                checkbox_with_conflict("Riichi", self.is_riichi, Message::ToggleRiichi, is_menzen),
                checkbox_with_conflict(
                    "Double Riichi",
                    self.is_daburu_riichi,
                    Message::ToggleDoubleRiichi,
                    is_menzen
                ),
                checkbox_with_conflict(
                    "Ippatsu",
                    self.is_ippatsu,
                    Message::ToggleIppatsu,
                    self.is_riichi || self.is_daburu_riichi
                ),
            ]
            .spacing(10),
            // Special Yaku
            column![
                text("Special Yaku:"),
                row![
                    checkbox_with_conflict(
                        "Tenhou",
                        self.is_tenhou,
                        Message::ToggleTenhou,
                        is_tsumo && is_oya && is_menzen
                    ),
                    checkbox_with_conflict(
                        "Chiihou",
                        self.is_chiihou,
                        Message::ToggleChiihou,
                        is_tsumo && !is_oya && is_menzen
                    ),
                    checkbox_with_conflict(
                        "Renhou",
                        self.is_renhou,
                        Message::ToggleRenhou,
                        is_ron && is_menzen
                    ),
                ]
                .spacing(10),
                row![
                    checkbox_with_conflict(
                        "Haitei",
                        self.is_haitei,
                        Message::ToggleHaitei,
                        is_tsumo
                    ),
                    checkbox_with_conflict("Houtei", self.is_houtei, Message::ToggleHoutei, is_ron),
                    checkbox_with_conflict(
                        "Rinshan",
                        self.is_rinshan,
                        Message::ToggleRinshan,
                        is_tsumo
                    ),
                    checkbox_with_conflict(
                        "Chankan",
                        self.is_chankan,
                        Message::ToggleChankan,
                        is_ron
                    ),
                ]
                .spacing(10),
            ]
            .spacing(5),
            row![
                text(format!("Honba: {}", self.honba)),
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
                    .on_press_maybe(if self.honba > 0 {
                        Some(Message::DecrementHonba)
                    } else {
                        None
                    }),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            {
                let five_tile_count = self.count_five_tiles();

                if five_tile_count > 0 {
                    row![
                        text(format!("Akadora: {}", self.num_akadora)),
                        button(text("+"))
                            .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                                background_color: Color::from_rgb(0.0, 0.0, 0.6),
                                text_color: Color::WHITE,
                            })))
                            .on_press_maybe(
                                if self.num_akadora < five_tile_count && self.num_akadora < 4 {
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
                            .on_press_maybe(if self.num_akadora > 0 {
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
                row(self
                    .dora_indicators
                    .iter()
                    .map(|t| iced::widget::Image::<iced::widget::image::Handle>::new(
                        get_tile_image_path(t, false)
                    )
                    .width(30)
                    .into())
                    .collect::<Vec<Element<Message>>>())
                .spacing(5),
                button(text("Add"))
                    .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                        background_color: Color::from_rgb(0.0, 0.0, 0.6),
                        text_color: Color::WHITE,
                    })))
                    .on_press(Message::StartAddDora),
                if self.is_riichi {
                    column![
                        text("Ura Dora:"),
                        row(self
                            .uradora_indicators
                            .iter()
                            .map(|t| iced::widget::Image::<iced::widget::image::Handle>::new(
                                get_tile_image_path(t, false)
                            )
                            .width(30)
                            .into())
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
            .on_press_maybe(if self.winning_tile.is_some() {
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

    fn view_result(&self) -> Element<'_, Message> {
        use crate::implements::scoring::{AgariResult, HandLimit};
        use crate::implements::yaku::Yaku;

        let content = match &self.score_result {
            Some(Ok(result)) => {
                let AgariResult {
                    han,
                    fu,
                    yaku_list,
                    limit_name,
                    total_payment,
                    oya_payment,
                    ko_payment,
                    honba,
                    agari_type,
                    is_oya,
                    ..
                } = result;

                // Check valid yaku
                let valid_yaku_count = yaku_list
                    .iter()
                    .filter(|y| !matches!(y, Yaku::Dora | Yaku::UraDora | Yaku::AkaDora))
                    .count();

                if valid_yaku_count == 0 && limit_name.is_none() {
                    column![
                        text("No Yaku Found")
                            .size(30)
                            .style(Color::from_rgb(0.8, 0.0, 0.0)),
                        text("You need at least 1 Yaku to win.").size(20),
                        text("(Dora does not count as Yaku)")
                            .size(16)
                            .style(Color::from_rgb(0.5, 0.5, 0.5))
                    ]
                    .spacing(15)
                    .align_items(iced::Alignment::Center)
                } else {
                    // Header
                    let header = text("Calculation Result")
                        .size(30)
                        .style(Color::from_rgb(0.0, 0.0, 0.0));

                    // Total Score
                    let score_text = text(format!("{} Points", total_payment))
                        .size(40)
                        .style(Color::from_rgb(0.8, 0.2, 0.2)); // Accent color

                    // Limit Name (if any)
                    let limit_text = if let Some(limit) = limit_name {
                        let limit_str = match limit {
                            HandLimit::Mangan => "Mangan",
                            HandLimit::Haneman => "Haneman",
                            HandLimit::Baiman => "Baiman",
                            HandLimit::Sanbaiman => "Sanbaiman",
                            HandLimit::Yakuman => "Yakuman",
                        };
                        text(limit_str)
                            .size(24)
                            .style(Color::from_rgb(0.8, 0.0, 0.0))
                    } else {
                        text("")
                    };

                    // Han / Fu
                    let han_fu_text = if limit_name.as_ref() == Some(&HandLimit::Yakuman) {
                        text(format!("{} Han", han)).size(20)
                    } else {
                        text(format!("{} Han / {} Fu", han, fu)).size(20)
                    };

                    // Yaku List
                    let mut yaku_col =
                        column![text("Yaku:").size(18).style(Color::from_rgb(0.3, 0.3, 0.3))];
                    let mut dora_count = 0;
                    let mut uradora_count = 0;
                    let mut akadora_count = 0;

                    for yaku in yaku_list {
                        match yaku {
                            Yaku::Dora => dora_count += 1,
                            Yaku::UraDora => uradora_count += 1,
                            Yaku::AkaDora => akadora_count += 1,
                            _ => {
                                yaku_col = yaku_col.push(text(format!("• {:?}", yaku)).size(18));
                            }
                        }
                    }

                    if dora_count > 0 {
                        yaku_col = yaku_col.push(text(format!("• Dora x{}", dora_count)).size(18));
                    }
                    if uradora_count > 0 {
                        yaku_col =
                            yaku_col.push(text(format!("• Ura Dora x{}", uradora_count)).size(18));
                    }
                    if akadora_count > 0 {
                        yaku_col =
                            yaku_col.push(text(format!("• Aka Dora x{}", akadora_count)).size(18));
                    }

                    // Payment Detail
                    let tsumo_bonus = *honba as u32 * 100;
                    let ron_bonus = *honba as u32 * 300;

                    let payment_text = match (*is_oya, agari_type) {
                        (true, AgariType::Tsumo) => {
                            format!(
                                "Dealer Tsumo\nEach Non-Dealer pays: {} (+{} honba)",
                                oya_payment, tsumo_bonus
                            )
                        }
                        (false, AgariType::Tsumo) => {
                            format!(
                                "Non-Dealer Tsumo\nDealer pays: {} (+{} honba)\nOther Non-Dealers pay: {} (+{} honba)",
                                oya_payment, tsumo_bonus, ko_payment, tsumo_bonus
                            )
                        }
                        (true, AgariType::Ron) => {
                            format!(
                                "Dealer Ron\nDiscarder pays: {} (+{} honba)",
                                total_payment - ron_bonus,
                                ron_bonus
                            )
                        }
                        (false, AgariType::Ron) => {
                            format!(
                                "Non-Dealer Ron\nDiscarder pays: {} (+{} honba)",
                                total_payment - ron_bonus,
                                ron_bonus
                            )
                        }
                    };

                    let payment_section = container(text(payment_text).size(16)).padding(10);

                    column![
                        header,
                        score_text,
                        limit_text,
                        han_fu_text,
                        yaku_col.spacing(5),
                        payment_section
                    ]
                    .spacing(15)
                    .align_items(iced::Alignment::Center)
                }
            }
            Some(Err(_)) => column![
                text("No Yaku Found")
                    .size(30)
                    .style(Color::from_rgb(0.8, 0.0, 0.0)),
                text("You need at least 1 Yaku to win.").size(20),
                text("(Dora does not count as Yaku)")
                    .size(16)
                    .style(Color::from_rgb(0.5, 0.5, 0.5))
            ]
            .spacing(15)
            .align_items(iced::Alignment::Center),
            None => column![text("No result available.")],
        };

        column![
            content,
            button(text("Start Over"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.0, 0.6, 0.0),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::StartOver)
        ]
        .spacing(30)
        .align_items(iced::Alignment::Center)
        .into()
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
