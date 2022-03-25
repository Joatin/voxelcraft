use crate::interface::components;
use crate::interface::components::styles;
use crate::interface::message::Message;
use crate::interface::page::Page;
use iced::alignment::Horizontal;
use iced::{button, Alignment, Column, Element, Length, Space, Text};

pub const WORLD_SELECTION_PAGE_ROUTE: &str = "WORLD_SELECTION";

pub struct WorldSelectionPage {
    new_world_button: button::State,
}

impl WorldSelectionPage {
    pub fn new() -> Self {
        let new_world_button = button::State::new();
        Self { new_world_button }
    }
}

impl Page for WorldSelectionPage {
    fn name(&self) -> &str {
        WORLD_SELECTION_PAGE_ROUTE
    }

    fn view(&mut self) -> Element<'_, Message> {
        Column::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(Space::new(Length::Shrink, Length::Units(100)))
            .push(
                Column::new()
                    .width(Length::Units(350))
                    .align_items(Alignment::Center)
                    .push(
                        Text::new("Choose world")
                            .horizontal_alignment(Horizontal::Center)
                            .size(40),
                    )
                    .push(
                        components::button(&mut self.new_world_button, "CREATE NEW WORLD")
                            .width(Length::Fill)
                            .style(styles::Button::Primary)
                            .on_press(Message::CreateNewGame),
                    ),
            )
            .into()
    }
}
