use crate::gui::messages::Message;
use crate::gui::state::RiichiGui;
use crate::gui::styles::ColoredButtonStyle;
use crate::implements::game::AgariType;
use crate::implements::scoring::{AgariResult, HandLimit};
use crate::implements::yaku::Yaku;
use iced::widget::{button, column, container, text};
use iced::{Color, Element, theme};

pub fn build_result_view(gui: &RiichiGui) -> Element<'_, Message> {
    let content = match &gui.score_result {
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
                num_akadora,
            } = result;

            let valid_yaku_count = yaku_list
                .iter()
                .filter(|y| !matches!(y, Yaku::Dora | Yaku::UraDora | Yaku::AkaDora))
                .count();

            if valid_yaku_count == 0 && limit_name.is_none() {
                // Error message
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
                // Success message
                // Header
                let header = text("Calculation Result")
                    .size(30)
                    .style(Color::from_rgb(0.0, 0.0, 0.0));

                // Total Score
                let score_text = text(format!("{} Points", total_payment))
                    .size(40)
                    .style(Color::from_rgb(0.8, 0.2, 0.2));

                // Limit Name
                let limit_str = if let Some(limit) = limit_name {
                    Some(match limit {
                        HandLimit::Mangan => "Mangan",
                        HandLimit::Haneman => "Haneman",
                        HandLimit::Baiman => "Baiman",
                        HandLimit::Sanbaiman => "Sanbaiman",
                        HandLimit::Yakuman => "Yakuman",
                    })
                } else {
                    None
                };

                // Han/Fu Display
                let han_fu_text = if limit_name.as_ref() == Some(&HandLimit::Yakuman) {
                    text(format!("{} Han", han)).size(20)
                } else {
                    text(format!("{} Han / {} Fu", han, fu)).size(20)
                };

                // Yaku List Display
                let mut yaku_col =
                    column![text("Yaku:").size(18).style(Color::from_rgb(0.3, 0.3, 0.3))];
                let mut dora_count = 0;
                let mut uradora_count = 0;

                for yaku in yaku_list {
                    match yaku {
                        Yaku::Dora => dora_count += 1,
                        Yaku::UraDora => uradora_count += 1,
                        Yaku::AkaDora => {}
                        _ => {
                            yaku_col = yaku_col.push(text(format!("• {:?}", yaku)).size(18));
                        }
                    }
                }

                // Append bonus han counts
                if dora_count > 0 {
                    yaku_col = yaku_col.push(text(format!("• Dora x{}", dora_count)).size(18));
                }
                if uradora_count > 0 {
                    yaku_col =
                        yaku_col.push(text(format!("• Ura Dora x{}", uradora_count)).size(18));
                }
                if *num_akadora > 0 {
                    yaku_col = yaku_col.push(text(format!("• Aka Dora x{}", num_akadora)).size(18));
                }

                // Payment Detail Breakdown
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

                // Build column conditionally with or without limit text
                let mut result_column = column![header, score_text];

                if let Some(limit) = limit_str {
                    result_column = result_column
                        .push(text(limit).size(24).style(Color::from_rgb(0.8, 0.0, 0.0)));
                }

                result_column = result_column
                    .push(han_fu_text)
                    .push(yaku_col.spacing(5))
                    .push(payment_section)
                    .spacing(15)
                    .align_items(iced::Alignment::Center);

                result_column
            }
        }
        Some(Err(_)) => column![
            text("No Yaku Found")
                .size(30)
                .style(Color::from_rgb(0.8, 0.0, 0.0)),
            text("You need at least 1 Yaku to win.").size(20),
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
