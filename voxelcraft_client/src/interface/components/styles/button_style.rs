use iced::button;
use iced::{Background, Color, Vector};

pub enum Button {
    Primary,
    Secondary,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
            })),
            border_radius: 12.0,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 2.0),
            ..self.active()
        }
    }
}