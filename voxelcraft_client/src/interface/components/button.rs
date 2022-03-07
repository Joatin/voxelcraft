use iced::{Button, Text, Length};
use iced::alignment::Horizontal;

pub fn button<'a, Message: Clone>(
    state: &'a mut iced::button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(Horizontal::Center).width(Length::Fill),
    )
        .padding(16)
        .min_width(100)
}