use iced::Color;
use iced::widget::button;

pub struct ColoredButtonStyle {
    pub background_color: Color,
    pub text_color: Color,
}

impl button::StyleSheet for ColoredButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(self.background_color)),
            text_color: self.text_color,
            border: iced::Border::with_radius(7.0),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(iced::Background::Color(Color {
                a: 0.8,
                ..self.background_color
            })),
            ..active
        }
    }
}

impl ColoredButtonStyle {
    pub const PRIMARY: Self = Self {
        background_color: Color::from_rgb(0.0, 0.6, 0.0),
        text_color: Color::WHITE,
    };

    pub const INFO: Self = Self {
        background_color: Color::from_rgb(0.0, 0.0, 0.6),
        text_color: Color::WHITE,
    };

    pub const SECONDARY: Self = Self {
        background_color: Color::from_rgb(0.6, 0.6, 0.6),
        text_color: Color::WHITE,
    };

    pub const DANGER: Self = Self {
        background_color: Color::from_rgb(0.6, 0.0, 0.0),
        text_color: Color::WHITE,
    };

    pub const NEUTRAL: Self = Self {
        background_color: Color::WHITE,
        text_color: Color::BLACK,
    };

    pub const NEUTRAL_HOVER: Self = Self {
        background_color: Color::from_rgb(0.95, 0.95, 0.95),
        text_color: Color::BLACK,
    };
}

pub struct OverlayStyle;

impl iced::widget::container::StyleSheet for OverlayStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(Color {
                a: 0.95,
                ..Color::BLACK
            })),
            ..Default::default()
        }
    }
}
